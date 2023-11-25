use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_toplevel_js_format_1_b1efed9d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {name} from \"testproj\";\n\n(name: \"node_modules/testproj\");\n(name: \"custom_resolve_dir/testproj\"); // Error: Resolve from node_modules first!") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport { name } from \"testproj\";\n\n(name: \"node_modules/testproj\");\n(name: \"custom_resolve_dir/testproj\"); // Error: Resolve from node_modules first!");
}
