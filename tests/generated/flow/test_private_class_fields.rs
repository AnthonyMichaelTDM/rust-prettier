#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_quotes_js_quote_propsconsistent_format_1_075ff22b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("class Foo\n{\na:a;\n  'q-w': ee;\n#privateProp: number;}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  \"a\": a;\n  \"q-w\": ee;\n  #privateProp: number;\n}"
    );
}
