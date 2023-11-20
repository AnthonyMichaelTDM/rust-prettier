#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_method_default_parameters_js_format_1_a4b1abb1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n\n    b: string;\n\n    c(d = this.b) { // ok - can use \\`this\\` in function default parameter values\n\n    }\n\n    e() {\n        return this.b;\n    }\n\n    f(g = this.e()) { // ok - can use \\`this\\` in function default parameter values\n\n    }\n\n    h(i: number = this.b) { // error\n\n    }\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  b: string;\n\n  c(d = this.b) {\n    // ok - can use \\`this\\` in function default parameter values\n  }\n\n  e() {\n    return this.b;\n  }\n\n  f(g = this.e()) {\n    // ok - can use \\`this\\` in function default parameter values\n  }\n\n  h(i: number = this.b) {\n    // error\n  }\n}");
}
