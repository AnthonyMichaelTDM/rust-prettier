#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_80a03694() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass Foo {\n  annotationOnly: string;\n  initOnly = 'asdf'\n  initWithAnnotation: string = 'asdf';\n  [computed]: string;\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nclass Foo {\n  annotationOnly: string;\n  initOnly = \"asdf\";\n  initWithAnnotation: string = \"asdf\";\n  [computed]: string;\n}");
    Ok(())
}
