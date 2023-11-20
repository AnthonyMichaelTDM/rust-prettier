#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_conditional_js_format_1_344b86ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction a(): number {\n  var x: ?string = null;\n  return x ? 1 : 0;\n}\n\nfunction b(): number {\n    var x: ?number = null;\n    return x != null ? x : 0;\n}\n\nfunction c(): number {\n  // equivalent to \\`return (x && 1) || 0\\`\n  var x = false;\n  var temp = (x ? 1 : x);\n  return temp ? temp : 0;\n}\n\nfunction d(): string { // expected \\`: number | boolean\\`\n  // equivalent to \\`return x != null && x\\`\n  var x: ?number = null;\n  return (x != null) ? x : (x != null);\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction a(): number {\n  var x: ?string = null;\n  return x ? 1 : 0;\n}\n\nfunction b(): number {\n  var x: ?number = null;\n  return x != null ? x : 0;\n}\n\nfunction c(): number {\n  // equivalent to \\`return (x && 1) || 0\\`\n  var x = false;\n  var temp = x ? 1 : x;\n  return temp ? temp : 0;\n}\n\nfunction d(): string {\n  // expected \\`: number | boolean\\`\n  // equivalent to \\`return x != null && x\\`\n  var x: ?number = null;\n  return x != null ? x : x != null;\n}");
}
