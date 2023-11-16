#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_indexer_js_format_1_321eff5c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("declare class A {\n  [number]: boolean;\n  static [string]: boolean;\n}\n\n// Read-only\ndeclare class B {\n  +[number]: boolean;\n  static +[string]: boolean;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class A {\n  [number]: boolean;\n  static [string]: boolean;\n}\n\n// Read-only\ndeclare class B {\n  +[number]: boolean;\n  static +[string]: boolean;\n}");
}
