use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comment_js_format_1_93e34c17() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("o = {\n  name:\n    // Comment A\n    // Comment B\n    (({id: type}: any): CreativeConcept),\n};\n\no = {\n  name: // Comment A\n  // Comment B\n  (({ id: type }: any): CreativeConcept)\n};\n\no = {\n  name: // Comment B // Comment A\n  (({ id: type }: any): CreativeConcept)\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "o = {\n  name:\n    // Comment A\n    // Comment B\n    (({ id: type }: any): CreativeConcept),\n};\n\no = {\n  // Comment A\n  name:\n    // Comment B\n    (({ id: type }: any): CreativeConcept),\n};\n\no = {\n  // Comment B // Comment A\n  name: (({ id: type }: any): CreativeConcept),\n};");
}
