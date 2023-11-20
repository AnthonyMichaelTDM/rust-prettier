#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_params_css_format_1_286025af() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["css"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}");
}
