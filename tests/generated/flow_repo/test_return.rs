use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_function_return_js_format_1_88da8cdc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n    foo() { }\n    bar() { return; }\n    fn(x:number) { return x; }\n}\n\nfunction f(x): number {\n  if (x > 1) {\n    return 42;\n  }\n}\n\nfunction g(x): ?number {\n  if (x > 1) {\n    return 42;\n  }\n}\n\nfunction h(x): number {\n  if (x > 1) {\n    return 42;\n  }\n  return;\n}\n\nfunction i(x): ?number {\n  if (x > 1) {\n    return 42;\n  }\n  return;\n}\n\nmodule.exports = C;\n\n//function fn(x:number) { return x; }\n//module.exports = fn;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  foo() {}\n  bar() {\n    return;\n  }\n  fn(x: number) {\n    return x;\n  }\n}\n\nfunction f(x): number {\n  if (x > 1) {\n    return 42;\n  }\n}\n\nfunction g(x): ?number {\n  if (x > 1) {\n    return 42;\n  }\n}\n\nfunction h(x): number {\n  if (x > 1) {\n    return 42;\n  }\n  return;\n}\n\nfunction i(x): ?number {\n  if (x > 1) {\n    return 42;\n  }\n  return;\n}\n\nmodule.exports = C;\n\n//function fn(x:number) { return x; }\n//module.exports = fn;");
}
#[test]
fn test_void_js_format_1_fc2be1f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* This is a regression test. At one point we incorrectly inferred the return\n   type of functions that have an explicit \\`undefined\\` to be only \\`undefined\\` --\n   ignoring other possible exits. */\nfunction f(b) {\n  if (b) {\n    return undefined;\n  } else {\n    return \"nope\";\n  }\n}\n\n(f(true): void); // error: string ~> void") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* This is a regression test. At one point we incorrectly inferred the return\n   type of functions that have an explicit \\`undefined\\` to be only \\`undefined\\` --\n   ignoring other possible exits. */\nfunction f(b) {\n  if (b) {\n    return undefined;\n  } else {\n    return \"nope\";\n  }\n}\n\n(f(true): void); // error: string ~> void");
}
