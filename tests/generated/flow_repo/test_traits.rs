#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_4330cc62() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class Foo extends Qux<string> mixins Bar<number> {\n  // KeyedCollection <: Collection\n  // ...KeyedIterable\n}\ndeclare class Bar<T> extends Baz<T> {\n  // KeyedIterable <: Iterable\n  y: T\n}\ndeclare class Qux<T> extends Baz<T> {\n  // Collection <: Iterable\n  y: T, z: T\n}\ndeclare class Baz<T> {\n  // Iterable\n  x: T\n}\n\n((new Foo).x: number); // error: Qux wins\n((new Foo).y: string); // error: Bar wins\n((new Foo).z: number); // error: Qux wins") ? ;
    assert_eq ! (formatted , "declare class Foo extends Qux<string> mixins Bar<number> {\n  // KeyedCollection <: Collection\n  // ...KeyedIterable\n}\ndeclare class Bar<T> extends Baz<T> {\n  // KeyedIterable <: Iterable\n  y: T;\n}\ndeclare class Qux<T> extends Baz<T> {\n  // Collection <: Iterable\n  y: T;\n  z: T;\n}\ndeclare class Baz<T> {\n  // Iterable\n  x: T;\n}\n\n(new Foo().x: number); // error: Qux wins\n(new Foo().y: string); // error: Bar wins\n(new Foo().z: number); // error: Qux wins");
    Ok(())
}
#[test]
fn test_test_2_js_format_1_44fbba1f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare interface I { }\ndeclare class C mixins I { }")?;
    assert_eq!(
        formatted,
        "declare interface I {}\ndeclare class C mixins I {}"
    );
    Ok(())
}
