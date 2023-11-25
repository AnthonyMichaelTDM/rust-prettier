#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_in_type_annotation_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_in_type_annotation_js_format_1_13b8853e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type A = () => infer R extends/* comment */ string");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "type A = () => infer R extends /* comment */ string;"
    );
}
#[test]
fn test_conditional_types_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_conditional_types_js_format_1_7bcca032() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nexport type DeepReadonly<T> = T extends any[] ? DeepReadonlyArray<T[number]> : T extends object ? DeepReadonlyObject<T> : T;\n\ninterface DeepReadonlyArray<T> extends ReadonlyArray<DeepReadonly<T>> {}\n\ntype TypeName<T> =\n  T extends string ? \"string\" :\n  T extends number ? \"number\" :\n  T extends boolean ? \"boolean\" :\n  T extends undefined ? \"undefined\" :\n  T extends Function ? \"function\" :\n  \"object\";\n\ntype Type01 = 0 extends (1 extends 2  ? 3 : 4) ? 5 : 6;\ntype Type02 = 0 extends ((1 extends 2  ? 3 : 4)) ? 5 : 6;\ntype Type03 = 0 extends (((1 extends 2  ? 3 : 4))) ? 5 : 6;\ntype Type04 = 0 extends ((((1 extends 2  ? 3 : 4)))) ? 5 : 6;\ntype Type05 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;\ntype Type06 = ((0 extends 1 ? 2 : 3)) extends 4 ? 5 : 6;\ntype Type07 = (((0 extends 1 ? 2 : 3))) extends 4 ? 5 : 6;\ntype Type08 = ((((0 extends 1 ? 2 : 3)))) extends 4 ? 5 : 6;\n\ntype T1 = () => void extends T ? U : V;\ntype T1a = () => (void extends T ? U : V);\ntype T1b = () => (void) extends T ? U : V;\ntype T2 = (() => void) extends T ? U : V;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nexport type DeepReadonly<T> = T extends any[]\n  ? DeepReadonlyArray<T[number]>\n  : T extends object\n    ? DeepReadonlyObject<T>\n    : T;\n\ninterface DeepReadonlyArray<T> extends ReadonlyArray<DeepReadonly<T>> {}\n\ntype TypeName<T> = T extends string\n  ? \"string\"\n  : T extends number\n    ? \"number\"\n    : T extends boolean\n      ? \"boolean\"\n      : T extends undefined\n        ? \"undefined\"\n        : T extends Function\n          ? \"function\"\n          : \"object\";\n\ntype Type01 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;\ntype Type02 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;\ntype Type03 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;\ntype Type04 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;\ntype Type05 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;\ntype Type06 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;\ntype Type07 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;\ntype Type08 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;\n\ntype T1 = () => void extends T ? U : V;\ntype T1a = () => void extends T ? U : V;\ntype T1b = () => void extends T ? U : V;\ntype T2 = (() => void) extends T ? U : V;");
}
#[test]
fn test_cursor_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_cursor_js_format_1_858e0ddd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .cursor_offset(31)
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("type A = () => infer R extends <|>string");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "type A = () => infer R extends <|>string;");
}
#[test]
fn test_infer_type_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_infer_type_js_format_1_4997590a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype TestReturnType<T extends (...args: any[]) => any> = T extends (...args: any[]) => infer R ? R : any;\n\ntype Unpacked<T> =\n  T extends (infer U)[] ? U :\n  T extends (...args: any[]) => infer U ? U :\n  T extends Promise<infer U> ? U :\n  T;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ntype TestReturnType<T extends (...args: any[]) => any> = T extends (\n  ...args: any[]\n) => infer R\n  ? R\n  : any;\n\ntype Unpacked<T> = T extends (infer U)[]\n  ? U\n  : T extends (...args: any[]) => infer U\n    ? U\n    : T extends Promise<infer U>\n      ? U\n      : T;");
}
#[test]
fn test_nested_in_condition_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_nested_in_condition_js_format_1_c59e250f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype Foo =\n  (ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider) extends\n  DoubleGobbledygookProvider\n    ? UniqueDalgametreService\n    : CompositeZamazingoResolver;\n\ntype Foo2 =\n  DoubleGobbledygookProvider extends\n  (ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider)\n    ? UniqueDalgametreService\n    : CompositeZamazingoResolver;\n\ntype Foo3 =\n  (ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider) extends\n  (DoubleGobbledygookProvider extends MockGobbledygookProvider ? MockThingamabobberFactory : ThingamabobberFactory)\n    ? UniqueDalgametreService\n    : CompositeZamazingoResolver;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ntype Foo = (\n  ThingamabobberFactory extends AbstractThingamabobberFactory\n    ? GobbledygookProvider\n    : CompositeGobbledygookProvider\n) extends DoubleGobbledygookProvider\n  ? UniqueDalgametreService\n  : CompositeZamazingoResolver;\n\ntype Foo2 = DoubleGobbledygookProvider extends (\n  ThingamabobberFactory extends AbstractThingamabobberFactory\n    ? GobbledygookProvider\n    : CompositeGobbledygookProvider\n)\n  ? UniqueDalgametreService\n  : CompositeZamazingoResolver;\n\ntype Foo3 = (\n  ThingamabobberFactory extends AbstractThingamabobberFactory\n    ? GobbledygookProvider\n    : CompositeGobbledygookProvider\n) extends (\n  DoubleGobbledygookProvider extends MockGobbledygookProvider\n    ? MockThingamabobberFactory\n    : ThingamabobberFactory\n)\n  ? UniqueDalgametreService\n  : CompositeZamazingoResolver;");
}
#[test]
fn test_new_ternary_spec_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_new_ternary_spec_js_format_1_68b6ed64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// TypeScript examples:\ntype TypeName<T> =\n  T extends string ? \"string\"\n  : T extends number ? \"number\"\n  : T extends boolean ? \"boolean\"\n  : T extends undefined ? \"undefined\"\n  : T extends Function ? \"function\"\n  : \"object\";\n\ntype Unpacked<T> =\n  T extends (infer U)[] ? U\n  : T extends (...args: any[]) => infer U ?\n    SomeReallyLongThingThatBreaksTheLine<U>\n  : T extends Promise<infer U> ? U\n  : T;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// TypeScript examples:\ntype TypeName<T> = T extends string\n  ? \"string\"\n  : T extends number\n    ? \"number\"\n    : T extends boolean\n      ? \"boolean\"\n      : T extends undefined\n        ? \"undefined\"\n        : T extends Function\n          ? \"function\"\n          : \"object\";\n\ntype Unpacked<T> = T extends (infer U)[]\n  ? U\n  : T extends (...args: any[]) => infer U\n    ? SomeReallyLongThingThatBreaksTheLine<U>\n    : T extends Promise<infer U>\n      ? U\n      : T;");
}
#[test]
fn test_parentheses_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_parentheses_js_format_1_f009c109() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// #13275\ntype Foo<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// #13275\ntype Foo<T> = T extends ((...a: any[]) => infer R extends string) ? R : never;");
}
