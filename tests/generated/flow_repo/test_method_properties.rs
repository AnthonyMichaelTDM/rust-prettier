#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_exports_optional_prop_js_format_1_a5a77b77() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\ndeclare class Foo {\n  bar?: () => string\n}\n\nexport {Foo};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\ndeclare class Foo {\n  bar?: () => string;\n}\n\nexport { Foo };"
    );
}
#[test]
fn test_test_js_format_1_5177448d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  C() { }\n  foo() { }\n  static bar() { }\n  qux() { this.constructor.x; }\n}\nC.x;\n(new C).foo.x;\nC.bar.x;\n\nimport {Foo} from './exports_optional_prop';\nconst foo = new Foo();\n(foo.bar(): string); // error, could be undefined\n\nfunction f(x) {\n  (x.bar(): string); // error. caused by \\`f(foo)\\`; annotate x to track it down.\n}\nf(foo);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  C() {}\n  foo() {}\n  static bar() {}\n  qux() {\n    this.constructor.x;\n  }\n}\nC.x;\nnew C().foo.x;\nC.bar.x;\n\nimport { Foo } from \"./exports_optional_prop\";\nconst foo = new Foo();\n(foo.bar(): string); // error, could be undefined\n\nfunction f(x) {\n  (x.bar(): string); // error. caused by \\`f(foo)\\`; annotate x to track it down.\n}\nf(foo);");
}
