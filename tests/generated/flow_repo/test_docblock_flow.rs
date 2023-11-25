#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_license_with_flow_js_format_1_caad8d9d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* Copyright example */\n/* @flow */\n\n(\"\": void); // error")?;
    assert_eq!(
        formatted,
        "/* Copyright example */\n/* @flow */\n\n(\"\": void); // error"
    );
    Ok(())
}
#[test]
fn test_max_header_tokens_js_format_1_4078833d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n/* second token */\n/* third token */\n/**\n * After max_header_tokens (in .flowconfig), we no longer care:\n *\n * @flow\n */") ? ;
    assert_eq ! (formatted , "/* @flow */\n/* second token */\n/* third token */\n/**\n * After max_header_tokens (in .flowconfig), we no longer care:\n *\n * @flow\n */");
    Ok(())
}
#[test]
fn test_multiple_flows_1_js_format_1_e9ee9e7e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n/* @flow */")?;
    assert_eq!(formatted, "/* @flow */\n/* @flow */");
    Ok(())
}
#[test]
fn test_multiple_flows_2_js_format_1_fddae2f5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/**\n * @flow\n * @noflow\n */")?;
    assert_eq!(formatted, "/**\n * @flow\n * @noflow\n */");
    Ok(())
}
#[test]
fn test_multiple_provides_module_1_js_format_1_f6210eb3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule Foo\n * @providesModule Bar\n * @flow\n */")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule Foo\n * @providesModule Bar\n * @flow\n */"
    );
    Ok(())
}
#[test]
fn test_multiple_provides_module_2_js_format_1_5649bb3a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/**\n * @providesModule Foo\n * @flow\n */\n/**\n * @providesModule Bar\n */")?;
    assert_eq!(
        formatted,
        "/**\n * @providesModule Foo\n * @flow\n */\n/**\n * @providesModule Bar\n */"
    );
    Ok(())
}
#[test]
fn test_use_strict_with_flow_js_format_1_18e50e09() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("\"use strict\";\n/* @flow */\n\n(\"\": void); // error")?;
    assert_eq!(
        formatted,
        "\"use strict\";\n/* @flow */\n\n(\"\": void); // error"
    );
    Ok(())
}
#[test]
fn test_with_flow_js_format_1_3c2a4ad6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */\n\n(\"\": void); // error")?;
    assert_eq!(formatted, "/* @flow */\n\n(\"\": void); // error");
    Ok(())
}
#[test]
fn test_without_flow_js_format_1_e2384d32() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("/* some other comment */\n\n(\"\": void); // no error")?;
    assert_eq!(
        formatted,
        "/* some other comment */\n\n(\"\": void); // no error"
    );
    Ok(())
}
