#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_format_1_8bebd345() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class A {}\n\nvar test1 = A.bar; // Error bar doesn't exist\nvar test2: string = A.name;\nvar test3: number = A.name; // Error string ~> number\n\nvar a = new A();\nvar test4 = a.constructor.bar; // Error bar doesn't exist\nvar test5: string = a.constructor.name;\nvar test6: number = a.constructor.name; // Error string ~> number") ? ;
    assert_eq ! (formatted , "class A {}\n\nvar test1 = A.bar; // Error bar doesn't exist\nvar test2: string = A.name;\nvar test3: number = A.name; // Error string ~> number\n\nvar a = new A();\nvar test4 = a.constructor.bar; // Error bar doesn't exist\nvar test5: string = a.constructor.name;\nvar test6: number = a.constructor.name; // Error string ~> number");
    Ok(())
}
#[test]
fn test_function_js_format_1_f74c6cc5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* TODO - we currently say that a function's statics are an AnyObjT and\n * anything goes. When we start enforcing the statics properly, we'll need to\n * know that .name exists\n */") ? ;
    assert_eq ! (formatted , "/* TODO - we currently say that a function's statics are an AnyObjT and\n * anything goes. When we start enforcing the statics properly, we'll need to\n * know that .name exists\n */");
    Ok(())
}
