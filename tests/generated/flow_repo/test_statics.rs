#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_class_statics_js_format_1_41f99419() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("class C {\n  static f(x:number) { }\n  static x:string;\n}\n\nC.g = function(x:string) { C.f(x); };\nC.g(0);\n\nvar x:number = C.x;\nC.x = 0") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  static f(x: number) {}\n  static x: string;\n}\n\nC.g = function (x: string) {\n  C.f(x);\n};\nC.g(0);\n\nvar x: number = C.x;\nC.x = 0;");
}
#[test]
fn test_funstatics_js_format_1_155e42ec() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function C() { }\nC.prototype.f = function() { return C.g(0); }\nC.g = function(x) { return x; };\n\nvar x:string = new C().f();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function C() {}\nC.prototype.f = function () {\n  return C.g(0);\n};\nC.g = function (x) {\n  return x;\n};\n\nvar x: string = new C().f();");
}
