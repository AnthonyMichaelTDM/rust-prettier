#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_abnormal_js_format_1_560e0dd3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo(x: boolean) {\n  var ii = 10;\n  while (ii-- >= 0) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  //console.log('this is still reachable');\n}\n\nfunction bar(x: boolean) {\n  var ii = 0;\n  while (ii > 0) {\n    return;\n  }\n  //console.log('this is still reachable');\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo(x: boolean) {\n  var ii = 10;\n  while (ii-- >= 0) {\n    if (x) {\n      continue;\n    }\n    return;\n  }\n  //console.log('this is still reachable');\n}\n\nfunction bar(x: boolean) {\n  var ii = 0;\n  while (ii > 0) {\n    return;\n  }\n  //console.log('this is still reachable');\n}");
}
#[test]
fn test_test_js_format_1_291673c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n    m() { return new C; }\n}\nfunction blah() {}\nvar node: ?C = new C;\nwhile (node) {\n    var parent = node.m();\n    var cloneable: C = node;\n    blah();\n    node = parent.m();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  m() {\n    return new C();\n  }\n}\nfunction blah() {}\nvar node: ?C = new C();\nwhile (node) {\n  var parent = node.m();\n  var cloneable: C = node;\n  blah();\n  node = parent.m();\n}");
}
