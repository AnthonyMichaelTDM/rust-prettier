#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_media_indent_with_import_css_format_1_b7c10ce3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@import \"foo\";@import \"bar\";\n.wrapper {\n@media (min-width: 1025px) {\nmax-width: 600px;\n}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@import \"foo\";\n@import \"bar\";\n.wrapper {\n  @media (min-width: 1025px) {\n    max-width: 600px;\n  }\n}");
}
