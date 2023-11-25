#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_98e3a322() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function foo(x: number): string;\ndeclare function foo(x: string): number;\ndeclare function foo<X>(x: X): X;\n\n(foo(0): string); // OK\n(foo(\"hello\"): number); // OK\n(foo(false): void); // error, boolean ~/~ undefined") ? ;
    assert_eq ! (formatted , "declare function foo(x: number): string;\ndeclare function foo(x: string): number;\ndeclare function foo<X>(x: X): X;\n\n(foo(0): string); // OK\n(foo(\"hello\"): number); // OK\n(foo(false): void); // error, boolean ~/~ undefined");
    Ok(())
}
