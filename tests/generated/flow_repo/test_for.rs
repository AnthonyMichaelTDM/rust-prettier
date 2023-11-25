use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abnormal_js_format_1_aa827120() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(x: boolean) {\n  var max = 10;\n  for (var ii = 0; ii < max; ii++) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bar(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction baz(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    continue;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bliffl(x: boolean) {\n  var max = 10;\n  loop1: for (var ii = 0; ii < max; ii++) {\n    loop2: for (var jj = 0; jj < max; jj++) {\n      break loop1;\n    }\n    console.log('this is still reachable');\n  }\n  console.log('this is still reachable');\n}\n\nfunction corge(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    break;\n  }\n  console.log('this is still reachable');\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(x: boolean) {\n  var max = 10;\n  for (var ii = 0; ii < max; ii++) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bar(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction baz(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    continue;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bliffl(x: boolean) {\n  var max = 10;\n  loop1: for (var ii = 0; ii < max; ii++) {\n    loop2: for (var jj = 0; jj < max; jj++) {\n      break loop1;\n    }\n    console.log(\"this is still reachable\");\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction corge(x: boolean) {\n  var max = 0;\n  for (var ii = 0; ii < max; ii++) {\n    break;\n  }\n  console.log(\"this is still reachable\");\n}");
}
#[test]
fn test_abnormal_for_in_js_format_1_c9d057c6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x: boolean) {\n  var obj = { a: 1, b: 2};\n  for (var prop in obj) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bar(x: boolean) {\n  for (var prop in {}) {\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction baz(x: boolean) {\n  for (var prop in {}) {\n    continue;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bliffl(x: boolean) {\n  var obj = { a: 1, b: 2};\n  loop1: for (var prop1 in obj) {\n    loop2: for (var prop2 in obj) {\n      break loop1;\n    }\n    console.log('this is still reachable');\n  }\n  console.log('this is still reachable');\n}\n\nfunction corge(x: boolean) {\n  for (var prop in {}) {\n    break;\n  }\n  console.log('this is still reachable');\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(x: boolean) {\n  var obj = { a: 1, b: 2 };\n  for (var prop in obj) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bar(x: boolean) {\n  for (var prop in {}) {\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction baz(x: boolean) {\n  for (var prop in {}) {\n    continue;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bliffl(x: boolean) {\n  var obj = { a: 1, b: 2 };\n  loop1: for (var prop1 in obj) {\n    loop2: for (var prop2 in obj) {\n      break loop1;\n    }\n    console.log(\"this is still reachable\");\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction corge(x: boolean) {\n  for (var prop in {}) {\n    break;\n  }\n  console.log(\"this is still reachable\");\n}");
}
#[test]
fn test_abnormal_for_of_js_format_1_5c94c360() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x: boolean) {\n  var arr = [1, 2, 3];\n  for (var elem of arr) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bar(x: boolean) {\n  for (var elem of []) {\n    return;\n  }\n  console.log('this is still reachable');\n}\n\nfunction baz(x: boolean) {\n  for (var elem of []) {\n    continue;\n  }\n  console.log('this is still reachable');\n}\n\nfunction bliffl(x: boolean) {\n  var arr = [1, 2, 3];\n  loop1: for (var elem of arr) {\n    loop2: for (var elem of arr) {\n      break loop1;\n    }\n    console.log('this is still reachable');\n  }\n  console.log('this is still reachable');\n}\n\nfunction corge(x: boolean) {\n  for (var elem of []) {\n    break;\n  }\n  console.log('this is still reachable');\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(x: boolean) {\n  var arr = [1, 2, 3];\n  for (var elem of arr) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bar(x: boolean) {\n  for (var elem of []) {\n    return;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction baz(x: boolean) {\n  for (var elem of []) {\n    continue;\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction bliffl(x: boolean) {\n  var arr = [1, 2, 3];\n  loop1: for (var elem of arr) {\n    loop2: for (var elem of arr) {\n      break loop1;\n    }\n    console.log(\"this is still reachable\");\n  }\n  console.log(\"this is still reachable\");\n}\n\nfunction corge(x: boolean) {\n  for (var elem of []) {\n    break;\n  }\n  console.log(\"this is still reachable\");\n}");
}
