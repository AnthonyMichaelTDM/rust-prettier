#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_tag_should_in_fill_html_format_1_fc6dc04e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<a-long-long-long-element>foo bar foo bar\n  foo bar foo bar foo bar foo bar foo bar\n  foo bar foo bar</a-long-long-long-element>\n<!-- The end tag should stay in 80 print width -->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<a-long-long-long-element\n  >foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo\n  bar</a-long-long-long-element\n>\n<!-- The end tag should stay in 80 print width -->");
}
