#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_b59370c3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("keep = <p>\n  Welcome to the <strong>Universal React Starter-kyt</strong>.\n  This starter kyt should serve as the base for an advanced,\n  server-rendered React app.\n</p>\n\nnewlines_text =\n  <div>\n    hi\n    there\n    how\n\n    are you\n\n\n    are you fine today?\n  </div>\n\nnewlines_text_spaced =\n  <div>\n\n    space above\n\n    space below\n\n  </div>\n\nnewlines_elems_spaced =\n  <div>\n\n    <span>space above</span>\n\n    <span>space below</span>\n\n  </div>\n\nnewlines_mixed =\n  <div>\n    hi\n    <span>there</span>\n\n    how\n\n    are <strong>you</strong>\n\n\n    are you fine today?\n  </div>\n\nnewlines_elems =\n  <div>\n    <div>\n\n\n      <div></div>\n\n\n    </div>\n\n\n    hi\n\n\n    <div></div>\n\n\n    <span />\n\n\n    <Big />\n\n\n  </div>\n\nregression_extra_newline = (\n  <div>\n    <span\n      className=\"nuclide-console-new-messages-notification-icon icon icon-nuclicon-arrow-down\"\n    />\n    New Messages\n  </div>\n);\n\n\nregression_extra_newline_2 = (\n  <div>\n    (\n    <FormattedMessage\n      id=\"some-id\"\n      defaultMessage=\"some loooooooooooooooooooooooooooong default\"\n    />\n    )\n  </div>\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "keep = (\n  <p>\n    Welcome to the <strong>Universal React Starter-kyt</strong>. This starter\n    kyt should serve as the base for an advanced, server-rendered React app.\n  </p>\n);\n\nnewlines_text = <div>hi there how are you are you fine today?</div>;\n\nnewlines_text_spaced = <div>space above space below</div>;\n\nnewlines_elems_spaced = (\n  <div>\n    <span>space above</span>\n\n    <span>space below</span>\n  </div>\n);\n\nnewlines_mixed = (\n  <div>\n    hi\n    <span>there</span>\n    how are <strong>you</strong>\n    are you fine today?\n  </div>\n);\n\nnewlines_elems = (\n  <div>\n    <div>\n      <div></div>\n    </div>\n    hi\n    <div></div>\n    <span />\n    <Big />\n  </div>\n);\n\nregression_extra_newline = (\n  <div>\n    <span className=\"nuclide-console-new-messages-notification-icon icon icon-nuclicon-arrow-down\" />\n    New Messages\n  </div>\n);\n\nregression_extra_newline_2 = (\n  <div>\n    (\n    <FormattedMessage\n      id=\"some-id\"\n      defaultMessage=\"some loooooooooooooooooooooooooooong default\"\n    />\n    )\n  </div>\n);");
}
#[test]
fn test_windows_js_format_1_1476b1fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div>\nText\n</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div>Text</div>;");
}
