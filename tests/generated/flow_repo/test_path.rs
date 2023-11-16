#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_while_js_format_1_bbe9cf09() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var x = 1;\nwhile(typeof x == \"number\" || typeof x == \"string\") {\n    x = x + 1;\n    if (true) x = \"\";\n}\nvar z:number = x;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x = 1;\nwhile (typeof x == \"number\" || typeof x == \"string\") {\n  x = x + 1;\n  if (true) x = \"\";\n}\nvar z: number = x;");
}
