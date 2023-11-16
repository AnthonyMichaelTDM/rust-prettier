#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_text_js_format_1_c3afed7d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = {\n  viewer: graphql\\`\n    fragment x on Viewer {\n      y(named: [\n        \"projects_feedback_ids\" # PROJECTS_FEEDBACK_IDS\n      ]) {\n        name\n      }\n    }\n  \\`,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = {\n  viewer: graphql\\`\n    fragment x on Viewer {\n      y(\n        named: [\n          \"projects_feedback_ids\" # PROJECTS_FEEDBACK_IDS\n        ]\n      ) {\n        name\n      }\n    }\n  \\`,\n};");
}
