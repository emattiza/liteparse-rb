# Changelog

## v0.1.9 (2026-06-24)

- Relocatable platform gems — precompiled .bundle/.so no longer linked to CI Ruby
- Remove magnus embed feature for runtime symbol resolution

## v0.1.4 (2026-06-24)

- Fix: simplify release workflow — publish source gem only (platform gems later)

## v0.1.3 (2026-06-24)

- Fix: output built gem to `pkg/` where release action expects it

## v0.1.2 (2026-06-24)

- Fix: run `bundle install` after version bumps to sync Gemfile.lock
- Fix: add `rake release` task for rubygems/release-gem action

## v0.1.1 (2026-06-24)

- Fix: release workflow — add `rake release` task for source gem publishing

## v0.1.0 (2026-06-24)

- Initial release of liteparse-rb
- PDF parsing with layout-aware text extraction (text items, bounding boxes, fonts)
- OCR via Tesseract (auto-downloads traineddata, ImageMagick for image files)
- Screenshot rendering of PDF pages to PNG
- Text search across parsed items
- `parse_bytes` for in-memory document parsing
- Prebuilt platform gems for arm64-darwin, x86_64-linux, aarch64-linux
- Source gem for all other platforms