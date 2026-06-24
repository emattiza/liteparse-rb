require_relative "types"

module LiteParse
  # LiteParse is defined natively in the Rust extension.
  # This file exists to mirror the Python wrapper structure and provide
  # a convenient require path.
  #
  # Usage:
  #   require "liteparse"
  #   parser = LiteParse::LiteParse.new(ocr_enabled: true)
  #   result = parser.parse("document.pdf")
  #   puts result.text
end