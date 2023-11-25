#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nested_css_format_1_5c4d7da7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("     .foo {font-size   :12px      ;\n\n\n  >.nested-1\n \n  {\npadding: 10px;   .nested-2-1{\n      color       :      red     ;\n    }\n           +      .nested-2-2      {\n  color:blue;  }}\n\n\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".foo {\n  font-size: 12px;\n\n  > .nested-1 {\n    padding: 10px;\n    .nested-2-1 {\n      color: red;\n    }\n    + .nested-2-2 {\n      color: blue;\n    }\n  }\n}");
}
