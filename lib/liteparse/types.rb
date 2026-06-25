require_relative "liteparse/liteparse"

# All types (TextItem, ParsedPage, ParseResult, etc.) are defined natively
# in the Rust extension. This file re-exports them for convenience.

module LiteParse
  # A single item of text with spatial position information.
  # @!method text
  #   @return [String] The text content of this item
  # @!method x
  #   @return [Float] Left edge X coordinate of the text bounding box
  # @!method y
  #   @return [Float] Top edge Y coordinate of the text bounding box
  # @!method width
  #   @return [Float] Width of the text bounding box
  # @!method height
  #   @return [Float] Height of the text bounding box
  # @!method font_name
  #   @return [String, nil] Name of the font used, if available
  # @!method font_size
  #   @return [Float, nil] Font size in points, if available
  # @!method confidence
  #   @return [Float, nil] OCR/text extraction confidence (0.0–1.0), if available
  class TextItem; end

  # A single parsed page from a document.
  # @!method page_num
  #   @return [Integer] 1-indexed page number
  # @!method width
  #   @return [Float] Page width in points
  # @!method height
  #   @return [Float] Page height in points
  # @!method text
  #   @return [String] Full concatenated text of this page
  # @!method text_items
  #   @return [Array<LiteParse::TextItem>] Individual text items with spatial information
  class ParsedPage; end

  # The complete result of parsing a document.
  # @!method pages
  #   @return [Array<LiteParse::ParsedPage>] All parsed pages
  # @!method text
  #   @return [String] Full concatenated text across all pages
  # @!method images
  #   @return [Array<LiteParse::ExtractedImage>] Images extracted from the document
  # @!method num_pages
  #   @return [Integer] Total number of pages parsed
  # @!method get_page(page_num)
  #   Retrieve a specific page by its 1-indexed page number.
  #   @param page_num [Integer] 1-indexed page number
  #   @return [LiteParse::ParsedPage, nil] The page, or nil if not found
  class ParseResult; end

  # An image extracted from a parsed document.
  # @!method id
  #   @return [String] Image identifier
  # @!method page
  #   @return [Integer] Page number where the image was found
  # @!method format
  #   @return [String] Image format (e.g. "png", "jpeg")
  # @!method bytes
  #   @return [String] Raw image bytes (binary string)
  class ExtractedImage; end

  # A screenshot of a document page rendered as an image.
  # @!method page_num
  #   @return [Integer] 1-indexed page number of the screenshot
  # @!method width
  #   @return [Integer] Width of the screenshot in pixels
  # @!method height
  #   @return [Integer] Height of the screenshot in pixels
  # @!method image_bytes
  #   @return [String] PNG image bytes (binary string)
  class ScreenshotResult; end

  # The current configuration of a LiteParse parser instance.
  # @!method ocr_language
  #   @return [String] Language used for OCR (e.g. "eng")
  # @!method ocr_enabled
  #   @return [Boolean] Whether OCR is enabled
  # @!method ocr_server_url
  #   @return [String, nil] External OCR server URL, if configured
  # @!method ocr_server_headers
  #   @return [Hash<String, String>, nil] Headers for OCR server requests
  # @!method tessdata_path
  #   @return [String, nil] Path to Tesseract tessdata directory
  # @!method max_pages
  #   @return [Integer] Maximum pages to parse
  # @!method target_pages
  #   @return [String, nil] Page range expression, if configured
  # @!method dpi
  #   @return [Float] Rendering DPI
  # @!method output_format
  #   @return [String] Output format: "json", "text", or "markdown"
  # @!method preserve_very_small_text
  #   @return [Boolean] Whether very small text is preserved
  # @!method password
  #   @return [String, nil] Document password, if set
  # @!method quiet
  #   @return [Boolean] Whether non-error output is suppressed
  # @!method num_workers
  #   @return [Integer] Number of worker threads
  class Config; end

  # No additional Ruby wrapping needed — the native classes are registered
  # directly on the LiteParse module by the Rust init function.
end