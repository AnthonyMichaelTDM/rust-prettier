#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_bracket_spacingfalse_format_1_4993a647() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const arr1 = [1,2,3,4];\nconst arr2 = [1, 2, 3, 4];")?;
    assert_eq!(
        formatted,
        "const arr1 = [1, 2, 3, 4];\nconst arr2 = [1, 2, 3, 4];"
    );
    Ok(())
}
#[test]
fn test_array_js_format_1_4993a647() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const arr1 = [1,2,3,4];\nconst arr2 = [1, 2, 3, 4];")?;
    assert_eq!(
        formatted,
        "const arr1 = [1, 2, 3, 4];\nconst arr2 = [1, 2, 3, 4];"
    );
    Ok(())
}
#[test]
fn test_object_js_bracket_spacingfalse_format_1_b69240e7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("const obj1 = {a:1, b:2, c:3}\nconst obj2 = { a:1, b:2, c:3 };")?;
    assert_eq!(
        formatted,
        "const obj1 = {a: 1, b: 2, c: 3};\nconst obj2 = {a: 1, b: 2, c: 3};"
    );
    Ok(())
}
#[test]
fn test_object_js_format_1_b69240e7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("const obj1 = {a:1, b:2, c:3}\nconst obj2 = { a:1, b:2, c:3 };")?;
    assert_eq!(
        formatted,
        "const obj1 = { a: 1, b: 2, c: 3 };\nconst obj2 = { a: 1, b: 2, c: 3 };"
    );
    Ok(())
}
