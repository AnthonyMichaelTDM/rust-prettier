#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_property_js_format_1_c019b51c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  TEMPLATE =\n    // tab index is needed so we can focus, which is needed for keyboard events\n    '<div class=\"ag-large-text\" tabindex=\"0\">' +\n    '<div class=\"ag-large-textarea\"></div>' +\n    '</div>';\n}") ? ;
    assert_eq ! (formatted , "class X {\n  TEMPLATE =\n    // tab index is needed so we can focus, which is needed for keyboard events\n    '<div class=\"ag-large-text\" tabindex=\"0\">' +\n    '<div class=\"ag-large-textarea\"></div>' +\n    \"</div>\";\n}");
    Ok(())
}
#[test]
fn test_misc_js_format_1_06e90a8e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("class x {\n  focus() // comment 1\n  {\n    // comment 2\n  }\n}")?;
    assert_eq!(
        formatted,
        "class x {\n  focus() { // comment 1\n    // comment 2\n  }\n}"
    );
    Ok(())
}
#[test]
fn test_superclass_js_format_1_a7aef2bf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A // comment 1\n  // comment 2\n  extends B {}\n\nclass A1 extends B // comment1\n// comment2\n// comment3\n{}\n\nclass A2 /* a */ extends B {}\nclass A3 extends B /* a */ {}\nclass A4 extends /* a */ B {}\n\n(class A5 // comment 1\n  // comment 2\n  extends B {});\n\n(class A6 extends B // comment1\n// comment2\n// comment3\n{});\n\n(class A7 /* a */ extends B {});\n(class A8 extends B /* a */ {});\n(class A9 extends /* a */ B {});\n\nclass a extends b // comment\n{\n  constructor() {}\n}\n\nclass c extends d\n// comment2\n{\n  constructor() {}\n}\n\nclass C2  // comment\nextends Base\n{  foo(){} }") ? ;
    assert_eq ! (formatted , "class A // comment 1\n  // comment 2\n  extends B {}\n\nclass A1 extends B {\n  // comment1\n  // comment2\n  // comment3\n}\n\nclass A2 /* a */ extends B {}\nclass A3 extends B /* a */ {}\nclass A4 extends /* a */ B {}\n\n(class A5 // comment 1\n  // comment 2\n  extends B {});\n\n(class A6 extends B {\n  // comment1\n  // comment2\n  // comment3\n});\n\n(class A7 /* a */ extends B {});\n(class A8 extends B /* a */ {});\n(class A9 extends /* a */ B {});\n\nclass a extends b {\n  // comment\n  constructor() {}\n}\n\nclass c extends d {\n  // comment2\n  constructor() {}\n}\n\nclass C2 // comment\n  extends Base\n{\n  foo() {}\n}");
    Ok(())
}
