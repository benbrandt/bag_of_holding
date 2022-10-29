#![warn(
    clippy::pedantic,
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused
)]

use features::Feature;
use serde_json::json;
use sources::Book;

#[test]
fn serializes_with_citation() {
    let feature = Feature::new("Title", Book::Phb);
    assert_eq!(json!(feature), "Title (PHB)");
}
