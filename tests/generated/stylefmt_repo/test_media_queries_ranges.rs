#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_media_queries_ranges_css_format_1_06e2b495() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@media (width >= 500px) and (width <= 1200px) {\n  .rule {color:red;}\n}\n@custom-media --only-medium-screen (    width    >=500px     ) and (width<=    1200px    ) ;\n@media (       --only-medium-screen ){\n  .rule{color:blue;}}") ? ;
    assert_eq ! (formatted , "@media (width >= 500px) and (width <= 1200px) {\n  .rule {\n    color: red;\n  }\n}\n@custom-media --only-medium-screen (width >=500px) and (width<= 1200px);\n@media (--only-medium-screen) {\n  .rule {\n    color: blue;\n  }\n}");
    Ok(())
}
