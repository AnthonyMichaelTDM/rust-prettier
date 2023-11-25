#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_include_scss_format_1_07ae1284() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("@mixin foo() {\n  a {\n    color: #f99;\n  }\n}\n\n@include foo() /* comment*/")?;
    assert_eq!(
        formatted,
        "@mixin foo() {\n  a {\n    color: #f99;\n  }\n}\n\n@include foo(); /* comment*/"
    );
    Ok(())
}
#[test]
fn test_include_2_scss_format_1_56bfbaa1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("@mixin foo() {\n  a {\n    color: #f99;\n  }\n}\n\n@include foo() // comment")?;
    assert_eq!(
        formatted,
        "@mixin foo() {\n  a {\n    color: #f99;\n  }\n}\n\n@include foo(); // comment"
    );
    Ok(())
}
#[test]
fn test_url_scss_format_1_63793fd5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("scss")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("@import ur\n  l(//fonts.googleapis.com/css?family=Open+Sans:400,400italic);")?;
    assert_eq!(
        formatted,
        "@import ur\n  l(; //fonts.googleapis.com/css?family=Open+Sans:400,400italic);"
    );
    Ok(())
}
