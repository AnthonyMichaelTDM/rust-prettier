use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_format_1_28fbf59b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nclass Foo {\n  prop: ?string;\n  #privateProp: number;\n  fun(): void {\n    this.#privateProp;\n    this.#privateProp = 4;\n  }\n}\n\nconst foo = new Foo();\nfoo.prop;\nfoo.prop = null;\nfoo.fun();\n\nif (foo.prop != null) {\n  foo.prop;\n}\n\nfunction f(x: ?Foo): void {\n  x.fun();\n}\n\nfunction f(x: Foo | null | void): void {\n  x.fun();\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nclass Foo {\n  prop: ?string;\n  #privateProp: number;\n  fun(): void {\n    this.#privateProp;\n    this.#privateProp = 4;\n  }\n}\n\nconst foo = new Foo();\nfoo.prop;\nfoo.prop = null;\nfoo.fun();\n\nif (foo.prop != null) {\n  foo.prop;\n}\n\nfunction f(x: ?Foo): void {\n  x.fun();\n}\n\nfunction f(x: Foo | null | void): void {\n  x.fun();\n}");
}
#[test]
fn test_example_js_format_1_e24d6c0b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar lib = require('./library');\n\nfunction add(a: number, b: number): number {\n  return a + b;\n}\n\nvar re = /^keynote (talk){2} (lightning){3,5} (talk){2} closing partytime!!!/\n\n// t123456\nadd(lib.iTakeAString(42), 7);\n\n// D123456\nlib.bar();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar lib = require(\"./library\");\n\nfunction add(a: number, b: number): number {\n  return a + b;\n}\n\nvar re = /^keynote (talk){2} (lightning){3,5} (talk){2} closing partytime!!!/;\n\n// t123456\nadd(lib.iTakeAString(42), 7);\n\n// D123456\nlib.bar();");
}
#[test]
fn test_imports_js_format_1_e1dadfcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport thing from \"./helpers/exports_default.js\";\nthing;\n\nimport {foo, bar as baz} from \"./helpers/exports_named.js\";\nfoo;\nbaz;\n\nimport * as things from \"./helpers/exports_named.js\";\nthings;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport thing from \"./helpers/exports_default.js\";\nthing;\n\nimport { foo, bar as baz } from \"./helpers/exports_named.js\";\nfoo;\nbaz;\n\nimport * as things from \"./helpers/exports_named.js\";\nthings;");
}
#[test]
fn test_library_js_format_1_4354a1f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nmodule.exports = {\n\n  iTakeAString: function(name: string): number {\n    return 42;\n  },\n\n  bar: function(): number {\n    return 42;\n  },\n\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nmodule.exports = {\n  iTakeAString: function (name: string): number {\n    return 42;\n  },\n\n  bar: function (): number {\n    return 42;\n  },\n};");
}
