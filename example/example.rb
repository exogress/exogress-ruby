require_relative "../lib/exogress"
require 'logger'

logger = Logger.new(STDOUT)
logger.level = Logger::WARN

Instance.set_logger(logger)
logger.info("Starting exogress.!")
instance = Instance.new({
    access_key_id: "01F68JEA8XW0MM1XGGR47F7KSD",
    secret_access_key: "a83Xj28xao6UkHRasZUhVVrrhc26w8RMJsyV7kkgn7jU",
    account: "glebpom",
    project: "location-tester",
    labels: {
        "label1": "val1",
    }
})

Thread.new do
    print("Spawn instance!")
    instance.spawn()
    print("instance stopped")
end

sleep(5)
print("reload")
instance.reload
sleep(5)
print("stop")
instance.stop
