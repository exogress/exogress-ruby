use lazy_static::lazy_static;

use rutie::{Class, Object, RString, VM, Hash, Symbol, AnyException, Boolean, NilClass};
use exogress_common::entities::{AccessKeyId, ProjectName, AccountName, LabelName, LabelValue};
use rutie::Exception;
use std::str::FromStr;
use hashbrown::HashMap;
use rutie::types::ValueType;
use futures::channel::mpsc::{self, UnboundedSender, UnboundedReceiver};
use futures::channel::oneshot;
use exogress_common::entities::SmolStr;
use exogress_common::client_core::Client;
use tokio::runtime::Runtime;
use trust_dns_resolver::TokioAsyncResolver;
use trust_dns_resolver::TokioHandle;
use rutie::Thread;
use std::sync::Arc;
use rutie::AnyObject;

const CRATE_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct RustInstance {
    client: parking_lot::Mutex<Option<exogress_common::client_core::Client>>,
    reload_config_tx: parking_lot::Mutex<UnboundedSender<()>>,
    reload_config_rx: Arc<parking_lot::Mutex<Option<UnboundedReceiver<()>>>>,
    stop_tx: parking_lot::Mutex<Option<oneshot::Sender<()>>>,
    stop_rx: Arc<parking_lot::Mutex<Option<oneshot::Receiver<()>>>>,
}

wrappable_struct!(RustInstance, RustInstanceWrapper, INSTANCE_WRAPPER);

fn get_string_entity<T>(k: &str, args: &Hash) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Display,
{
    args
        .at(&Symbol::new(k))
        .try_convert_to::<RString>()
        .map_err(|e| VM::raise_ex(e))
        .unwrap()
        .to_string()
        .parse::<T>()
        .map_err(|e| {
            let exception = AnyException::new("EntityError", Some(e.to_string().as_str()));
            VM::raise_ex(exception);
        })
        .unwrap()
}

