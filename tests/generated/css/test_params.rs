#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_params_css_format_1_286025af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}") ? ;
    assert_eq ! (formatted , ".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}");
    Ok(())
}
