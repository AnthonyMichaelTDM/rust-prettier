#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_intersection_js_format_1_5558ed5f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x: $All<Error,{type:number}>): number {\n    return x.type;\n}\n\nfunction bar(x: Error & {type:number}): number {\n    return x.type;\n}") ? ;
    assert_eq ! (formatted , "function foo(x: $All<Error, { type: number }>): number {\n  return x.type;\n}\n\nfunction bar(x: Error & { type: number }): number {\n  return x.type;\n}");
    Ok(())
}
#[test]
fn test_objassign_js_format_1_fa5f27ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Test intersection of objects flowing to spread assignment.\n *\n * Definitions in lib/lib.js\n *\n * @noflow\n */\n\ndeclare var x: ObjAssignT;\n\nlet y: ObjAssignT = { ...x }; // should be fine") ? ;
    assert_eq ! (formatted , "/**\n * Test intersection of objects flowing to spread assignment.\n *\n * Definitions in lib/lib.js\n *\n * @noflow\n */\n\ndeclare var x: ObjAssignT;\n\nlet y: ObjAssignT = { ...x }; // should be fine");
    Ok(())
}
#[test]
fn test_pred_js_format_1_75623a95() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Test interaction of object intersections and predicates.\n * Definitions in lib/lib.js\n *\n * @flow\n */\n\ntype DuplexStreamOptions = ReadableStreamOptions & WritableStreamOptions & {\n  allowHalfOpen? : boolean,\n  readableObjectMode? : boolean,\n  writableObjectMode? : boolean\n};\n\nfunction hasObjectMode_bad(options: DuplexStreamOptions): boolean {\n  return options.objectMode\n    || options.readableObjectMode\n    || options.writableObjectMode; // error, undefined ~> boolean\n}\n\nfunction hasObjectMode_ok(options: DuplexStreamOptions): boolean {\n  return !!(options.objectMode\n    || options.readableObjectMode\n    || options.writableObjectMode);\n}") ? ;
    assert_eq ! (formatted , "/**\n * Test interaction of object intersections and predicates.\n * Definitions in lib/lib.js\n *\n * @flow\n */\n\ntype DuplexStreamOptions = ReadableStreamOptions &\n  WritableStreamOptions & {\n    allowHalfOpen?: boolean,\n    readableObjectMode?: boolean,\n    writableObjectMode?: boolean,\n  };\n\nfunction hasObjectMode_bad(options: DuplexStreamOptions): boolean {\n  return (\n    options.objectMode ||\n    options.readableObjectMode ||\n    options.writableObjectMode\n  ); // error, undefined ~> boolean\n}\n\nfunction hasObjectMode_ok(options: DuplexStreamOptions): boolean {\n  return !!(\n    options.objectMode ||\n    options.readableObjectMode ||\n    options.writableObjectMode\n  );\n}");
    Ok(())
}
#[test]
fn test_test_fun_js_format_1_4b95c45b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Tests for intersections of function types.\n *\n * Note: Flow abuses intersection types to model\n * function overloading, which precludes using a\n * correct intersection of return types in the result.\n *\n * Here we test the special case where return types\n * are equal. Tests of the overloading behavior can\n * be found in tests/overload\n *\n * Definitions lin lib/lib.js\n *\n * @noflow\n */\n\n// intersection of function types satisfies union of param types\n\ntype F = (_: ObjA) => void;\ntype G = (_: ObjB) => void;\ntype FG = (_: ObjA | ObjB) => void;\n\ndeclare var fun1 : F & G;\n\n(fun1 : FG);\n\nvar fun2 : FG = fun1;\n\n// simpler variation\ndeclare var f : ((_: number) => void) & ((_: string) => void);\nvar g: (_: number | string) => void = f;") ? ;
    assert_eq ! (formatted , "/**\n * Tests for intersections of function types.\n *\n * Note: Flow abuses intersection types to model\n * function overloading, which precludes using a\n * correct intersection of return types in the result.\n *\n * Here we test the special case where return types\n * are equal. Tests of the overloading behavior can\n * be found in tests/overload\n *\n * Definitions lin lib/lib.js\n *\n * @noflow\n */\n\n// intersection of function types satisfies union of param types\n\ntype F = (_: ObjA) => void;\ntype G = (_: ObjB) => void;\ntype FG = (_: ObjA | ObjB) => void;\n\ndeclare var fun1: F & G;\n\n(fun1: FG);\n\nvar fun2: FG = fun1;\n\n// simpler variation\ndeclare var f: ((_: number) => void) & ((_: string) => void);\nvar g: (_: number | string) => void = f;");
    Ok(())
}
#[test]
fn test_test_obj_js_format_1_178ab9be() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Tests for intersections of object types\n *\n * @noflow\n */\n\n// TODO we should give explicit errors for incompatibilities\n// which make an intersection uninhabitable:\n// - shared mutable properties with different types\n// - different dictionary types\n//\n// Currently we give no such errors. Instead, we rely on\n// the impossibility of providing a value for such a type\n// to produce errors on value inflows. This is clearly\n// suboptimal, since eg declared vars require no explicit\n// provision of values. This leaves the impossible types\n// free to flow downstream and satisfy impossible constraints.\n\n// intersection of object types satisfies union of properties\ndeclare var a: A;\nvar b: B = a;\n\n// intersection of dictionary types:\ndeclare var c: C;\nvar d: D = c; // ok\n\n// dict type mismatch\ntype E = { [key: string]: string };\nvar e: E = c; // error") ? ;
    assert_eq ! (formatted , "/**\n * Tests for intersections of object types\n *\n * @noflow\n */\n\n// TODO we should give explicit errors for incompatibilities\n// which make an intersection uninhabitable:\n// - shared mutable properties with different types\n// - different dictionary types\n//\n// Currently we give no such errors. Instead, we rely on\n// the impossibility of providing a value for such a type\n// to produce errors on value inflows. This is clearly\n// suboptimal, since eg declared vars require no explicit\n// provision of values. This leaves the impossible types\n// free to flow downstream and satisfy impossible constraints.\n\n// intersection of object types satisfies union of properties\ndeclare var a: A;\nvar b: B = a;\n\n// intersection of dictionary types:\ndeclare var c: C;\nvar d: D = c; // ok\n\n// dict type mismatch\ntype E = { [key: string]: string };\nvar e: E = c; // error");
    Ok(())
}
