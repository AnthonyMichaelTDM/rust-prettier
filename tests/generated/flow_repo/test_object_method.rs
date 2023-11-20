#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_id_js_format_1_1569afc9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare function id<X>(_: X): X;\n\nmodule.exports = id;");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare function id<X>(_: X): X;\n\nmodule.exports = id;"
    );
}
#[test]
fn test_subtype_js_format_1_2a14ce4d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface Interface {\n  m(): void;\n}\nimport type { ObjectType } from './test';\n\nfunction subtypeCheck(x: Interface): ObjectType { return x; }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface Interface {\n  m(): void;\n}\nimport type { ObjectType } from \"./test\";\n\nfunction subtypeCheck(x: Interface): ObjectType {\n  return x;\n}");
}
#[test]
fn test_test_js_format_1_0bbfacf9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const id = require('./id');\n\nexport type ObjectType = {\n  +m: () => void,\n};\n\nfunction methodCaller(x: ObjectType) {\n  x.m();\n};\n\nmodule.exports = id(\n  methodCaller\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const id = require(\"./id\");\n\nexport type ObjectType = {\n  +m: () => void,\n};\n\nfunction methodCaller(x: ObjectType) {\n  x.m();\n}\n\nmodule.exports = id(methodCaller);");
}
#[test]
fn test_test_2_js_format_1_f5bc3f89() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction f() {\n  return this.p;\n}\n\nvar a = {\n  p: 0,\n  f\n}\n\nvar b = {\n  f\n}\n\na.f(); // okey-dokie\nb.f(); // error, property \\`p\\` not found") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction f() {\n  return this.p;\n}\n\nvar a = {\n  p: 0,\n  f,\n};\n\nvar b = {\n  f,\n};\n\na.f(); // okey-dokie\nb.f(); // error, property \\`p\\` not found");
}
#[test]
fn test_test_3_js_format_1_3853f2b4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nfunction foo() {\n  this.m();\n}\n\nfunction bar(f: () => void) {\n  f(); // passing global object as \\`this\\`\n  ({ f }).f(); // passing container object as \\`this\\`\n}\n\nbar(foo); // error, since \\`this\\` is used non-trivially in \\`foo\\`\n\nfunction qux(o: { f: () => void }) {\n  o.f(); // passing o as \\`this\\`\n}\n\nqux({ f: foo });  // error, since \\`this\\` is used non-trivially in \\`foo\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nfunction foo() {\n  this.m();\n}\n\nfunction bar(f: () => void) {\n  f(); // passing global object as \\`this\\`\n  ({ f }).f(); // passing container object as \\`this\\`\n}\n\nbar(foo); // error, since \\`this\\` is used non-trivially in \\`foo\\`\n\nfunction qux(o: { f: () => void }) {\n  o.f(); // passing o as \\`this\\`\n}\n\nqux({ f: foo }); // error, since \\`this\\` is used non-trivially in \\`foo\\`");
}
