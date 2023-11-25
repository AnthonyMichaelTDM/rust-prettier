#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_traces_js_format_1_3aa33e46() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\n * The location marked with the FlowFixMe does not show up in the original\n * error but shows up in the flow check --traces 10 result. This test makes\n * sure that we don't suppress the error due to a location that only shows up\n * when --traces is turned on.\n */\n\n// $FlowFixMe - Error unused suppression\nfunction bar(): number { return 5; }\n\nfunction foo(x: string) {\n  return bar();\n}\n\nvar a: string = foo('hi'); // error number ~> string") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * The location marked with the FlowFixMe does not show up in the original\n * error but shows up in the flow check --traces 10 result. This test makes\n * sure that we don't suppress the error due to a location that only shows up\n * when --traces is turned on.\n */\n\n// $FlowFixMe - Error unused suppression\nfunction bar(): number {\n  return 5;\n}\n\nfunction foo(x: string) {\n  return bar();\n}\n\nvar a: string = foo(\"hi\"); // error number ~> string");
}
