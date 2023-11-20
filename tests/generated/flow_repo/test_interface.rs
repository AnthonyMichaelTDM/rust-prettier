#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_js_format_1_57864f18() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("interface I { x: number }\nexport type J = I; // workaround for export interface");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "interface I {\n  x: number;\n}\nexport type J = I; // workaround for export interface"
    );
}
#[test]
fn test_indexer_js_format_1_2ce27d66() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ninterface Ok {\n  [key: string]: string;\n}\n\ninterface Bad {\n  [k1: string]: string;\n  [k2: number]: number; // error: not supported (yet)\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ninterface Ok {\n  [key: string]: string;\n}\n\ninterface Bad {\n  [k1: string]: string;\n  [k2: number]: number; // error: not supported (yet)\n}");
}
#[test]
fn test_interface_js_format_1_9d95845a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class C { x: number; }\n\nvar x: string = new C().x;\n\ninterface I { x: number; }\n\nvar i = new I(); // error\n\nfunction testInterfaceName(o: I) {\n  (o.name: string); // error, name is static\n  (o.constructor.name: string); // ok\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare class C {\n  x: number;\n}\n\nvar x: string = new C().x;\n\ninterface I {\n  x: number;\n}\n\nvar i = new I(); // error\n\nfunction testInterfaceName(o: I) {\n  (o.name: string); // error, name is static\n  (o.constructor.name: string); // ok\n}");
}
#[test]
fn test_test_js_format_1_94c29b4b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface I { y: string }\ninterface I_ { x: number }\ninterface J extends I, I_ { }\ninterface K extends J { }\n\nvar k: K = { x: \"\", y: \"\" }; // error: x should be number\n(k.x: string); // error: x is number\n(k.y: string);\n\ndeclare class C { x: number }\n\ninterface A<Y> { y: Y }\ninterface A_<X> { x: X }\ninterface B<Z> extends A<string>, A_<Z> { z: Z }\ninterface E<Z> extends B<Z> { }\n\nvar e: E<number> = { x: \"\", y: \"\", z: \"\" }; // error: x and z should be numbers\n(e.x: string); // error: x is number\n(e.y: string);\n(e.z: string); // error: z is number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface I {\n  y: string;\n}\ninterface I_ {\n  x: number;\n}\ninterface J extends I, I_ {}\ninterface K extends J {}\n\nvar k: K = { x: \"\", y: \"\" }; // error: x should be number\n(k.x: string); // error: x is number\n(k.y: string);\n\ndeclare class C {\n  x: number;\n}\n\ninterface A<Y> {\n  y: Y;\n}\ninterface A_<X> {\n  x: X;\n}\ninterface B<Z> extends A<string>, A_<Z> {\n  z: Z;\n}\ninterface E<Z> extends B<Z> {}\n\nvar e: E<number> = { x: \"\", y: \"\", z: \"\" }; // error: x and z should be numbers\n(e.x: string); // error: x is number\n(e.y: string);\n(e.z: string); // error: z is number");
}
#[test]
fn test_test_2_js_format_1_f834a925() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import type { J } from './import';\ninterface K { }\ninterface L extends J, K { y: string }\n\nfunction foo(l: L) { l.x; l.y; l.z; } // error: z not found in L\n\n// interface + multiple inheritance is similar to object type + intersection\ntype M = { y: string } & J & { z: boolean }\n\nfunction bar(m: M) { m.x; m.y; m.z; } // OK") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import type { J } from \"./import\";\ninterface K {}\ninterface L extends J, K {\n  y: string;\n}\n\nfunction foo(l: L) {\n  l.x;\n  l.y;\n  l.z;\n} // error: z not found in L\n\n// interface + multiple inheritance is similar to object type + intersection\ntype M = { y: string } & J & { z: boolean };\n\nfunction bar(m: M) {\n  m.x;\n  m.y;\n  m.z;\n} // OK");
}
#[test]
fn test_test_3_js_format_1_79ba8ac6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface I { x: number, y : string }\ninterface J { y : number }\ninterface K extends I, J { x: string } // error: x is number in I\nfunction foo(k: K) {\n  (k.x: number); // error: x is string in K\n  (k.y: number); // error: y is string in I\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface I {\n  x: number;\n  y: string;\n}\ninterface J {\n  y: number;\n}\ninterface K extends I, J {\n  x: string;\n} // error: x is number in I\nfunction foo(k: K) {\n  (k.x: number); // error: x is string in K\n  (k.y: number); // error: y is string in I\n}");
}
#[test]
fn test_test_4_js_format_1_f00aba7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface I { foo(x: number): void; }\n(function foo(x: number) { }: I); // error, property \\`foo\\` not found function\n\ndeclare class C {\n  bar(i: I): void;\n  bar(f: (x: number) => void): void;\n}\n\nnew C().bar((x: string) => { }); // error, number ~/~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface I {\n  foo(x: number): void;\n}\n(function foo(x: number) {}: I); // error, property \\`foo\\` not found function\n\ndeclare class C {\n  bar(i: I): void;\n  bar(f: (x: number) => void): void;\n}\n\nnew C().bar((x: string) => {}); // error, number ~/~> string");
}
