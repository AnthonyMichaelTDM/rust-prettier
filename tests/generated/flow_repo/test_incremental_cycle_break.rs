#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_e59a20bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule A\n * @flow\n */\n\nrequire('B');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule A\n * @flow\n */\n\nrequire(\"B\");"
    );
}
#[test]
fn test_b_js_format_1_664cdacc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/**\n * @providesModule B\n * @flow\n */\nrequire('A');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @providesModule B\n * @flow\n */\nrequire(\"A\");"
    );
}
