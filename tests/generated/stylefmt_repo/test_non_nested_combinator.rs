#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_non_nested_combinator_css_format_1_14985b5b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".card-meta {\n  @include padding(null 1rem 1rem);background-color: $card-base-background-color;\n  color: $meta-font-color;\n  .card-content--with-image+&\n  {padding-top: 1rem;\n  }a {\n    color: inherit;\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".card-meta {\n  @include padding(null 1rem 1rem);\n  background-color: $card-base-background-color;\n  color: $meta-font-color;\n  .card-content--with-image + & {\n    padding-top: 1rem;\n  }\n  a {\n    color: inherit;\n  }\n}");
}
