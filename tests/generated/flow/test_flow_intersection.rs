#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_intersection_js_format_1_0f160ee1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type State = {\n  sharedProperty: any;\n} & (\n  | { discriminant: \"FOO\"; foo: any }\n  | { discriminant: \"BAR\"; bar: any }\n  | { discriminant: \"BAZ\"; baz: any } \n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type State = {\n  sharedProperty: any,\n} & (\n  | { discriminant: \"FOO\", foo: any }\n  | { discriminant: \"BAR\", bar: any }\n  | { discriminant: \"BAZ\", baz: any }\n);");
}
