use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_74b626d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @noflow */\n\ninterface IFoo { foo: string }\n\nclass C1 implements IFoo {} // error: property \\`foo\\` not found\nclass C2 implements IFoo { foo: number } // error: number <~> string\nclass C3 implements IFoo { foo: string } // ok\n\n(new C1: IFoo); // ok, we already errored at def site\n\ninterface IBar { bar: number }\n\nclass C4 implements IFoo, IBar {} // error: properties \\`foo\\`, \\`bar\\` not found\n(new C4: IBar); // ok, we already errored at def site\n\ninterface IFooBar extends IFoo { bar: number }\n\nclass C5 implements IFooBar {} // error: properties \\`foo\\`, \\`bar\\` not found\n(new C5: IFooBar); // ok, already errored at def site\n(new C5: IFoo); // ok, already errored at def site\n(new C5: IBar); // error: property \\`bar\\` not found (despite IBar < IFooBar)\n\nclass C6 extends C1 {}\n(new C6: IFoo); // ok, C1 implements IFoo\n\nclass C7 implements C1 {} // error: C1 is a class, expected an interface\n\n// ensure BoundT substituted appropriately\ninterface IPoly<T> { x: T }\nclass C8<T> implements IPoly<T> { x: T }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @noflow */\n\ninterface IFoo {\n  foo: string;\n}\n\nclass C1 implements IFoo {} // error: property \\`foo\\` not found\nclass C2 implements IFoo {\n  foo: number;\n} // error: number <~> string\nclass C3 implements IFoo {\n  foo: string;\n} // ok\n\n(new C1(): IFoo); // ok, we already errored at def site\n\ninterface IBar {\n  bar: number;\n}\n\nclass C4 implements IFoo, IBar {} // error: properties \\`foo\\`, \\`bar\\` not found\n(new C4(): IBar); // ok, we already errored at def site\n\ninterface IFooBar extends IFoo {\n  bar: number;\n}\n\nclass C5 implements IFooBar {} // error: properties \\`foo\\`, \\`bar\\` not found\n(new C5(): IFooBar); // ok, already errored at def site\n(new C5(): IFoo); // ok, already errored at def site\n(new C5(): IBar); // error: property \\`bar\\` not found (despite IBar < IFooBar)\n\nclass C6 extends C1 {}\n(new C6(): IFoo); // ok, C1 implements IFoo\n\nclass C7 implements C1 {} // error: C1 is a class, expected an interface\n\n// ensure BoundT substituted appropriately\ninterface IPoly<T> {\n  x: T;\n}\nclass C8<T> implements IPoly<T> {\n  x: T;\n}");
}
