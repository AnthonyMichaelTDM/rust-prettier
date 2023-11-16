#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_a_js_format_1_2705cde1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule A\n * @flow\n */\n\nimport type T from \"T\";\n\nexport default class {\n  p: T;\n\n  constructor() {\n    this.p = 0;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule A\n * @flow\n */\n\nimport type T from \"T\";\n\nexport default class {\n  p: T;\n\n  constructor() {\n    this.p = 0;\n  }\n}");
}
#[test]
fn test_b_js_format_1_8ca15f53() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nimport A from \"A\"\n\nclass B extends A {\n  p: string; // OK, string ~> any\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nimport A from \"A\";\n\nclass B extends A {\n  p: string; // OK, string ~> any\n}");
}
#[test]
fn test_t_js_format_1_d8c5f2cf() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/**\n * @providesModule T\n */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/**\n * @providesModule T\n */");
}
