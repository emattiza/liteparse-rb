require_relative "liteparse/liteparse"

# All types (TextItem, ParsedPage, ParseResult, etc.) are defined natively
# in the Rust extension. This file re-exports them for convenience.

module LiteParse
  # No additional Ruby wrapping needed — the native classes are registered
  # directly on the LiteParse module by the Rust init function.
end