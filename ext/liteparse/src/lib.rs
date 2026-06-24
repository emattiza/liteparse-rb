// Re-export all types and functions from the workspace crate.
pub use liteparse_ruby::*;

/// Registers all LiteParse classes/modules with Ruby.
#[magnus::init]
fn init(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    liteparse_ruby::define_liteparse_module(ruby)
}