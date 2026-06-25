$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "liteparse"
require "minitest/autorun"
require "fileutils"

FIXTURES_DIR = File.expand_path("fixtures", __dir__)
SAMPLE_PDF = File.join(FIXTURES_DIR, "sample.pdf")
SAMPLE_DOC = File.join(FIXTURES_DIR, "sample3.doc")
RECEIPT_PNG = File.join(FIXTURES_DIR, "receipt.png")

def create_parser(**kwargs)
  LiteParse::LiteParse.new(**kwargs)
end

def imagemagick_available?
  system("convert --version > /dev/null 2>&1")
end

def skip_unless_imagemagick
  skip "ImageMagick not installed (needed for image file parsing)" unless imagemagick_available?
end