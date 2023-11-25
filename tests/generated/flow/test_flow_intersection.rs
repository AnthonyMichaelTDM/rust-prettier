#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_intersection_js_format_1_0f160ee1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type State = {\n  sharedProperty: any;\n} & (\n  | { discriminant: \"FOO\"; foo: any }\n  | { discriminant: \"BAR\"; bar: any }\n  | { discriminant: \"BAZ\"; baz: any } \n);") ? ;
    assert_eq ! (formatted , "type State = {\n  sharedProperty: any,\n} & (\n  | { discriminant: \"FOO\", foo: any }\n  | { discriminant: \"BAR\", bar: any }\n  | { discriminant: \"BAZ\", baz: any }\n);");
    Ok(())
}
