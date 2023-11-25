#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_private_in_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_private_in_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_private_in_js_format_1_f877cf0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (#prop in obj) {\n}\n\n#prop in     obj;\n\n#prop      in obj;\n\n#prop in\nobj;\n\n#prop\nin\nobj;\n\n#prop\nin obj;") ? ;
    assert_eq ! (formatted , "if (#prop in obj) {\n}\n\n#prop in obj;\n\n#prop in obj;\n\n#prop in obj;\n\n#prop in obj;\n\n#prop in obj;");
    Ok(())
}
