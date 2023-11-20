#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_js_format_1_6885ce6a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Foo = {\n  method(\n    arg: number, // I belong with baz\n    qux: string\n  ) : void\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Foo = {\n  method(\n    arg: number, // I belong with baz\n    qux: string,\n  ): void,\n};");
}
#[test]
fn test_consistent_breaking_js_format_1_605ad57b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo1 {\n  flowParseFunctionTypeParams(\n    params: N.FlowFunctionTypeParam[] = [],\n  ): { params: N.FlowFunctionTypeParam[], rest: ?N.FlowFunctionTypeParam } {\n    // ...\n  }\n}\n\ntype Foo2 = {\n  flowParseFunctionTypeParams(\n    params: N.FlowFunctionTypeParam[]\n  ): { params: N.FlowFunctionTypeParam[], rest: ?N.FlowFunctionTypeParam }\n}\n\n{\n  function flowParseFunctionTypeParams(\n    params: N.FlowFunctionTypeParam[]\n  ): { params: N.FlowFunctionTypeParam[], rest: ?N.FlowFunctionTypeParam } {\n    // ...\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo1 {\n  flowParseFunctionTypeParams(params: N.FlowFunctionTypeParam[] = []): {\n    params: N.FlowFunctionTypeParam[],\n    rest: ?N.FlowFunctionTypeParam,\n  } {\n    // ...\n  }\n}\n\ntype Foo2 = {\n  flowParseFunctionTypeParams(params: N.FlowFunctionTypeParam[]): {\n    params: N.FlowFunctionTypeParam[],\n    rest: ?N.FlowFunctionTypeParam,\n  },\n};\n\n{\n  function flowParseFunctionTypeParams(params: N.FlowFunctionTypeParam[]): {\n    params: N.FlowFunctionTypeParam[],\n    rest: ?N.FlowFunctionTypeParam,\n  } {\n    // ...\n  }\n}");
}
#[test]
fn test_method_js_format_1_3dea1065() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T = { method: () => void };\ntype T2 = { method(): void };\ndeclare class X { method(): void }\ndeclare function f(): void;\nvar f: () => void;\n\ndeclare class X {\n  static deserialize(): mixed,\n  static deserialize: () => mixed,\n}\n\ninterface I {\n  static(): number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T = { method: () => void };\ntype T2 = { method(): void };\ndeclare class X {\n  method(): void;\n}\ndeclare function f(): void;\nvar f: () => void;\n\ndeclare class X {\n  static deserialize(): mixed;\n  static deserialize: () => mixed;\n}\n\ninterface I {\n  static(): number;\n}");
}
