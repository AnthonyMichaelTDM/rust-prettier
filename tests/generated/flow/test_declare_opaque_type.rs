#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_13848_js_format_1_6830a6cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n */\n/**\n */\ndeclare opaque type Type: string;\n\n/* ! DON'T add code before comment */") ? ;
    assert_eq ! (formatted , "/**\n */\n/**\n */\ndeclare opaque type Type: string;\n\n/* ! DON'T add code before comment */");
    Ok(())
}
#[test]
fn test_issue_13848_2_js_format_1_6b68727f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n */\n/**\n */\ndeclare opaque type Type1: string;\ndeclare opaque type Type2: string;\n\n/* ! DON'T add code before comment */") ? ;
    assert_eq ! (formatted , "/**\n */\n/**\n */\ndeclare opaque type Type1: string;\ndeclare opaque type Type2: string;\n\n/* ! DON'T add code before comment */");
    Ok(())
}
