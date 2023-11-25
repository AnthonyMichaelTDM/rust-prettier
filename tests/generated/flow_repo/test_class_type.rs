use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_59cd5917() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A { }\nfunction foo(x: Class<A>): A {\n  return new x(); // OK\n}\n\nclass B {\n  constructor(_: any) { }\n}\nfunction bar(x: Class<B>): B {\n  return new x(); // error (too few args)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class A {}\nfunction foo(x: Class<A>): A {\n  return new x(); // OK\n}\n\nclass B {\n  constructor(_: any) {}\n}\nfunction bar(x: Class<B>): B {\n  return new x(); // error (too few args)\n}");
}
#[test]
fn test_test_2_js_format_1_6c108b9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// A function to typecheck values against their types. Covariance of Class<.>\n// makes it useless in such a function (when limited to classes and instances),\n// since everything can be trivially satisfied by going to \\`mixed\\`.\ndeclare function check<X>(cls: $Type<X>, inst: X): void;\n\nclass A { }\nclass B extends A { }\nclass C { }\n\ncheck(B, new A);\ncheck(A, new B);\ncheck(C, new A);\ncheck(C, new B);\ncheck(B, new C);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// A function to typecheck values against their types. Covariance of Class<.>\n// makes it useless in such a function (when limited to classes and instances),\n// since everything can be trivially satisfied by going to \\`mixed\\`.\ndeclare function check<X>(cls: $Type<X>, inst: X): void;\n\nclass A {}\nclass B extends A {}\nclass C {}\n\ncheck(B, new A());\ncheck(A, new B());\ncheck(C, new A());\ncheck(C, new B());\ncheck(B, new C());");
}
