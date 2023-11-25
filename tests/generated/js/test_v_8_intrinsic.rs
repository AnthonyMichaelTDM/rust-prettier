#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_avoid_conflicts_to_pipeline_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_avoid_conflicts_to_pipeline_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_avoid_conflicts_to_pipeline_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_avoid_conflicts_to_pipeline_js_format_1_3b462a77() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// |>\nconst status = %GetOptimizationStatus(fn);")?;
    assert_eq!(
        formatted,
        "// |>\nconst status = %GetOptimizationStatus(fn);"
    );
    Ok(())
}
#[test]
fn test_intrinsic_call_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_intrinsic_call_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_intrinsic_call_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_intrinsic_call_js_format_1_5664d48e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function doSmth()     {\n            %DebugPrint\n        (\n                foo )\n  }\n\n    function printFunc  (\n        f\n) {\n    if(%\n    IsAsmWasmCode(f))              console.log(\"asm.js\");\n        if(\n\n        % IsWasmCode(\n        f))\n            console.log (\n                \"wasm\"\n            );\n\n    console.log\n    (%\n        GetFunctioName(f)\n        );\n}") ? ;
    assert_eq ! (formatted , "function doSmth() {\n  %DebugPrint(foo);\n}\n\nfunction printFunc(f) {\n  if (%IsAsmWasmCode(f)) console.log(\"asm.js\");\n  if (%IsWasmCode(f)) console.log(\"wasm\");\n\n  console.log(%GetFunctioName(f));\n}");
    Ok(())
}
