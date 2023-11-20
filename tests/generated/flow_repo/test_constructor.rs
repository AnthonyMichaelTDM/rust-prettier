#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_constructor_js_format_1_c5d549d8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n    constructor() { }\n}\n\nclass D {\n    constructor():number { }\n}\n\n// the return type of a constructor overrides the type of the class\ndeclare class Bar<T> {}\ndeclare class Foo<T> {\n  constructor<U>(iterable: U): Bar<U>;\n}\n(new Foo('x'): Bar<string>); // ok\n(new Foo(123): Bar<string>); // error, number !~> string\n\n// also overrides when it returns a different specialization of the same class\ndeclare class Baz<T> {\n  constructor<U>(iterable: U): Baz<U>;\n}\n(new Baz('x'): Baz<string>); // ok\n(new Baz(123): Baz<string>); // error, number !~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  constructor() {}\n}\n\nclass D {\n  constructor(): number {}\n}\n\n// the return type of a constructor overrides the type of the class\ndeclare class Bar<T> {}\ndeclare class Foo<T> {\n  constructor<U>(iterable: U): Bar<U>;\n}\n(new Foo(\"x\"): Bar<string>); // ok\n(new Foo(123): Bar<string>); // error, number !~> string\n\n// also overrides when it returns a different specialization of the same class\ndeclare class Baz<T> {\n  constructor<U>(iterable: U): Baz<U>;\n}\n(new Baz(\"x\"): Baz<string>); // ok\n(new Baz(123): Baz<string>); // error, number !~> string");
}
