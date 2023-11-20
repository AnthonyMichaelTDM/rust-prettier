#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_await_keywords_js_babel_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_await_keywords_js_format_1_30e2b5f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export class C1 {\n  p = await (0);\n}\n\nawait (0);\n\nasync function foo() {\n  function bar(x = await (2)) {}\n}\n\nexport class C2 {\n  p = await 0;\n}\n\nfunction foo(promise) { await (promise); }\n\nfunction a() {\n  return await (1)\n}\n\n() => { await (x) };\n\nfunction foo() {\n  await\n  (foo);\n}\n\nexport class C {\n  p = await (0);\n}\n\nawait (0);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class C1 {\n  p = await(0);\n}\n\nawait(0);\n\nasync function foo() {\n  function bar(x = await(2)) {}\n}\n\nexport class C2 {\n  p = await;\n  0;\n}\n\nfunction foo(promise) {\n  await(promise);\n}\n\nfunction a() {\n  return await(1);\n}\n\n() => {\n  await(x);\n};\n\nfunction foo() {\n  await(foo);\n}\n\nexport class C {\n  p = await(0);\n}\n\nawait(0);");
}
