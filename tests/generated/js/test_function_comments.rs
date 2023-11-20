#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_params_trail_comments_js_format_1_e5bb13f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function Foo(\n  bar\n  // Trailing comment\n) {}\n\nfunction Foo(\n  {bar}\n  // Trailing comment\n) {}\n\nfunction Foo(\n  [bar]\n  // Trailing comment\n) {}\n\nfunction Foo(\n  bar = 1\n  // Trailing comment\n) {}\n\nfunction Foo(\n  ...bar\n  // Trailing comment\n) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function Foo(\n  bar,\n  // Trailing comment\n) {}\n\nfunction Foo(\n  { bar },\n  // Trailing comment\n) {}\n\nfunction Foo(\n  [bar],\n  // Trailing comment\n) {}\n\nfunction Foo(\n  bar = 1,\n  // Trailing comment\n) {}\n\nfunction Foo(\n  ...bar\n  // Trailing comment\n) {}");
}
