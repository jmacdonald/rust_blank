# frozen_string_literal: true

require_relative "lib/rust_blank/version"

Gem::Specification.new do |spec|
  spec.name = "rust_blank"
  spec.version = RustBlank::VERSION
  spec.authors = ["Jordan MacDonald"]
  spec.email = ["jordan@wastedintelligence.com"]

  spec.summary = "Rust-based String#is_blank method"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|circleci)|appveyor)})
    end
  end
  spec.require_paths = ["lib"]

  spec.extensions = ["ext/rust_blank/extconf.rb"]

  # needed until rubygems supports Rust support is out of beta
  spec.add_dependency "rb_sys", "~> 0.9.39"

  # only needed when developing or packaging your gem
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"
end
