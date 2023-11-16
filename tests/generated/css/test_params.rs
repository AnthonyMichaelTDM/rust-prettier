#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_params_css_format_1_286025af() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format (".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ".clusterPlannerDialog input[type=\"text\"],\n.clusterPlannerDialog .uiTypeahead {\n  width: 220px;\n}");
}
