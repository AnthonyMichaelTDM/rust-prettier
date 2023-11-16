#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_css_format_1_a2be3917() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format(".blah {\n  /* hloow <|> */\n  background-color: white;\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        ".blah {\n  /* hloow <|> */\n  background-color: white;\n}"
    );
}
