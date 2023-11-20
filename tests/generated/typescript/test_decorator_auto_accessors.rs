#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_decorator_auto_accessors_new_line_ts_format_1_da0b4aaf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  accessor\n  [\"bar\"];\n}\n\nclass Foo {\n  static accessor\n  bar;\n}\n\nclass Foo {\n  accessor\n  bar;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  accessor;\n  [\"bar\"];\n}\n\nclass Foo {\n  static accessor;\n  bar;\n}\n\nclass Foo {\n  accessor;\n  bar;\n}");
}
#[test]
fn test_decorator_auto_accessors_type_annotations_ts_format_1_7c10edc5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("abstract class Foo {\n  accessor prop: number = 1;\n  static accessor prop2: number = 1;\n  accessor #prop3: number = 1;\n  accessor [prop4]: number = 1;\n  private accessor prop5: number = 1;\n  abstract accessor prop6: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "abstract class Foo {\n  accessor prop: number = 1;\n  static accessor prop2: number = 1;\n  accessor #prop3: number = 1;\n  accessor [prop4]: number = 1;\n  private accessor prop5: number = 1;\n  abstract accessor prop6: number;\n}");
}
