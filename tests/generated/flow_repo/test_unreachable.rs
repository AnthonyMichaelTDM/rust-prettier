use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_typecheck_js_format_1_7fe74422() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction test1(): string {\n  return bar();\n\n  function bar() {\n    return 0;\n  }\n}\n\n// regression test for analysis after abnormal control flow:\n// consts must not become bot (EmptyT).\n\nfunction test2() {\n  const n = 0;\n\n  return;\n\n  function f() {\n    var x: typeof n = 0;  // no error, n is still number\n    var y: string = n;    // error, n is number (EmptyT would work)\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction test1(): string {\n  return bar();\n\n  function bar() {\n    return 0;\n  }\n}\n\n// regression test for analysis after abnormal control flow:\n// consts must not become bot (EmptyT).\n\nfunction test2() {\n  const n = 0;\n\n  return;\n\n  function f() {\n    var x: typeof n = 0; // no error, n is still number\n    var y: string = n; // error, n is number (EmptyT would work)\n  }\n}");
}
#[test]
fn test_unreachable_js_format_1_50dda95c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(x, y) {\n  \"use strict\";\n  return bar(x) + baz(y);\n\n  // function declaration is hoisted, should not generate warning\n  function bar (ecks) {\n    return x + ecks;\n  }\n\n  // assignment is not hoisted, should generate warning\n  var baz = function (why) {\n    return y + why;\n  };\n\n  // variable declaration is hoisted, should not generate warning\n  var x, y, z;\n\n  // assignments are not hoisted, should generate 2 warnings\n  var t,\n      u = 5,\n      v,\n      w = 7;\n}\n\nfoo(1, 2);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(x, y) {\n  \"use strict\";\n  return bar(x) + baz(y);\n\n  // function declaration is hoisted, should not generate warning\n  function bar(ecks) {\n    return x + ecks;\n  }\n\n  // assignment is not hoisted, should generate warning\n  var baz = function (why) {\n    return y + why;\n  };\n\n  // variable declaration is hoisted, should not generate warning\n  var x, y, z;\n\n  // assignments are not hoisted, should generate 2 warnings\n  var t,\n    u = 5,\n    v,\n    w = 7;\n}\n\nfoo(1, 2);");
}
