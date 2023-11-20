#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_any_js_format_1_957d7eb9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x:any):any { return x; }\nfunction bar(x:any):mixed { return x; }\nfunction qux(x:mixed):any { return x; }\n\nvar x:string = foo(0);\nvar y:string = bar(0);\nvar z:string = qux(0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x: any): any {\n  return x;\n}\nfunction bar(x: any): mixed {\n  return x;\n}\nfunction qux(x: mixed): any {\n  return x;\n}\n\nvar x: string = foo(0);\nvar y: string = bar(0);\nvar z: string = qux(0);");
}
#[test]
fn test_anyexportflowfile_js_format_1_95c02f6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nmodule.exports = ((x: any) => x: any);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nmodule.exports = ((x: any) => x: any);"
    );
}
#[test]
fn test_flowfixme_js_format_1_b07c796d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n  FlowFixMe is a synonym for any, used by the Flow team to\n  signal a needed mod to JS devs.\n\n  @flow\n */\n\n// no param\nfunction foo(x:$FlowFixMe):$FlowFixMe { return x; }\nfunction bar(x:$FlowFixMe):mixed { return x; }\n// param (info only)\nfunction qux(x:$FlowFixMe<number>):$FlowFixMe<number> { return x; }\n// ...params are still checked. unknown type\nfunction baz(x:$FlowFixMe<nnumber>): $FlowFixMe<number> { return x; }\n\nvar x:string = foo(0);\nvar y:string = bar(0);\nvar z:string = qux(0);\nvar w:string = baz(0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n  FlowFixMe is a synonym for any, used by the Flow team to\n  signal a needed mod to JS devs.\n\n  @flow\n */\n\n// no param\nfunction foo(x: $FlowFixMe): $FlowFixMe {\n  return x;\n}\nfunction bar(x: $FlowFixMe): mixed {\n  return x;\n}\n// param (info only)\nfunction qux(x: $FlowFixMe<number>): $FlowFixMe<number> {\n  return x;\n}\n// ...params are still checked. unknown type\nfunction baz(x: $FlowFixMe<nnumber>): $FlowFixMe<number> {\n  return x;\n}\n\nvar x: string = foo(0);\nvar y: string = bar(0);\nvar z: string = qux(0);\nvar w: string = baz(0);");
}
#[test]
fn test_flowissue_js_format_1_96c1de6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n  $FlowIssue is a synonym for any, used by JS devs to signal\n  a potential typechecker bug to the Flow team.\n\n  @flow\n */\n\n// no param\nfunction foo(x:$FlowIssue):$FlowIssue { return x; }\nfunction bar(x:$FlowIssue):mixed { return x; }\n// param (info only)\nfunction qux(x:$FlowIssue<number>):$FlowIssue<number> { return x; }\n// ...params are still checked. unknown type\nfunction baz(x:$FlowIssue<nnumber>): $FlowIssue<number> { return x; }\n\nvar x:string = foo(0);\nvar y:string = bar(0);\nvar z:string = qux(0);\nvar w:string = baz(0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n  $FlowIssue is a synonym for any, used by JS devs to signal\n  a potential typechecker bug to the Flow team.\n\n  @flow\n */\n\n// no param\nfunction foo(x: $FlowIssue): $FlowIssue {\n  return x;\n}\nfunction bar(x: $FlowIssue): mixed {\n  return x;\n}\n// param (info only)\nfunction qux(x: $FlowIssue<number>): $FlowIssue<number> {\n  return x;\n}\n// ...params are still checked. unknown type\nfunction baz(x: $FlowIssue<nnumber>): $FlowIssue<number> {\n  return x;\n}\n\nvar x: string = foo(0);\nvar y: string = bar(0);\nvar z: string = qux(0);\nvar w: string = baz(0);");
}
#[test]
fn test_nonflowfile_js_format_1_9501635f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @noflow\n\nmodule.exports = (x) => x;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// @noflow\n\nmodule.exports = (x) => x;");
}
#[test]
fn test_propagate_js_format_1_a57f9eae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ndeclare class C {\n  bar(n1: number, n2: number): number;\n  bar(s1: string, s2: string): string;\n}\n\nfunction foo(c: C, x: any): string {\n  let y = x.y;\n  return c.bar(0, y); // should be able to select first case and error\n}\n\nvar any_fun1 = require('./nonflowfile');\nfunction bar1(x: mixed) {\n  if (any_fun1(x)) {\n    (x: boolean);\n  }\n}\n\nvar any_fun2 = require('./anyexportflowfile');\nfunction bar2(x: mixed) {\n  if (any_fun2(x)) {\n    (x: boolean);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ndeclare class C {\n  bar(n1: number, n2: number): number;\n  bar(s1: string, s2: string): string;\n}\n\nfunction foo(c: C, x: any): string {\n  let y = x.y;\n  return c.bar(0, y); // should be able to select first case and error\n}\n\nvar any_fun1 = require(\"./nonflowfile\");\nfunction bar1(x: mixed) {\n  if (any_fun1(x)) {\n    (x: boolean);\n  }\n}\n\nvar any_fun2 = require(\"./anyexportflowfile\");\nfunction bar2(x: mixed) {\n  if (any_fun2(x)) {\n    (x: boolean);\n  }\n}");
}
#[test]
fn test_reach_js_format_1_c31c2438() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * like class and function values, any-typed values may be used in\n * type annotations. Here we test propagation of any through the\n * annotation - without it, the body of the if will be unreachable\n */\n\ntype AsyncRequest = any;\n\nfunction foo(o: ?AsyncRequest) {\n  if (o) {\n    var n: number = o;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * like class and function values, any-typed values may be used in\n * type annotations. Here we test propagation of any through the\n * annotation - without it, the body of the if will be unreachable\n */\n\ntype AsyncRequest = any;\n\nfunction foo(o: ?AsyncRequest) {\n  if (o) {\n    var n: number = o;\n  }\n}");
}
