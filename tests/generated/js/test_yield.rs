#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arrow_js_format_1_c5d17c97() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "function *f() {\n  (yield a => a);\n  (yield async a => a);\n  (yield async (a) => a);\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function* f() {\n  yield (a) => a;\n  yield async (a) => a;\n  yield async (a) => a;\n}"
    );
}
#[test]
fn test_conditional_js_format_1_be876c7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function* f1() {\n  a = (yield) ? 1 : 1;\n  a = yield 1 ? 1 : 1;\n  a = (yield 1) ? 1 : 1;\n  a = 1 ? yield : yield;\n  a = 1 ? yield 1 : yield 1;\n}\n\nfunction* f2() {\n  a = yield* 1 ? 1 : 1;\n  a = (yield* 1) ? 1 : 1;\n  a = 1 ? yield* 1 : yield* 1;\n}\n\nasync function f3() {\n  a = await 1 ? 1 : 1;\n  a = (await 1) ? 1 : 1;\n  a = 1 ? await 1 : await 1;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function* f1() {\n  a = (yield) ? 1 : 1;\n  a = yield 1 ? 1 : 1;\n  a = (yield 1) ? 1 : 1;\n  a = 1 ? yield : yield;\n  a = 1 ? yield 1 : yield 1;\n}\n\nfunction* f2() {\n  a = yield* 1 ? 1 : 1;\n  a = (yield* 1) ? 1 : 1;\n  a = 1 ? yield* 1 : yield* 1;\n}\n\nasync function f3() {\n  a = (await 1) ? 1 : 1;\n  a = (await 1) ? 1 : 1;\n  a = 1 ? await 1 : await 1;\n}");
}
#[test]
fn test_jsx_js_format_1_1abd30e9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "function* f() {\n  yield (<div>generator</div>)\n  yield (<div><p>generator</p></div>)\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function* f() {\n  yield <div>generator</div>;\n  yield (\n    <div>\n      <p>generator</p>\n    </div>\n  );\n}");
}
#[test]
fn test_jsx_without_parenthesis_js_format_1_0ee3e64d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "function* f() {\n  yield <div>generator</div>\n  yield <div><p>generator</p></div>\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function* f() {\n  yield <div>generator</div>;\n  yield (\n    <div>\n      <p>generator</p>\n    </div>\n  );\n}");
}
