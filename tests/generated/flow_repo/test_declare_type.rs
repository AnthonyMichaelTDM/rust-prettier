use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_declare_type_js_format_1_ac1fb6f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n////////////////////////////////////////////////////////////\n// == Import Declared Type Alias From Declared Module == //\n//////////////////////////////////////////////////////////\nimport type {baz} from \"ModuleAliasFoo\";\nimport {foo} from \"ModuleAliasFoo\";\nvar k1: baz = 42;\nvar k2: baz = \"shab\"; // Error: string to int\nvar k3: toz = foo(k1); // works\n\nimport type {toz} from \"ModuleAliasFoo\";\nvar k4: toz = foo(k1); // works\n\n//////////////////////////////////////////////////////////\n// == Declared Module with exports prop (issue 880) == //\n////////////////////////////////////////////////////////\n\nimport blah from 'foo';\nimport type { Foo, Bar, Id } from 'foo';\n\nblah(0, 0);\n\n({toz:3} : Foo); // error : {toz : number} ~> string\n\n(3 : Bar); // error : number ~> A\n\n(\"lol\" : Id<number>); // error : string ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n////////////////////////////////////////////////////////////\n// == Import Declared Type Alias From Declared Module == //\n//////////////////////////////////////////////////////////\nimport type { baz } from \"ModuleAliasFoo\";\nimport { foo } from \"ModuleAliasFoo\";\nvar k1: baz = 42;\nvar k2: baz = \"shab\"; // Error: string to int\nvar k3: toz = foo(k1); // works\n\nimport type { toz } from \"ModuleAliasFoo\";\nvar k4: toz = foo(k1); // works\n\n//////////////////////////////////////////////////////////\n// == Declared Module with exports prop (issue 880) == //\n////////////////////////////////////////////////////////\n\nimport blah from \"foo\";\nimport type { Foo, Bar, Id } from \"foo\";\n\nblah(0, 0);\n\n({ toz: 3 }: Foo); // error : {toz : number} ~> string\n\n(3: Bar); // error : number ~> A\n\n(\"lol\": Id<number>); // error : string ~> number");
}
