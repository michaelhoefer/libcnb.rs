require "bundler/setup"
require "pathname"

def spec_dir
  Pathname(__dir__)
end

def root_dir
  spec_dir.join("..")
end

def examples_dir
  root_dir.join("examples")
end


Cutlass.config do |config|
  config.default_builder = "heroku/buildpacks:20"
end
