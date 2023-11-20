#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_readme_css_format_1_b168deee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// mixin for clearfix\n        @mixin      clearfix    ()      { &:before,\n  &:after {\n                content:\" \";\n    display              : table;  }\n\n  &:after        {clear: both;}\n   }.class\n{\n       padding:10px;@include        clearfix();}\n     .base {  color: red;  } // placeholder\n\n%base\n{\n\n\npadding: 12px\n}\n\n.foo{\n@extend      .base;}\n\n.bar\n      {     @extend            %base;\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// mixin for clearfix\n@mixin clearfix() {\n  &:before,\n  &:after {\n    content: \" \";\n    display: table;\n  }\n\n  &:after {\n    clear: both;\n  }\n}\n.class {\n  padding: 10px;\n  @include clearfix();\n}\n.base {\n  color: red;\n} // placeholder\n\n%base {\n  padding: 12px;\n}\n\n.foo {\n  @extend .base;\n}\n\n.bar {\n  @extend %base;\n}");
}
