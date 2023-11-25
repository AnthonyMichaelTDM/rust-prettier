#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_more_switch_js_format_1_b02c9209() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(x): number {\n    switch (x) {\n      case 0:\n      case 1: return 1;\n      default: throw new Error('hi');\n    }\n}\n\nfunction bar(x) {\n    switch (x) {\n      case 0: break;\n      default: return;\n    }\n    1;\n}\n\nfunction baz(x): number {\n  switch (x) {\n    case 0: break;\n    case 1: return 1;\n    default: throw new Error('hi');\n  }\n  return 2;\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(x): number {\n  switch (x) {\n    case 0:\n    case 1:\n      return 1;\n    default:\n      throw new Error(\"hi\");\n  }\n}\n\nfunction bar(x) {\n  switch (x) {\n    case 0:\n      break;\n    default:\n      return;\n  }\n  1;\n}\n\nfunction baz(x): number {\n  switch (x) {\n    case 0:\n      break;\n    case 1:\n      return 1;\n    default:\n      throw new Error(\"hi\");\n  }\n  return 2;\n}");
    Ok(())
}
#[test]
fn test_switch_js_format_1_b285d22f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\nfunction foo(\n): number {\n    switch ('foo') {\n    case 'foo':\n        return 1;\n    }\n    return 2;\n}\n\nfunction bar() {\n    switch ('bar') {\n    case 'bar':\n        break;\n    default:\n        break;\n    }\n}\n\nfunction qux(b) {\n    var x = b? 0: \"\";\n    switch('qux') {\n    case '':\n        x = 0;\n    case 'qux':\n        x = x*x;\n    }\n}") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\nfunction foo(): number {\n  switch (\"foo\") {\n    case \"foo\":\n      return 1;\n  }\n  return 2;\n}\n\nfunction bar() {\n  switch (\"bar\") {\n    case \"bar\":\n      break;\n    default:\n      break;\n  }\n}\n\nfunction qux(b) {\n  var x = b ? 0 : \"\";\n  switch (\"qux\") {\n    case \"\":\n      x = 0;\n    case \"qux\":\n      x = x * x;\n  }\n}");
    Ok(())
}
#[test]
fn test_switch_default_fallthrough_js_format_1_92ab5963() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\nfunction foo(x : mixed): string {\n    var a = \"\";\n    var b = \"\";\n\n    switch (x) {\n      case \"foo\":\n        a = 0;\n      default:\n        b = 0;\n    }\n\n    // a is now string | number\n    (a : string); // error, string | number ~/> string\n    (a : number); // error, string | number ~/> number\n\n    // b is now number\n    (b : number); // ok\n    return b; // error, number ~/> string\n}\n\nfunction baz(x: mixed): number {\n    var a = \"\";\n    var b = \"\";\n\n    switch (x) {\n      case \"baz\":\n        a = 0;\n        break;\n      case \"bar\":\n        a = \"\";\n      default:\n        b = 0;\n    }\n\n    // a is now string | number\n    (a : string); // error, string | number ~/> string\n    (a : number); // error, string | number ~/> number\n\n    // b is now string | number\n    (b : string); // error, string | number ~/> string\n    (b : number); // error, string | number ~/> number\n\n    return a+b; // error, string ~/> number\n}") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\nfunction foo(x: mixed): string {\n  var a = \"\";\n  var b = \"\";\n\n  switch (x) {\n    case \"foo\":\n      a = 0;\n    default:\n      b = 0;\n  }\n\n  // a is now string | number\n  (a: string); // error, string | number ~/> string\n  (a: number); // error, string | number ~/> number\n\n  // b is now number\n  (b: number); // ok\n  return b; // error, number ~/> string\n}\n\nfunction baz(x: mixed): number {\n  var a = \"\";\n  var b = \"\";\n\n  switch (x) {\n    case \"baz\":\n      a = 0;\n      break;\n    case \"bar\":\n      a = \"\";\n    default:\n      b = 0;\n  }\n\n  // a is now string | number\n  (a: string); // error, string | number ~/> string\n  (a: number); // error, string | number ~/> number\n\n  // b is now string | number\n  (b: string); // error, string | number ~/> string\n  (b: number); // error, string | number ~/> number\n\n  return a + b; // error, string ~/> number\n}");
    Ok(())
}
#[test]
fn test_trailing_cases_js_format_1_06bc4adb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * trailing cases are allowed - spot checks that we handle them as usual\n * @flow\n */\nfunction f1(i) {\n  var x;\n\n  switch (i) {\n  case 0:\n    x = 0;\n    break;\n  case 1:\n    x = 1;\n    break;\n  default:\n    x = -1;\n    break;\n  case 2:\n    x = \"2\";\n    break;\n  }\n\n  var y:number = x; // error, number | string ~/> number\n}\n\nfunction f2(i) {\n  var x;\n\n  switch (i) {\n  case 0:\n  case 1:\n  default:\n    x = 1;\n    break;\n  case 2:\n    // does not fall through default\n  }\n\n  var y:number = x; // error, number | uninitialized ~/> number\n}\n\nfunction f3(i) {\n  var x;\n\n  switch (i) {\n  case 0:\n  case 1:\n  default:\n    // falls through to subsequent cases\n  case 2:\n    x = 1;\n  }\n\n  var y:number = x; // no error\n}\n\nfunction foo(x): number {\n    switch (x) {\n      case 0:\n      default: throw new Error('hi');\n      case 1: return 1;\n    }\n}\n\nfunction bar(x) {\n    switch (x) {\n      default: return;\n      case 0: break;\n    }\n    1;\n}\n\nfunction baz(x): number {\n  switch (x) {\n    case 0: break;\n    default: throw new Error('hi');\n    case 1: return 1;\n  }\n  return 2;\n}") ? ;
    assert_eq ! (formatted , "/**\n * trailing cases are allowed - spot checks that we handle them as usual\n * @flow\n */\nfunction f1(i) {\n  var x;\n\n  switch (i) {\n    case 0:\n      x = 0;\n      break;\n    case 1:\n      x = 1;\n      break;\n    default:\n      x = -1;\n      break;\n    case 2:\n      x = \"2\";\n      break;\n  }\n\n  var y: number = x; // error, number | string ~/> number\n}\n\nfunction f2(i) {\n  var x;\n\n  switch (i) {\n    case 0:\n    case 1:\n    default:\n      x = 1;\n      break;\n    case 2:\n    // does not fall through default\n  }\n\n  var y: number = x; // error, number | uninitialized ~/> number\n}\n\nfunction f3(i) {\n  var x;\n\n  switch (i) {\n    case 0:\n    case 1:\n    default:\n    // falls through to subsequent cases\n    case 2:\n      x = 1;\n  }\n\n  var y: number = x; // no error\n}\n\nfunction foo(x): number {\n  switch (x) {\n    case 0:\n    default:\n      throw new Error(\"hi\");\n    case 1:\n      return 1;\n  }\n}\n\nfunction bar(x) {\n  switch (x) {\n    default:\n      return;\n    case 0:\n      break;\n  }\n  1;\n}\n\nfunction baz(x): number {\n  switch (x) {\n    case 0:\n      break;\n    default:\n      throw new Error(\"hi\");\n    case 1:\n      return 1;\n  }\n  return 2;\n}");
    Ok(())
}
