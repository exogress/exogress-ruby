# frozen_string_literal: true

require "bundler/gem_tasks"
require "rbconfig"

task default: :build_lib

# Mac OS with rbenv users keep leaving behind build artifacts from
#   when they tried to build against a statically linked Ruby and then
#   try against a dynamically linked one causing errors in the build result.
desc 'Preventative work'
task :tidy_up do
  sh 'cargo clean'
end

desc 'Build Rust extension'
task :build_lib do
  case RbConfig::CONFIG['host_os']
  when /darwin|mac os/
    sh 'cargo rustc --release -- -C link-args=-Wl,-undefined,dynamic_lookup'
  else
    sh 'cargo build --release'
  end
end

desc 'bundle install'
task :bundle_install do
  sh 'bundle install'
end

begin
    require "rspec/core/rake_task"
    RSpec::Core::RakeTask.new(spec: [:bundle_install, :build_lib]) do |t|
    end
rescue LoadError
end