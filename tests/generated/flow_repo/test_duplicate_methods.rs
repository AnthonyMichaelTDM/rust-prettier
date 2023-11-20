#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_91bb58f3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C1 {\n  m() { }\n  m() { }\n}\n\nnew C1().m();\n\nclass C2 {\n  get m() { return 42; }\n  m() { }\n}\n\nnew C2().m();\n\nclass C3 {\n  set m(x) { }\n  m() { }\n}\n\nnew C3().m();\n\nclass C4 {\n  get m() { return 42; }\n  set m(x: number) { }\n}\n\nnew C4().m = new C4().m - 42;\n\nclass C5 {\n  m() { }\n  get m() { return 42; }\n  set m(x: number) { }\n}\n\nnew C5().m = new C5().m - 42;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C1 {\n  m() {}\n  m() {}\n}\n\nnew C1().m();\n\nclass C2 {\n  get m() {\n    return 42;\n  }\n  m() {}\n}\n\nnew C2().m();\n\nclass C3 {\n  set m(x) {}\n  m() {}\n}\n\nnew C3().m();\n\nclass C4 {\n  get m() {\n    return 42;\n  }\n  set m(x: number) {}\n}\n\nnew C4().m = new C4().m - 42;\n\nclass C5 {\n  m() {}\n  get m() {\n    return 42;\n  }\n  set m(x: number) {}\n}\n\nnew C5().m = new C5().m - 42;");
}
