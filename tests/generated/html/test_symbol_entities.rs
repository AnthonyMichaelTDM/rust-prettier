#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_symbol_entitites_html_format_1_8fe3ed5a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<p>I will display &euro;</p>\n<p>I will display &excl;</p>\n<p>I will display &#8364;</p>\n<p>I will display &#x20AC;</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<p>I will display &euro;</p>\n<p>I will display &excl;</p>\n<p>I will display &#8364;</p>\n<p>I will display &#x20AC;</p>");
}
