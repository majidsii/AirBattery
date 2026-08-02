//! Integration tests for this crate.

use diagnostics::sanitize_identifier;

#[test]
fn sanitizer_is_stable_and_does_not_expose_input() {
    let first = sanitize_identifier("AA:BB:CC:DD:EE:FF");
    let second = sanitize_identifier("AA:BB:CC:DD:EE:FF");
    assert_eq!(first, second);
    assert!(!first.contains("AA:BB"));
}
