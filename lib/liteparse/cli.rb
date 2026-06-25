require_relative "../liteparse"

module LiteParse
  # CLI support is handled by the native `lit` binary.
  # Run `lit --help` from the command line for usage.
  #
  # This module provides a programmatic entry point that delegates
  # to the native CLI runner.
  #
  # @example
  #   LiteParse::CLI.run(["parse", "document.pdf"])
  module CLI
    module_function

    # Run the CLI with the given arguments.
    # @param args [Array<String>] Command-line arguments (defaults to ARGV)
    # @return [void]
    def run(args = ARGV)
      LiteParse.run_cli(args)
    rescue => e
      $stderr.puts "Error: #{e.message}"
      exit 1
    end
  end
end