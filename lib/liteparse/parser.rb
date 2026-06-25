require_relative "types"

module LiteParse
  # LiteParse is defined natively in the Rust extension.
  # This file exists to mirror the Python wrapper structure and provide
  # a convenient require path.
  #
  # @example Basic usage
  #   require "liteparse"
  #   parser = LiteParse::LiteParse.new(ocr_enabled: true)
  #   result = parser.parse("document.pdf")
  #   puts result.text

  # @!method parse(input)
  #   Parse a document from a file path.
  #   @param input [String] Path to the document file (.pdf, .docx, .pptx, .xlsx, .html, image, etc.)
  #   @return [LiteParse::ParseResult] Parsed document with pages, text, and images
  #   @raise [RuntimeError] If parsing fails
  #   @example
  #     result = parser.parse("report.pdf")
  #     result.pages.each { |page| puts page.text }

  # @!method parse_bytes(data)
  #   Parse a document from raw bytes.
  #   @param data [String] Raw document bytes (binary string)
  #   @return [LiteParse::ParseResult] Parsed document with pages, text, and images
  #   @raise [RuntimeError] If parsing fails
  #   @example
  #     data = File.binread("report.pdf")
  #     result = parser.parse_bytes(data)

  # @!method config
  #   Get the current parser configuration.
  #   @return [LiteParse::Config] The active configuration
end