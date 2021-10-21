# frozen_string_literal: true

require_relative "../spec_helper"


RSpec.describe "Ruby sample app integration tests" do
  it "Blerg" do
    Cutlass::App.new(ruby_sample_buildpack).transaction do |app|
      write_gemfile(app.tmpdir)

      app.pack_build do |result|
        puts result.inspect
        expect(result.stdout).to include("blkajsdlfkjasdklj")
        expect(result.success?).to be_truthy
      end
    end
  end

  def ruby_sample_buildpack
    examples_dir.join("example-02-ruby-sample")
  end

  def write_gemfile(dir)
    dir.join("Gemfile").write(<<~EOM)
      source 'https://rubygems.org'
      gem 'rake'
    EOM

    dir.join("Gemfile.lock").write(<<~EOM)
      GEM
        remote: https://rubygems.org/
        specs:
          rake (13.0.6)

      PLATFORMS
        ruby
        x86_64-darwin-20
          x86_64-linux DEPENDENCIES
          rake

        BUNDLED WITH
          2.2.27
            EOM
  end
end
