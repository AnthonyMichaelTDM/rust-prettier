use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nested_2_css_format_1_3ffd89d3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".btn {&:hover {\n    color: red;\n  }&:active {\ncolor:blue;}  &:nth-child(5n+1) {\n    color:blue;\n  }\n  &:nth-child(-n+3) { color: green;}\n> li > a {   color: red}  >li>li {\n  color: blue;\n  }\n> p + p {\n    color: green;\n  }\n  }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".btn {\n  &:hover {\n    color: red;\n  }\n  &:active {\n    color: blue;\n  }\n  &:nth-child(5n + 1) {\n    color: blue;\n  }\n  &:nth-child(-n + 3) {\n    color: green;\n  }\n  > li > a {\n    color: red;\n  }\n  > li > li {\n    color: blue;\n  }\n  > p + p {\n    color: green;\n  }\n}");
}
