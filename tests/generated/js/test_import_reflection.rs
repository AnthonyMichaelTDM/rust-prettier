#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_js_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_comments_js_format_1_96b7b277() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* 0 */import /* 1 */module /* 2 */from /* 3 */from /* 4 */\"./module.wasm\"/* 5 */;",
    )?;
    assert_eq ! (formatted , "/* 0 */ import module /* 1 */ /* 2 */ from /* 3 */ from /* 4 */ \"./module.wasm\" /* 5 */;");
    Ok(())
}
#[test]
fn test_import_reflection_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_format_1_ca41c905() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import module foo from \"./module.wasm\";")?;
    assert_eq!(formatted, "import module foo from \"./module.wasm\";");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_default_import_mjs_format_1_7e4812c8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("import module foo from \"./module.wasm\";\nimport bar from \"./module.wasm\";")?;
    assert_eq!(
        formatted,
        "import module foo from \"./module.wasm\";\nimport bar from \"./module.wasm\";"
    );
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_mjs_format_1_95a9edfd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import module from from \"./module.wasm\";")?;
    assert_eq!(formatted, "import module from from \"./module.wasm\";");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_typescript_format_1_d41d8cd9() -> Result<()>
{
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_valid_from_as_default_module_binding_escaped_mjs_format_1_0fe07d47() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import module \\u0066rom from \"./module.wasm\";")?;
    assert_eq!(formatted, "import module from from \"./module.wasm\";");
    Ok(())
}
#[test]
fn test_valid_module_as_default_binding_mjs_format_1_bc39ef68() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import module from \"./module.wasm\";")?;
    assert_eq!(formatted, "import module from \"./module.wasm\";");
    Ok(())
}
#[test]
fn test_valid_module_as_default_binding_2_mjs_format_1_e2adcd8f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("mjs")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import module, { createRequire } from \"node:module\";")?;
    assert_eq!(
        formatted,
        "import module, { createRequire } from \"node:module\";"
    );
    Ok(())
}
