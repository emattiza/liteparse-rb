require_relative "../liteparse"

module LiteParse
  # CLI support is handled by the native `lit` binary.
  # Run `lit --help` from the command line for usage.
  module CLI
    module_function

    def run(args = ARGV)
      LiteParse.run_cli(args)
    rescue => e
      $stderr.puts "Error: #{e.message}"
      exit 1
    end
  end
end