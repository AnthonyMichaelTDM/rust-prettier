#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_custom_selectors_css_format_1_00ad5455() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@custom-selector  :--button     button,.button;\n@custom-selector       :--enter     :hover,:focus      ;\n\n:--button {display: inline-block;\n}:--button:--enter{\n  text-decoration:underline;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@custom-selector :--button button, .button;\n@custom-selector :--enter :hover, :focus;\n\n:--button {\n  display: inline-block;\n}\n:--button:--enter {\n  text-decoration: underline;\n}");
}
