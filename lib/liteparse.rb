require_relative "liteparse/version"
require_relative "liteparse/liteparse"

module LiteParse
  # Generic error raised by LiteParse operations.
  class Error < StandardError; end
end

# YARD declarations for methods defined in the Rust extension.
# These are invisible to YARD, so we declare them with @!method.
class LiteParse::LiteParse
  # @!method self.new(**kwargs)
  #   Create a new LiteParse parser instance.
  #   @param kwargs [Hash] Keyword arguments for parser configuration
  #   @option kwargs [String] :ocr_language ("eng") Language for OCR
  #   @option kwargs [Boolean] :ocr_enabled (true) Enable OCR
  #   @option kwargs [String, nil] :ocr_server_url URL of an external OCR server
  #   @option kwargs [Hash<String, String>, nil] :ocr_server_headers Headers for OCR server requests
  #   @option kwargs [String, nil] :tessdata_path Path to Tesseract tessdata directory
  #   @option kwargs [Integer] :max_pages (1000) Maximum pages to parse
  #   @option kwargs [String, nil] :target_pages Page range expression (e.g. "1-5,7")
  #   @option kwargs [Float] :dpi (150.0) Rendering DPI
  #   @option kwargs [String] :output_format ("json") Output format: "json", "text", or "markdown"
  #   @option kwargs [Boolean] :preserve_very_small_text (false) Preserve tiny text
  #   @option kwargs [String, nil] :password Password for encrypted documents
  #   @option kwargs [Boolean] :quiet (false) Suppress non-error output
  #   @option kwargs [Integer] :num_workers Number of worker threads (auto-detected)
  #   @option kwargs [String] :image_mode ("placeholder") Image mode: "placeholder", "embed", or "off"
  #   @option kwargs [Boolean] :extract_links (false) Extract hyperlinks
  #   @return [LiteParse::LiteParse]
  #   @example
  #     parser = LiteParse::LiteParse.new(ocr_enabled: true, dpi: 200)

  # @!method screenshot(input, page_numbers: nil)
  #   Take screenshots of document pages.
  #   @param input [String] Path to the document file
  #   @param page_numbers [Array<Integer>, nil] Specific page numbers (1-indexed) to screenshot. nil = all pages.
  #   @return [Array<LiteParse::ScreenshotResult>] Screenshot results with image bytes
  #   @example
  #     parser = LiteParse::LiteParse.new
  #     screenshots = parser.screenshot("document.pdf", page_numbers: [1, 3])
  #     screenshots.each { |s| File.write("page_#{s.page_num}.png", s.image_bytes) }
end

# Wrap native new to accept 0 args (the Rust constructor expects 1 positional arg).
LiteParse::LiteParse.singleton_class.alias_method :native_new, :new
LiteParse::LiteParse.define_singleton_method(:new) do |**kwargs|
  native_new(kwargs.empty? ? nil : kwargs)
end

# Wrap screenshot to accept page_numbers as keyword arg (matching Python API).
LiteParse::LiteParse.alias_method :native_screenshot, :screenshot
LiteParse::LiteParse.define_method(:screenshot) do |input, page_numbers: nil|
  native_screenshot(input, page_numbers)
end