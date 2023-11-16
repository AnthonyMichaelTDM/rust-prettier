#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_a_js_format_1_2aeac91a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./C');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./C\");");
}
#[test]
fn test_b_js_format_1_2aeac91a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./C');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./C\");");
}
#[test]
fn test_c_js_format_1_4fd47685() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("require('./D');\nrequire('./E');\nrequire('./F');\nrequire('./G');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "require(\"./D\");\nrequire(\"./E\");\nrequire(\"./F\");\nrequire(\"./G\");"
    );
}
#[test]
fn test_d_js_format_1_5db832ba() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./I');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./I\");");
}
#[test]
fn test_e_js_format_1_5db832ba() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./I');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./I\");");
}
#[test]
fn test_f_js_format_1_00d979fa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// empty");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// empty");
}
#[test]
fn test_g_js_format_1_6b7709ba() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./H');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./H\");");
}
#[test]
fn test_h_js_format_1_00d979fa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// empty");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// empty");
}
#[test]
fn test_i_js_format_1_3b5f1e3e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("require('./A');");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "require(\"./A\");");
}
