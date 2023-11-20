#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_quotes_js_quote_propsconsistent_format_1_075ff22b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .quote_props("consistent")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class Foo\n{\na:a;\n  'q-w': ee;\n#privateProp: number;}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo {\n  \"a\": a;\n  \"q-w\": ee;\n  #privateProp: number;\n}"
    );
}
