#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_client_optional_js_format_1_b7ae0483() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var qux = require('./optional');\n\nqux(0);")?;
    assert_eq!(formatted, "var qux = require(\"./optional\");\n\nqux(0);");
    Ok(())
}
#[test]
fn test_default_js_format_1_7d13d0ed() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("function f<T>(foo, bar = foo): [T, T] {\n  return [foo, bar];\n}")?;
    assert_eq!(
        formatted,
        "function f<T>(foo, bar = foo): [T, T] {\n  return [foo, bar];\n}"
    );
    Ok(())
}
#[test]
fn test_generic_js_format_1_2829341f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function x<T>(x: T = 0) {}\n\nclass C {\n  x<T>(x: T = 0) {}\n}")?;
    assert_eq!(
        formatted,
        "function x<T>(x: T = 0) {}\n\nclass C {\n  x<T>(x: T = 0) {}\n}"
    );
    Ok(())
}
#[test]
fn test_loop_js_format_1_f04e99d5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* Regression test. Improper handling of OptionalT repositioning can cause\n * reasons to grow \"optional x\" -> \"optional optional x\" -> \"optional optional\n * optional x\", which thwarts reason-based caches that prevent nontermination.\n */\nfunction f<T:*>(x:T,y?:T) { x = y }") ? ;
    assert_eq ! (formatted , "/* Regression test. Improper handling of OptionalT repositioning can cause\n * reasons to grow \"optional x\" -> \"optional optional x\" -> \"optional optional\n * optional x\", which thwarts reason-based caches that prevent nontermination.\n */\nfunction f<T: *>(x: T, y?: T) {\n  x = y;\n}");
    Ok(())
}
#[test]
fn test_maybe_js_format_1_f0a2aac5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("function foo(x?: string): ?string {\n    return x;\n}")?;
    assert_eq!(
        formatted,
        "function foo(x?: string): ?string {\n  return x;\n}"
    );
    Ok(())
}
#[test]
fn test_nullable_js_format_1_00d2a6fc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction optionalNullable1(x: {y?: ?number}) {\n  if (x.y !== null && x.y !== undefined) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable2(x: {y?: ?number}) {\n  if (x.y !== undefined && x.y !== null) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable3(x: {y?: ?number}) {\n  if (!(x.y !== null && x.y !== undefined)) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable4(x: {y?: ?number}) {\n  if (!(x.y !== undefined && x.y !== null)) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable5(x: {y?: ?number}) {\n  if (x.y === null || x.y === undefined) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable6(x: {y?: ?number}) {\n  if (x.y === undefined || x.y === null) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable7(x: {y?: ?number}) {\n  if (!(x.y === null || x.y === undefined)) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable8(x: {y?: ?number}) {\n  if (!(x.y === undefined || x.y === null)) {\n    x.y++;\n  }\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction optionalNullable1(x: { y?: ?number }) {\n  if (x.y !== null && x.y !== undefined) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable2(x: { y?: ?number }) {\n  if (x.y !== undefined && x.y !== null) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable3(x: { y?: ?number }) {\n  if (!(x.y !== null && x.y !== undefined)) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable4(x: { y?: ?number }) {\n  if (!(x.y !== undefined && x.y !== null)) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable5(x: { y?: ?number }) {\n  if (x.y === null || x.y === undefined) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable6(x: { y?: ?number }) {\n  if (x.y === undefined || x.y === null) {\n    x.y++; // should error\n  }\n}\n\nfunction optionalNullable7(x: { y?: ?number }) {\n  if (!(x.y === null || x.y === undefined)) {\n    x.y++;\n  }\n}\n\nfunction optionalNullable8(x: { y?: ?number }) {\n  if (!(x.y === undefined || x.y === null)) {\n    x.y++;\n  }\n}");
    Ok(())
}
#[test]
fn test_optional_js_format_1_1c3da34c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function bar(x?,y?) { x * 0; }\nbar(0);\n\nvar foo:(x?:number)=>void = bar;\nfoo();\n\nfunction qux(x=\"hello\",...y):string { foo(x); return y[0]; }\n\nqux(0,0); // Error, number ~> string\nqux(0,...[42, \"\"]); // Error, number ~> string\nqux(0,...[\"\",42]); // No error\n\nmodule.exports = qux;") ? ;
    assert_eq ! (formatted , "function bar(x?, y?) {\n  x * 0;\n}\nbar(0);\n\nvar foo: (x?: number) => void = bar;\nfoo();\n\nfunction qux(x = \"hello\", ...y): string {\n  foo(x);\n  return y[0];\n}\n\nqux(0, 0); // Error, number ~> string\nqux(0, ...[42, \"\"]); // Error, number ~> string\nqux(0, ...[\"\", 42]); // No error\n\nmodule.exports = qux;");
    Ok(())
}
#[test]
fn test_optional_param_js_format_1_45e0a2f5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\nfunction foo(x?: string): string {\n    if (x == null) { return 'foo'; }\n    return x;\n}\n\nfunction bar(obj: {x?: string}): string {\n    if (obj.x == null) { return 'foo'; }\n    return obj.x;\n}\n\nfunction baz(bar?) {\n    if (!bar) { return 1; }\n    return bar.duck\n}\n\nfunction testOptionalNullable(x?: ?string): string {\n    if (x == null) { return 'foo'; }\n    return x;\n}\n\nfunction testOptionalNullableDefault(x?: ?string = \"hi\"): string {\n    if (x == null) { return 'foo'; }\n    return x;\n}\n\nfunction testOptionalNullableProperty(obj: {x?: ?string}): string {\n    if (obj.x == null) { return 'foo'; }\n    return obj.x;\n}\n\nfunction testOptionalNullableFlowingToNullable(x?: ?string): ?string {\n  var f = function(y: ?string) {};\n  f(x);\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\nfunction foo(x?: string): string {\n  if (x == null) {\n    return \"foo\";\n  }\n  return x;\n}\n\nfunction bar(obj: { x?: string }): string {\n  if (obj.x == null) {\n    return \"foo\";\n  }\n  return obj.x;\n}\n\nfunction baz(bar?) {\n  if (!bar) {\n    return 1;\n  }\n  return bar.duck;\n}\n\nfunction testOptionalNullable(x?: ?string): string {\n  if (x == null) {\n    return \"foo\";\n  }\n  return x;\n}\n\nfunction testOptionalNullableDefault(x?: ?string = \"hi\"): string {\n  if (x == null) {\n    return \"foo\";\n  }\n  return x;\n}\n\nfunction testOptionalNullableProperty(obj: { x?: ?string }): string {\n  if (obj.x == null) {\n    return \"foo\";\n  }\n  return obj.x;\n}\n\nfunction testOptionalNullableFlowingToNullable(x?: ?string): ?string {\n  var f = function (y: ?string) {};\n  f(x);\n}");
    Ok(())
}
#[test]
fn test_optional_param_2_js_format_1_b73c0eb4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare class I<V> {\n    map<M>(\n        mapper: (value?: V) => M\n    ): I<M>;\n}\nvar i:I<number> = new I();\nvar j:I<number> = i.map(id => id);") ? ;
    assert_eq ! (formatted , "declare class I<V> {\n  map<M>(mapper: (value?: V) => M): I<M>;\n}\nvar i: I<number> = new I();\nvar j: I<number> = i.map((id) => id);");
    Ok(())
}
#[test]
fn test_optional_param_3_js_format_1_ec60c7c2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x?: number) {}\nfoo(undefined); // ok\n\nfunction bar(x = \"bar\"): string {\n  return x;\n}\nbar(undefined); // ok") ? ;
    assert_eq ! (formatted , "function foo(x?: number) {}\nfoo(undefined); // ok\n\nfunction bar(x = \"bar\"): string {\n  return x;\n}\nbar(undefined); // ok");
    Ok(())
}
#[test]
fn test_optional_param_4_js_format_1_32417d0a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(x?: number, ...y: Array<string>): [?number, Array<string>] {\n  return [x, y];\n}\n\nfoo(); // OK\nfoo(123), // OK\nfoo(123, 'hello'); // OK\n\n\nfoo(true); // ERROR boolean ~> number\nfoo(123, true); // ERROR boolean ~> string") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(x?: number, ...y: Array<string>): [?number, Array<string>] {\n  return [x, y];\n}\n\nfoo(); // OK\nfoo(123), // OK\n  foo(123, \"hello\"); // OK\n\nfoo(true); // ERROR boolean ~> number\nfoo(123, true); // ERROR boolean ~> string");
    Ok(())
}
#[test]
fn test_undefined_js_format_1_1d271a2b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x;\n\nfunction foo(bar? = undefined) {\n    x = bar;\n}\n\nfunction bar() {\n    return x.duck;\n}") ? ;
    assert_eq ! (formatted , "var x;\n\nfunction foo(bar? = undefined) {\n  x = bar;\n}\n\nfunction bar() {\n  return x.duck;\n}");
    Ok(())
}
#[test]
fn test_undefined_2_js_format_1_9833d08a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var x;\n\nfunction foo(bar?) {\n    x = bar;\n}\n\nfunction bar() {\n    return x.duck;\n}") ? ;
    assert_eq!(
        formatted,
        "var x;\n\nfunction foo(bar?) {\n  x = bar;\n}\n\nfunction bar() {\n  return x.duck;\n}"
    );
    Ok(())
}
