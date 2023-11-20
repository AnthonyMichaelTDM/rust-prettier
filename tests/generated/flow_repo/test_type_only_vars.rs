#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_33c0bd47() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nclass Foo {\n}\n\nclass Bar {\n}\n\nmodule.exports = {\n  Foo: Foo,\n  Bar: Bar\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nclass Foo {}\n\nclass Bar {}\n\nmodule.exports = {\n  Foo: Foo,\n  Bar: Bar,\n};");
}
#[test]
fn test_bad_shadowing_js_format_1_7de2c3ed() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type {Foo, Bar as Baz} from \"./A.js\";\n\ntype duck = {\n  quack: () => string;\n}\n\n// These string types should confict with the imported types\nvar A: string = \"Hello\";\nvar Foo: string = \"Goodbye\";\nvar Baz: string = \"Go away please\";\n\n// This string type should conflict with the typedef\nvar duck: string = \"quack\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type { Foo, Bar as Baz } from \"./A.js\";\n\ntype duck = {\n  quack: () => string,\n};\n\n// These string types should confict with the imported types\nvar A: string = \"Hello\";\nvar Foo: string = \"Goodbye\";\nvar Baz: string = \"Go away please\";\n\n// This string type should conflict with the typedef\nvar duck: string = \"quack\";");
}
#[test]
fn test_good_shadowing_js_format_1_08745fea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type {Foo, Bar as Baz} from \"./A.js\";\n\nvar A = require('./A.js');\nvar Foo = A.Foo;\nvar Baz = A.Bar;\n\n// errors in prev block leave type bindings in place, so these are errors\nvar m = A;\nvar n = Foo;\nvar o = Baz;\n\n// errors from value positions only\nvar a: Foo = new Foo();\nvar b: Foo = new A.Foo();\n(new A.Bar(): Baz);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type { Foo, Bar as Baz } from \"./A.js\";\n\nvar A = require(\"./A.js\");\nvar Foo = A.Foo;\nvar Baz = A.Bar;\n\n// errors in prev block leave type bindings in place, so these are errors\nvar m = A;\nvar n = Foo;\nvar o = Baz;\n\n// errors from value positions only\nvar a: Foo = new Foo();\nvar b: Foo = new A.Foo();\n(new A.Bar(): Baz);");
}
#[test]
fn test_import_type_js_format_1_07776d20() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type {Foo, Bar as Baz} from \"./A.js\";\n\nvar actualA = require('./A.js');\n\n// You can't use it as an identifier\nvar m = A;\nvar n = Foo;\nvar o = Baz;\n\n// But using it in a type should still work\nvar a: Foo = new actualA.Foo();\n(new actualA.Bar(): Baz);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nimport typeof A from \"./A.js\";\nimport type { Foo, Bar as Baz } from \"./A.js\";\n\nvar actualA = require(\"./A.js\");\n\n// You can't use it as an identifier\nvar m = A;\nvar n = Foo;\nvar o = Baz;\n\n// But using it in a type should still work\nvar a: Foo = new actualA.Foo();\n(new actualA.Bar(): Baz);");
}
#[test]
fn test_type_alias_js_format_1_830293b3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\ntype Foo = number;\n\n// You can't use it as an identifier\nvar x = Foo;\n\n// But using it in a type should still work\nvar a: Foo = 123;\nvar b: Array<Foo> = [123];\ntype c = Foo;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\ntype Foo = number;\n\n// You can't use it as an identifier\nvar x = Foo;\n\n// But using it in a type should still work\nvar a: Foo = 123;\nvar b: Array<Foo> = [123];\ntype c = Foo;");
}
