#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_shadow_js_format_1_e430cb4f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("var o = {};\n(o.p: string);\n(o: $Shape<{p:string}>);")?;
    assert_eq!(
        formatted,
        "var o = {};\n(o.p: string);\n(o: $Shape<{ p: string }>);"
    );
    Ok(())
}
#[test]
fn test_test_js_format_1_291edaf6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  field: number,\n}\n\nvar x: {field?: number} = {};\nvar y: $Shape<Foo> = x;\n(y.field: number)") ? ;
    assert_eq ! (formatted , "type Foo = {\n  field: number,\n};\n\nvar x: { field?: number } = {};\nvar y: $Shape<Foo> = x;\n(y.field: number);");
    Ok(())
}
