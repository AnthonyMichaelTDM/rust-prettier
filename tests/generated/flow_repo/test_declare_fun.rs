#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_98e3a322() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("declare function foo(x: number): string;\ndeclare function foo(x: string): number;\ndeclare function foo<X>(x: X): X;\n\n(foo(0): string); // OK\n(foo(\"hello\"): number); // OK\n(foo(false): void); // error, boolean ~/~ undefined") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function foo(x: number): string;\ndeclare function foo(x: string): number;\ndeclare function foo<X>(x: X): X;\n\n(foo(0): string); // OK\n(foo(\"hello\"): number); // OK\n(foo(false): void); // error, boolean ~/~ undefined");
}
