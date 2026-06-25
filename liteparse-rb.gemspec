require_relative "lib/liteparse/version"

Gem::Specification.new do |spec|
  spec.name = "liteparse-rb"
  spec.version = LiteParse::VERSION
  spec.authors = ["Evan Mattiza"]
  spec.summary = "Fast, lightweight PDF and document parsing with Ruby"
  spec.description = "Ruby bindings for LiteParse — an open-source document parser that extracts text with spatial layout information, bounding boxes, OCR support, and more."
  spec.homepage = "https://github.com/emattiza/liteparse-rb"
  spec.license = "Apache-2.0"
  spec.required_ruby_version = ">= 3.3"

  spec.files = Dir["lib/**/*.rb", "lib/**/*.{bundle,so,dylib}", "ext/**/*.{rb,rs}", "Cargo.toml", "Cargo.lock", "crates/**/*", "THIRDPARTY.md", "LICENSE"]
  spec.require_paths = ["lib"]

  spec.extensions = ["ext/liteparse/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
end