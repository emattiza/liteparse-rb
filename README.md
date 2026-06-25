# liteparse-rb

Ruby bindings for [LiteParse](https://github.com/run-llama/liteparse) — a fast, open-source document parser that extracts text with spatial layout information, bounding boxes, and OCR support.

Built with [magnus](https://github.com/matsadler/magnus) for native Rust→Ruby FFI.

## Installation

Add `liteparse-rb` to your application with Bundler:

```sh
bundle add liteparse-rb
```

This will add the gem to your `Gemfile` and install it.

You can also install it directly with:

```sh
gem install liteparse-rb
```

## Runtime Dependencies

The gem loads PDFium dynamically at runtime and shells out to external tools for certain input formats. Prebuilt platform gems include everything except the items below.

| Dependency | Required for | When needed | Config / Env var |
|---|---|---|---|
| **PDFium** (`libpdfium.so`/`.dylib`) | All PDF parsing and rendering | Always | `PDFIUM_LIB_PATH` (directory containing the shared library) |
| **Tesseract** (C++ library + traineddata) | OCR on scanned pages and images | Always (feature default, can disable with `tesseract: false`) | `TESSDATA_PREFIX` or `tessdata_path:` config option; traineddata auto-downloads if missing |
| **ImageMagick** (`magick` or `convert`) | Image file input (`.jpg`, `.png`, `.gif`, `.bmp`, `.tiff`, `.webp`, `.svg`) | Only when parsing image files | — |
| **LibreOffice** (`libreoffice` or `soffice`) | Office document input (`.docx`, `.pptx`, `.xlsx`, `.odt`, etc.) | Only when parsing office files | — |
| **Ghostscript** (`gs`) | Vector format conversion (`.svg`, `.eps`, `.ps`, `.ai`) | Only when parsing vector files (used by ImageMagick) | — |

### macOS (Homebrew)

```sh
brew install pdfium-binaries tesseract imagemagick libreoffice ghostscript
```

Set `PDFIUM_LIB_PATH` in your shell profile:

```sh
export PDFIUM_LIB_PATH="$(brew --prefix pdfium-binaries)/lib"
```

### macOS (Nix)

```sh
nix shell nixpkgs#pdfium-binaries nixpkgs#tesseract nixpkgs#imagemagick nixpkgs#libreoffice nixpkgs#ghostscript
```

Or add to `shell.nix` / `flake.nix`:

```nix
{pkgs}: pkgs.mkShell {
  buildInputs = [
    pkgs.pdfium-binaries
    pkgs.tesseract
    pkgs.imagemagick
    pkgs.libreoffice
    pkgs.ghostscript
  ];
  shellHook = ''
    export PDFIUM_LIB_PATH="${pkgs.pdfium-binaries}/lib"
  '';
}
```

### Linux (Debian/Ubuntu)

```sh
sudo apt-get install -y cmake libtesseract-dev tesseract-ocr-eng imagemagick libreoffice ghostscript
```

PDFium is bundled at build time for `x86_64-linux` and `aarch64-linux` platform gems. If using the source gem or a custom build, set `PDFIUM_LIB_PATH` as needed.

## Usage

### Basic parsing

```ruby
require "liteparse"

parser = LiteParse::LiteParse.new
result = parser.parse("document.pdf")

puts result.text          # full document text
puts result.num_pages     # page count

result.pages.each do |page|
  puts "Page #{page.page_num}: #{page.width}x#{page.height}"
  puts page.text
end
```

### Configuration

All keyword args match the [Python API](https://github.com/run-llama/liteparse):

```ruby
parser = LiteParse::LiteParse.new(
  ocr_enabled: false,
  output_format: "markdown",
  max_pages: 10,
  dpi: 200,
  password: "secret",
  quiet: true,
  image_mode: "embed",
  extract_links: true,
)
```

### Parse from bytes

```ruby
data = File.binread("document.pdf")
result = parser.parse_bytes(data)
```

### Text items with bounding boxes

Each page exposes `text_items` — individual word/phrase runs with position data:

```ruby
page.text_items.each do |item|
  puts "#{item.text} at (#{item.x}, #{item.y}) #{item.width}x#{item.height}"
  puts "  font: #{item.font_name}, size: #{item.font_size}"
  puts "  confidence: #{item.confidence}"  # OCR confidence (nil for native text)
end
```

### Searching text items

```ruby
items = page.text_items
matches = LiteParse.search_items(items, "swimmer name", case_sensitive: false)

matches.each do |m|
  puts "Found '#{m.text}' at x=#{m.x} y=#{m.y}"
end
```

### Screenshots

```ruby
screenshots = parser.screenshot("document.pdf", page_numbers: [1, 3])
screenshots.each do |s|
  File.binwrite("page_#{s.page_num}.png", s.image_bytes)
end
```

### Inspecting config

```ruby
cfg = parser.config
puts cfg.ocr_enabled       # true/false
puts cfg.output_format     # "json", "text", "markdown"
puts cfg.dpi               # 150.0
```

## Types

| Ruby Class | Description |
|---|---|
| `LiteParse::LiteParse` | Main parser |
| `LiteParse::ParseResult` | Parsed document with pages/text/images |
| `LiteParse::ParsedPage` | Single page with text items |
| `LiteParse::TextItem` | Word/phrase with bounding box |
| `LiteParse::ExtractedImage` | Embedded raster image (in `embed` mode) |
| `LiteParse::ScreenshotResult` | Page screenshot PNG bytes |
| `LiteParse::Config` | Resolved configuration |
| `LiteParse::ParseError` | Raised on parse failures |

## Development

Prerequisites: Rust toolchain, Ruby 3.3+, cmake.

```sh
bundle install
rake compile
ruby -I lib -e "require 'liteparse'; puts LiteParse::VERSION"
```

## Release

```sh
# Bump version in lib/liteparse/version.rb and crates/*/Cargo.toml
# Add entry to CHANGELOG.md
bundle install    # syncs Gemfile.lock with new version
git add -A && git commit -m "Release v0.1.0"
git tag v0.1.0
git push && git push --tags
```

Tag push triggers CI then publishes prebuilt platform gems to rubygems.org.

## License

Apache-2.0 — same as LiteParse upstream.
