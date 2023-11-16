#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_toplevel_js_format_1_b1efed9d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {name} from \"testproj\";\n\n(name: \"node_modules/testproj\");\n(name: \"custom_resolve_dir/testproj\"); // Error: Resolve from node_modules first!") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { name } from \"testproj\";\n\n(name: \"node_modules/testproj\");\n(name: \"custom_resolve_dir/testproj\"); // Error: Resolve from node_modules first!");
}
