#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_media_queries_ranges_css_format_1_06e2b495() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@media (width >= 500px) and (width <= 1200px) {\n  .rule {color:red;}\n}\n@custom-media --only-medium-screen (    width    >=500px     ) and (width<=    1200px    ) ;\n@media (       --only-medium-screen ){\n  .rule{color:blue;}}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@media (width >= 500px) and (width <= 1200px) {\n  .rule {\n    color: red;\n  }\n}\n@custom-media --only-medium-screen (width >=500px) and (width<= 1200px);\n@media (--only-medium-screen) {\n  .rule {\n    color: blue;\n  }\n}");
}
