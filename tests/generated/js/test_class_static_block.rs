#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_static_block_js_format_1_3073ee91() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  static #x = 42;\n  static y;\n  static {\n    try {\n      this.y = doSomethingWith(this.#x);\n    } catch {\n      this.y = \"unknown\";\n    }\n  }\n}\n  \nclass Foo {\n  static {}\n}\n  \nclass A1 {\n  static {\n    foo;\n  }\n}\n  \nclass A2 {\n  static {\n    foo;\n    bar;\n  }\n}\n ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class C {\n  static #x = 42;\n  static y;\n  static {\n    try {\n      this.y = doSomethingWith(this.#x);\n    } catch {\n      this.y = \"unknown\";\n    }\n  }\n}\n\nclass Foo {\n  static {}\n}\n\nclass A1 {\n  static {\n    foo;\n  }\n}\n\nclass A2 {\n  static {\n    foo;\n    bar;\n  }\n}");
}
#[test]
fn test_with_line_breaks_js_format_1_6d78ee76() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo {\n  static\n  {\n     1 + 1;\n  }\n};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class Foo {\n  static {\n    1 + 1;\n  }\n}");
}
