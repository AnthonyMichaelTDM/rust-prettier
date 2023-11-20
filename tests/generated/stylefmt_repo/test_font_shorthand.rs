#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_font_shorthand_css_format_1_9dce291d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["css"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".class {\n  font: normal normal 24px/1 \"myfont\";font: normal normal normal 12px/20px myfont;\n  font:normal 300 0.875em/1.3  \"myfont\", sans-serif;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".class {\n  font: normal normal 24px/1 \"myfont\";\n  font: normal normal normal 12px/20px myfont;\n  font:\n    normal 300 0.875em/1.3 \"myfont\",\n    sans-serif;\n}");
}
