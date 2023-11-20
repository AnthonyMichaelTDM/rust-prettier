#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_13848_js_format_1_6830a6cc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n */\n/**\n */\ndeclare opaque type Type: string;\n\n/* ! DON'T add code before comment */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n */\n/**\n */\ndeclare opaque type Type: string;\n\n/* ! DON'T add code before comment */");
}
#[test]
fn test_issue_13848_2_js_format_1_6b68727f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n */\n/**\n */\ndeclare opaque type Type1: string;\ndeclare opaque type Type2: string;\n\n/* ! DON'T add code before comment */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n */\n/**\n */\ndeclare opaque type Type1: string;\ndeclare opaque type Type2: string;\n\n/* ! DON'T add code before comment */");
}
