#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_func_inside_attr_js_format_1_764423d8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("<bar x={function (x): Array<string> {}} />");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<bar x={function (x): Array<string> {}} />;");
}
#[test]
fn test_generic_component_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_generic_component_js_format_1_8be6b91f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const c1 = <MyComponent<number> data={12} />\n\nconst c2 = <MyComponent<number> />\n\nconst c3 = <MyComponent<number> attr=\"value\" />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const c1 = <MyComponent<number> data={12} />;\n\nconst c2 = <MyComponent<number> />;\n\nconst c3 = <MyComponent<number> attr=\"value\" />;");
}
#[test]
fn test_return_type_js_format_1_61e982fa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("const fail = (): X => <x />;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const fail = (): X => <x />;");
}
