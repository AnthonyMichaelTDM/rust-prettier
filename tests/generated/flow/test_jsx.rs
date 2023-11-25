#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_func_inside_attr_js_format_1_764423d8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<bar x={function (x): Array<string> {}} />")?;
    assert_eq!(formatted, "<bar x={function (x): Array<string> {}} />;");
    Ok(())
}
#[test]
fn test_generic_component_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_generic_component_js_format_1_8be6b91f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const c1 = <MyComponent<number> data={12} />\n\nconst c2 = <MyComponent<number> />\n\nconst c3 = <MyComponent<number> attr=\"value\" />") ? ;
    assert_eq ! (formatted , "const c1 = <MyComponent<number> data={12} />;\n\nconst c2 = <MyComponent<number> />;\n\nconst c3 = <MyComponent<number> attr=\"value\" />;");
    Ok(())
}
#[test]
fn test_return_type_js_format_1_61e982fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const fail = (): X => <x />;")?;
    assert_eq!(formatted, "const fail = (): X => <x />;");
    Ok(())
}
