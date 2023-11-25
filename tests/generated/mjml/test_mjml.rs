#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_mjml_format_1_8bb12296() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<mjml></mjml");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<mjml></mjml>");
}
#[test]
fn test_head_mjml_format_1_7a39d5c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<mjml>\n  <mj-head>\n    <mj-title>\n    \n    \n    The green fix eats mango.\n    \n    \n    </mj-title>\n    <mj-breakpoint width=\"600px\" />\n    <mj-preview>Do \n    \n    you like cheese? We do!</mj-preview>\n    <mj-attributes>\n      <mj-text \n      \n      \n      \n      align=\"left\" color=\"#000\"\n      \n      \n         font-family=\"-apple-system,BlinkMacSystemFont,Helvetica,sans-serif\" />\n    </mj-attributes>\n  </mj-head>\n</mjml>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<mjml>\n  <mj-head>\n    <mj-title> The green fix eats mango. </mj-title>\n    <mj-breakpoint width=\"600px\" />\n    <mj-preview>Do you like cheese? We do!</mj-preview>\n    <mj-attributes>\n      <mj-text\n        align=\"left\"\n        color=\"#000\"\n        font-family=\"-apple-system,BlinkMacSystemFont,Helvetica,sans-serif\"\n      />\n    </mj-attributes>\n  </mj-head>\n</mjml>");
}
