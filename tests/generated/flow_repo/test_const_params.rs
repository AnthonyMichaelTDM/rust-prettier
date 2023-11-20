#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_862935d8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * test handling of const params\n * - reassignment prohibited\n * - durable refinements\n *\n * Currently gated in .flowconfig:\n *\n * [options]\n * experimental.const_params\n *\n * Syntax to follow\n *\n * @flow\n */\n\nfunction cannot_reassign(x: string) {\n  x = \"hey\"; // error, const param cannot be reassigned\n}\n\n// Note: const params use the same machinery as explicit\n// const bindings, which are tested more extensively elsewhere.\n// Here we're just making sure the machinery is hooked up.\n//\nfunction durable_refi(x: ?number) {\n  if (x) {\n    // ok: if x is truthy here, it's truthy everywhere\n    return () => { var y:number = x; };\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * test handling of const params\n * - reassignment prohibited\n * - durable refinements\n *\n * Currently gated in .flowconfig:\n *\n * [options]\n * experimental.const_params\n *\n * Syntax to follow\n *\n * @flow\n */\n\nfunction cannot_reassign(x: string) {\n  x = \"hey\"; // error, const param cannot be reassigned\n}\n\n// Note: const params use the same machinery as explicit\n// const bindings, which are tested more extensively elsewhere.\n// Here we're just making sure the machinery is hooked up.\n//\nfunction durable_refi(x: ?number) {\n  if (x) {\n    // ok: if x is truthy here, it's truthy everywhere\n    return () => {\n      var y: number = x;\n    };\n  }\n}");
}
