#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_js_format_1_864f3760() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var a: number;\nvar b: typeof a = \"...\";\nvar c: typeof a = \"...\";\n\ntype T = number;\nvar x:T = \"...\";\n\n// what about recursive unions?") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare var a: number;\nvar b: typeof a = \"...\";\nvar c: typeof a = \"...\";\n\ntype T = number;\nvar x: T = \"...\";\n\n// what about recursive unions?");
}
