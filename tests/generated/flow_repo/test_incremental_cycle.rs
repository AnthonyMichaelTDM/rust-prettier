#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_d6707c2f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nclass A {\n  b: number;\n  c: string;\n}\n\nmodule.exports = A;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nclass A {\n  b: number;\n  c: string;\n}\n\nmodule.exports = A;"
    );
    Ok(())
}
#[test]
fn test_b_js_format_1_01dda3b4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar A = require ('./A');\nimport type C from './C';\n\nclass B extends A {\n  c: C;\n}\n\nmodule.exports = B;") ? ;
    assert_eq ! (formatted , "// @flow\nvar A = require(\"./A\");\nimport type C from \"./C\";\n\nclass B extends A {\n  c: C;\n}\n\nmodule.exports = B;");
    Ok(())
}
#[test]
fn test_c_js_format_1_4d4e6c9e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nvar A = require ('./A');\nimport type B from './B';\n\nclass C extends A {\n  b: B;\n}\n\nmodule.exports = C;") ? ;
    assert_eq ! (formatted , "// @flow\nvar A = require(\"./A\");\nimport type B from \"./B\";\n\nclass C extends A {\n  b: B;\n}\n\nmodule.exports = C;");
    Ok(())
}
