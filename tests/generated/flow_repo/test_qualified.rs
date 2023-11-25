#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_qualified_js_format_1_b0cb55b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C { }\nvar M = { C: C };\n\nvar x:M.C = 0;\n\ntype foo = {bar: number};\n\ndeclare var of_type_foo: foo;\ntype bar = typeof of_type_foo.bar;\n\nvar a: bar = 42;\nvar b: bar = 'asdf'; // Error: string ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {}\nvar M = { C: C };\n\nvar x: M.C = 0;\n\ntype foo = { bar: number };\n\ndeclare var of_type_foo: foo;\ntype bar = typeof of_type_foo.bar;\n\nvar a: bar = 42;\nvar b: bar = \"asdf\"; // Error: string ~> number");
}
