#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_ts_trailing_commaes_5_format_1_c2984eef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("interface A1  // comment\n{  foo(): bar;}\n\ninterface A2  // comment\nextends Base\n{  foo(): bar;}\n\ninterface A3  // comment1\nextends Base  // comment2\n{  foo(): bar;}\n\ninterface A4  // comment1\nextends Base  // comment2\n              // comment3\n{  foo(): bar;}\n\ninterface A5  // comment1\nextends Base  // comment2\n              // comment3\n{             // comment4\nfoo(): bar;}\n\ninterface A6  // comment1\nextends Base  // comment2\n              // comment3\n{\n// comment4\nfoo(): bar;}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "interface A1 {\n  // comment\n  foo(): bar;\n}\n\ninterface A2 // comment\n  extends Base {\n  foo(): bar;\n}\n\ninterface A3 // comment1\n  extends Base {\n  // comment2\n  foo(): bar;\n}\n\ninterface A4 // comment1\n  extends Base {\n  // comment2\n  // comment3\n  foo(): bar;\n}\n\ninterface A5 // comment1\n  extends Base {\n  // comment2\n  // comment3\n  // comment4\n  foo(): bar;\n}\n\ninterface A6 // comment1\n  extends Base {\n  // comment2\n  // comment3\n  // comment4\n  foo(): bar;\n}");
}
#[test]
fn test_comments_declare_ts_trailing_commaes_5_format_1_4de68c3a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare interface a // 1\n  extends b  // 2\n{  foo: boolean}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare interface a // 1\n  extends b {\n  // 2\n  foo: boolean;\n}"
    );
}
#[test]
fn test_module_ts_trailing_commaes_5_format_1_1b3e2ab8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("declare module X {\n  declare interface Y { x: number; }\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "declare module X {\n  declare interface Y {\n    x: number;\n  }\n}"
    );
}
