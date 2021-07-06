# frozen_string_literal: true
require 'logger'

RSpec.describe Exogress do
  it "has a version number" do
    expect(Exogress::VERSION).not_to be nil
  end

  it "instance init with required params" do
    Instance.new({
        access_key_id: "01F68JEA8XW0MM1XGGR47F7KSD",
        secret_access_key: "aglkjqh3lthwaelkgjhslkjdfghlskdjfg",
        account: "user",
        project: "project",
    })
  end

  it "instance init with logger" do
    logger = Logger.new(STDOUT)

    Instance.new({
        access_key_id: "01F68JEA8XW0MM1XGGR47F7KSD",
        secret_access_key: "aglkjqh3lthwaelkgjhslkjdfghlskdjfg",
        account: "user",
        project: "project",
        logger: logger,
    })
  end

  it "instance init with additional params" do
    Instance.new({
        access_key_id: "01F68JEA8XW0MM1XGGR47F7KSD",
        secret_access_key: "aglkjqh3lthwaelkgjhslkjdfghlskdjfg",
        account: "user",
        project: "project",
        watch_config: true,
        config_path: "./Exofile.yml",
        labels: {
            "label1" => "val1",
            label2: "val2",
        },
    })
  end
end
