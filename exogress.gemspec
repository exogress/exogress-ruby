# frozen_string_literal: true

require_relative "lib/exogress/version"

Gem::Specification.new do |spec|
  spec.name          = "exogress"
  spec.version       = Exogress::VERSION
  spec.authors       = ["Exogress Team"]
  spec.email         = ["team@exogress.com"]

  spec.summary       = "Exogress client for Ruby"
  spec.description   = "Exogress is an edge network built for your cloud"
  spec.homepage      = "https://exogess.com"
  spec.license       = "Apache 2.0"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.4.0")

#   spec.metadata["allowed_push_host"] = "TODO: Set to 'http://mygemserver.com'"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/exogress/exogress-ruby"
#   spec.metadata["changelog_uri"] = "TODO: Put your gem's CHANGELOG.md URL here."

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{\A(?:test|spec|features)/}) }
  end
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = %w(Rakefile)

  spec.add_dependency 'rutie', '~> 0.0.4'
  spec.add_development_dependency "bundler"
  spec.add_development_dependency "rake", "~> 10.0"

end
