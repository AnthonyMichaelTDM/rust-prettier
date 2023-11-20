#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_68d8d996() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {\n    foo(): A { return this; }\n}\nclass B extends A {\n    foo(): B { return this; }\n}\nclass C extends A {}\nvar a: A = new B();\na.foo = function(): C { return new C(); }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {\n  foo(): A {\n    return this;\n  }\n}\nclass B extends A {\n  foo(): B {\n    return this;\n  }\n}\nclass C extends A {}\nvar a: A = new B();\na.foo = function (): C {\n  return new C();\n};");
}
