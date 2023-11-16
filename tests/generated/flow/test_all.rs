#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_generic_js_format_1_cd9ffbab() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("const Theme = React.createContext<\"light\" | \"dark\">(\"light\");");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "const Theme = React.createContext<\"light\" | \"dark\">(\"light\");"
    );
}
