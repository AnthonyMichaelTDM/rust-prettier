use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_case_css_format_1_595cf37e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@custom-media --KeepName (MIN-WIDTH: 500PX);\n\n.foo {\n    color: hsl(0.75TURN, 60%, 70%);\n}\n\np:FIRST-CHILD {\n    color: lime;\n    background-color: black;\n    padding: 5px;\n}\n\na::AFTER {\n    content: \"→\";\n}\n\na:AFTER {\n    content: \"→\";\n}\n\n::-WEBKIT-PROGRESS-BAR {\n    background-color: orange;\n}\n\nTABLE {}\n\n/* apply a dashed border to all unresolved elements */\n:unresolved {\n    border: 1px dashed red;\n    display: inline-block;\n}\n\n/* x-panel's that are unresolved are red */\nx-panel:unresolved {\n    color: red;\n}\n\n/* once the definition of x-panel is registered, it becomes green */\nx-panel {\n    color: green;\n    display: block;\n    padding: 5px;\n    display: block;\n}\n\n:host {\n    all: initial;\n    display: block;\n    contain: content;\n    text-align: center;\n    background: linear-gradient(to left, hotpink, transparent);\n    max-width: 500px;\n    margin: 0 auto;\n    border-radius: 8px;\n    transition: transform .2s ease-out;\n}\n:host([hidden]) {\n    display: none;\n}\n:host(:hover) {\n    transform: scale(1.1);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@custom-media --KeepName (min-width: 500px);\n\n.foo {\n  color: hsl(0.75turn, 60%, 70%);\n}\n\np:first-child {\n  color: lime;\n  background-color: black;\n  padding: 5px;\n}\n\na::after {\n  content: \"→\";\n}\n\na:after {\n  content: \"→\";\n}\n\n::-webkit-progress-bar {\n  background-color: orange;\n}\n\nTABLE {\n}\n\n/* apply a dashed border to all unresolved elements */\n:unresolved {\n  border: 1px dashed red;\n  display: inline-block;\n}\n\n/* x-panel's that are unresolved are red */\nx-panel:unresolved {\n  color: red;\n}\n\n/* once the definition of x-panel is registered, it becomes green */\nx-panel {\n  color: green;\n  display: block;\n  padding: 5px;\n  display: block;\n}\n\n:host {\n  all: initial;\n  display: block;\n  contain: content;\n  text-align: center;\n  background: linear-gradient(to left, hotpink, transparent);\n  max-width: 500px;\n  margin: 0 auto;\n  border-radius: 8px;\n  transition: transform 0.2s ease-out;\n}\n:host([hidden]) {\n  display: none;\n}\n:host(:hover) {\n  transform: scale(1.1);\n}");
}
#[test]
fn test_custom_selectors_css_format_1_70298188() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "@custom-selector :--camelCase .my-css-selector;\n\n:--camelCase {\n  content: red;\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@custom-selector :--camelCase .my-css-selector;\n\n:--camelCase {\n  content: red;\n}"
    );
}
