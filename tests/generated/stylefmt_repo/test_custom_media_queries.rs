use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_custom_media_queries_css_format_1_9b778374() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@custom-media     --small-viewport ( max-width:30em   )\n\n\n;\n\n@media (      --small-viewport    ){\n.rule{color:blue;}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@custom-media --small-viewport (max-width: 30em);\n\n@media (--small-viewport) {\n  .rule {\n    color: blue;\n  }\n}");
}
