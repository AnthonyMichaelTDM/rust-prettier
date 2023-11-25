use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_for_await_js_format_1_7677d32b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function foo() {\n  for await (num of asyncIterable) {\n    console.log(num);\n  }\n  for await (num of asyncGeneratorFunc()) {\n    console.log(num);\n  }\n}\n\n(async () => {\n  for await (num of asyncIterable) {\n    console.log(num);\n  }\n  for await (const x of delegate_yield()) {\n    x;\n  }\n})();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function foo() {\n  for await (num of asyncIterable) {\n    console.log(num);\n  }\n  for await (num of asyncGeneratorFunc()) {\n    console.log(num);\n  }\n}\n\n(async () => {\n  for await (num of asyncIterable) {\n    console.log(num);\n  }\n  for await (const x of delegate_yield()) {\n    x;\n  }\n})();");
}
