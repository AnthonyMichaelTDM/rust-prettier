#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_arrow_function_js_format_1_c1e27de2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const x = () => [].includes(true) || \"ontouchend\" in document\n\nconst y = () => [] in x") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = () => [].includes(true) || \"ontouchend\" in document;\n\nconst y = () => [] in x;");
}
#[test]
fn test_arrow_function_invalid_js_format_1_15b3722b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("() => [] in x");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "() => [] in x;");
}
