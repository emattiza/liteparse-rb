require "rake/extensiontask"

Rake::ExtensionTask.new("liteparse") do |ext|
  ext.lib_dir = "lib/liteparse"
  ext.source_pattern = "*.rs"
  ext.ext_dir = "ext/liteparse"
end

task default: :compile