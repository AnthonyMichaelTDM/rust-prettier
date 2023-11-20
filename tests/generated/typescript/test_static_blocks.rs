#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_multiple_ts_format_1_2a28cbcd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript", "babel-ts"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  static prop = 1\n  static {\n    console.log(Foo.prop++);\n  }\n  static {\n    console.log(Foo.prop++);\n  }\n  static {\n    console.log(Foo.prop++);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  static prop = 1;\n  static {\n    console.log(Foo.prop++);\n  }\n  static {\n    console.log(Foo.prop++);\n  }\n  static {\n    console.log(Foo.prop++);\n  }\n}");
}
#[test]
fn test_nested_ts_format_1_a19ae76a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "babel-ts"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo () {\n  return class {\n    static foo = 1;\n    static {\n      const c = class {\n        static bar = 2;\n        static {\n          // do\n        }\n      }\n    }\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {\n  return class {\n    static foo = 1;\n    static {\n      const c = class {\n        static bar = 2;\n        static {\n          // do\n        }\n      };\n    }\n  };\n}");
}
#[test]
fn test_static_blocks_ts_format_1_f02427ba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript", "babel-ts"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  static #count = 0;\n\n  get count() {\n    return Foo.#count;\n  }\n\n  static {\n    try {\n      const lastInstances = loadLastInstances();\n      Foo.#count += lastInstances.length;\n    }\n    catch {}\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  static #count = 0;\n\n  get count() {\n    return Foo.#count;\n  }\n\n  static {\n    try {\n      const lastInstances = loadLastInstances();\n      Foo.#count += lastInstances.length;\n    } catch {}\n  }\n}");
}
