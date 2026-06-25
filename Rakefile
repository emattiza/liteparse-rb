require "rake/extensiontask"
require "rake/testtask"

Rake::ExtensionTask.new("liteparse") do |ext|
  ext.lib_dir = "lib/liteparse"
  ext.source_pattern = "*.rs"
  ext.ext_dir = "ext/liteparse"
  ext.cross_compile = true
  ext.cross_platform = ["arm64-darwin", "x86_64-linux", "aarch64-linux"]
end

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/test_*.rb"]
  t.warning = false
end

begin
  require "yard"
  YARD::Rake::YardocTask.new
rescue LoadError
  # yard not installed
end

task default: :compile