#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_object_js_format_1_3007b723() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("state = {\n  // students\n  hoverColumn: -1\n};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "state = {\n  // students\n  hoverColumn: -1,\n};"
    );
}
#[test]
fn test_series_js_format_1_a105191c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("1 + ++x;\n1 + x++;\n\n+ ++x;\n+ x++;\n\nx++ + 1;\n++x + 1;\n\n1 - --x;\n1 - x--;\n\n- --x;\n- x--;\n\nx-- - 1;\n--x - 1;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "1 + ++x;\n1 + x++;\n\n+(++x);\n+x++;\n\nx++ + 1;\n++x + 1;\n\n1 - --x;\n1 - x--;\n\n-(--x);\n-x--;\n\nx-- - 1;\n--x - 1;");
}