fn get_string_entity_optional<T>(k: &str, args: &Hash) -> Option<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::fmt::Display,
{
    let val = args
        .at(&Symbol::new(k));
    if val.is_nil() {
        return None;
    } else {
        Some(val
            .try_convert_to::<RString>()
            .map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string()
            .parse::<T>()
            .map_err(|e| {
                let exception = AnyException::new("EntityError", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap())
    }
}

fn get_bool_entity(k: &str, default: bool, args: &Hash) -> bool
{
    let val = args
        .at(&Symbol::new(k));
    if val.is_nil() {
        return default;
    } else {
        val
            .try_convert_to::<Boolean>()
            .map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_bool()
    }
}

class!(Instance);

methods!(
    Instance,
    itself,

    fn new(args: Hash) -> Class {
        let args = args.map_err(|e| VM::raise_ex(e)).unwrap();
        let account: AccountName = get_string_entity("account", &args);
        let project: ProjectName = get_string_entity("project", &args);
        let access_key_id: AccessKeyId = get_string_entity("access_key_id", &args);
        let config_path: Option<String> = get_string_entity_optional("config_path", &args);
        let watch_config = get_bool_entity("watch_config", true, &args);

        let mut labels = hashbrown::HashMap::new();
        let labels_ruby = args.at(&Symbol::new("labels"));
        if labels_ruby.ty() == ValueType::Hash {
            let labels_ruby = labels_ruby
                .try_convert_to::<Hash>()
                .map_err(|e| VM::raise_ex(e))
                .unwrap();

            labels_ruby.each(|k,v| {
                let as_symbol = k
                        .try_convert_to::<Symbol>()
                        .map(|symbol| {
                            symbol
                                .to_string()
                        });

                let k = as_symbol.unwrap_or_else(|_| {
                    k
                        .try_convert_to::<RString>()
                        .map_err(|e| VM::raise_ex(e))
                        .unwrap()
                        .to_string()
                });

                labels.insert(
                    k.parse::<LabelName>()
                        .map_err(|e| {
                            let exception = AnyException::new("EntityError", Some(e.to_string().as_str()));
                            VM::raise_ex(exception);
                        })
                        .unwrap(),
                    v
                        .try_convert_to::<RString>()
                        .map_err(|e| VM::raise_ex(e))
                        .unwrap()
                        .to_string()
                        .parse::<LabelValue>()
                        .map_err(|e| {
                            let exception = AnyException::new("EntityError", Some("bad label value"));
                            VM::raise_ex(exception);
                        })
                        .unwrap()
                );
            });
        }
        let secret_access_key = args
            .at(&Symbol::new("secret_access_key"))
            .try_convert_to::<RString>()
            .map_err(|e| VM::raise_ex(e))
            .unwrap()
            .to_string();

        let mut client_builder = Client::builder();

        if let Some(config_path) = config_path {
            client_builder.config_path(config_path);
        }

        let client = client_builder
            .access_key_id(access_key_id.clone())
            .secret_access_key(secret_access_key.clone())
            .account(account.clone())
            .project(project.clone())
            .watch_config(watch_config)
            .profile(None)
            .labels(labels)
            .additional_connection_params({
                let mut map = HashMap::<SmolStr, SmolStr>::new();
                map.insert("client".into(), "ruby".into());
                map.insert("wrapper_version".into(), CRATE_VERSION.into());
                map
            })
            .build()
            .map_err(|e| {
                let exception = AnyException::new("ExogressError", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();

        let (reload_config_tx, reload_config_rx) = mpsc::unbounded();
        let (stop_tx, stop_rx) = oneshot::channel();

        let instance = RustInstance {
            client: parking_lot::Mutex::new(Some(client)),
            reload_config_tx: parking_lot::Mutex::new(reload_config_tx),
            reload_config_rx: Arc::new(parking_lot::Mutex::new(Some(reload_config_rx))),
            stop_tx: parking_lot::Mutex::new(Some(stop_tx)),
            stop_rx: Arc::new(parking_lot::Mutex::new(Some(stop_rx))),
        };

        Class::from_existing("Instance").wrap_data(instance, &*INSTANCE_WRAPPER)
    }

    fn spawn() -> NilClass {
        let inner = itself.get_data(&*INSTANCE_WRAPPER);

        let rt = Runtime::new().map_err(|e| {
                let exception = AnyException::new("ExogressError", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();

        let resolver = TokioAsyncResolver::from_system_conf(TokioHandle)
            .map_err(|e| {
                let exception = AnyException::new("ExogressError", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();


        // let stop_rx = inner
        //     .stop_rx
        //     .lock()
        //     .take()
        //     .ok_or_else(|| {
        //         let exception = AnyException::new("ExogressError", Some("instance has already been spawned"));
        //         VM::raise_ex(exception);
        //     })
        //     .unwrap();

        let work = || {
            let resolver = resolver.clone();
            let reload_config_tx = inner.reload_config_tx.lock().clone();
            let reload_config_rx = inner.reload_config_rx.clone();
            let stop_rx = inner.stop_rx.clone();

            rt.block_on(async move {
                println!("Spawn block_on");
                if let (Some(client), Some(reload_config_rx), Some(stop_rx)) = (inner.client.lock().take(), reload_config_rx.lock().take(), stop_rx.lock().take()) {
                    let spawn = client.spawn(reload_config_tx.clone(), reload_config_rx, resolver);

                    tokio::select! {
                        r = spawn => {
                            println!("spawn stopped");
                            if let Err(e) = r {
                                return Err(anyhow::anyhow!(e.to_string()));
                            }
                        },
                        _ = stop_rx => {
                            println!("stop by request");
                            // info!("stop exogress instance by request");
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("instance has already been spawned"));
                }

                Ok(())
            })
        };

        let unblocking_function = || {};

        let result = Thread::call_without_gvl(
            work,
            Some(unblocking_function)
        );

        result
            .map_err(|e| {
                let exception = AnyException::new("ExogressError", Some(e.to_string().as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();

        NilClass::new()
    }


    fn reload() -> NilClass {
        let inner = itself.get_data(&*INSTANCE_WRAPPER);

        inner.reload_config_tx
            .lock()
            .unbounded_send(())
            .map_err(|e| {
                let exception = AnyException::new("ExogressError", Some(format!("failed to send reload request: {}", e).as_str()));
                VM::raise_ex(exception);
            })
            .unwrap();

        NilClass::new()
    }

    fn stop() -> NilClass {
        let inner = itself.get_data(&*INSTANCE_WRAPPER);

        inner.stop_tx
            .lock()
            .take()
            .ok_or_else(|| {
                let exception = AnyException::new("ExogressError", Some("instance already stopped"));
                VM::raise_ex(exception)
            })
            .unwrap()
            .send(())
            .map_err(|_| {
                let exception = AnyException::new("ExogressError", Some("failed to send reload request"));
                VM::raise_ex(exception)
            })
            .unwrap();

        NilClass::new()
    }
);
