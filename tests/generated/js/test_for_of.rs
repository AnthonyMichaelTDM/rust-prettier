use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_identifier_js_format_1_c3eb4018() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("for ((async) of []);\nfor ((foo) of async);\nfor ((foo) of []) async;\n\nasync function f() {\n  for await (async of []);\n  for await ((async) of []);\n  for await ((foo) of async);\n  for await ((foo) of []) async;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "for ((async) of []);\nfor (foo of async);\nfor (foo of []) async;\n\nasync function f() {\n  for await (async of []);\n  for await (async of []);\n  for await (foo of async);\n  for await (foo of []) async;\n}");
}
