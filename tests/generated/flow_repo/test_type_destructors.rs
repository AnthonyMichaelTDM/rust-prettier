#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_non_maybe_type_js_format_1_0cd50afd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x: ?string): $NonMaybeType<?string> {\n  if (x != null) { return x; }\n  else return 0; // this should be an error\n}\n\n//(foo(): string); // should not be necessary to expose the error above\n\n(0: $NonMaybeType<null>); // error\n(0: $NonMaybeType<?number>); // ok\n(0: $NonMaybeType<number | null>); // ok") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x: ?string): $NonMaybeType<?string> {\n  if (x != null) {\n    return x;\n  } else return 0; // this should be an error\n}\n\n//(foo(): string); // should not be necessary to expose the error above\n\n(0: $NonMaybeType<null>); // error\n(0: $NonMaybeType<?number>); // ok\n(0: $NonMaybeType<number | null>); // ok");
}
#[test]
fn test_property_type_js_format_1_2884a6fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Malformed = $PropertyType<any, number>;\n\ntype Obj = { x: string };\ntype Obj_Prop_x = $PropertyType<Obj, 'x'>;\n\n(42: Obj_Prop_x);\n\nfunction foo(o: Obj): $PropertyType<Obj, 'x'> {\n  if (false) return o.x;\n  else return 0;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Malformed = $PropertyType<any, number>;\n\ntype Obj = { x: string };\ntype Obj_Prop_x = $PropertyType<Obj, \"x\">;\n\n(42: Obj_Prop_x);\n\nfunction foo(o: Obj): $PropertyType<Obj, \"x\"> {\n  if (false) return o.x;\n  else return 0;\n}");
}
#[test]
fn test_union_js_format_1_399a5aa9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x0: $NonMaybeType<number|string> = 0; // ok, number ~> number|string\nvar x1: $NonMaybeType<number|string> = true; // err, boolean ~> number|string\nvar x2: $PropertyType<{p:number}|{p:string},'p'> = 0; // ok, number ~> number|string\nvar x3: $PropertyType<{p:number}|{p:string},'p'> = true; // err, boolean ~> number|string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var x0: $NonMaybeType<number | string> = 0; // ok, number ~> number|string\nvar x1: $NonMaybeType<number | string> = true; // err, boolean ~> number|string\nvar x2: $PropertyType<{ p: number } | { p: string }, \"p\"> = 0; // ok, number ~> number|string\nvar x3: $PropertyType<{ p: number } | { p: string }, \"p\"> = true; // err, boolean ~> number|string");
}
