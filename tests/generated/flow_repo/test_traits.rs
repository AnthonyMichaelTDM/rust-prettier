#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_4330cc62() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class Foo extends Qux<string> mixins Bar<number> {\n  // KeyedCollection <: Collection\n  // ...KeyedIterable\n}\ndeclare class Bar<T> extends Baz<T> {\n  // KeyedIterable <: Iterable\n  y: T\n}\ndeclare class Qux<T> extends Baz<T> {\n  // Collection <: Iterable\n  y: T, z: T\n}\ndeclare class Baz<T> {\n  // Iterable\n  x: T\n}\n\n((new Foo).x: number); // error: Qux wins\n((new Foo).y: string); // error: Bar wins\n((new Foo).z: number); // error: Qux wins") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class Foo extends Qux<string> mixins Bar<number> {\n  // KeyedCollection <: Collection\n  // ...KeyedIterable\n}\ndeclare class Bar<T> extends Baz<T> {\n  // KeyedIterable <: Iterable\n  y: T;\n}\ndeclare class Qux<T> extends Baz<T> {\n  // Collection <: Iterable\n  y: T;\n  z: T;\n}\ndeclare class Baz<T> {\n  // Iterable\n  x: T;\n}\n\n(new Foo().x: number); // error: Qux wins\n(new Foo().y: string); // error: Bar wins\n(new Foo().z: number); // error: Qux wins");
}
#[test]
fn test_test_2_js_format_1_44fbba1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("declare interface I { }\ndeclare class C mixins I { }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare interface I {}\ndeclare class C mixins I {}"
    );
}
