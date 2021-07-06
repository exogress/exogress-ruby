#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, VM};

mod instance;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rutie_exogress() {
    Class::new("Instance", None).define(|klass| {
        klass.def_self("new", instance::new);
        klass.def("spawn", instance::spawn);
        klass.def("reload", instance::reload);
        klass.def("stop", instance::stop);
    });
}