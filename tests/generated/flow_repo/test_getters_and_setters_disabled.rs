use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_getters_and_setters_js_format_1_a76b4f25() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nvar f = {\n  get a() { return 4; },\n  set b(x: number) { this.c = x; },\n  c: 10,\n  get ['d']() { return 'foo'; },\n  set ['d'](x: number) {},\n};\n\ntype T = {\n  get a(): number,\n  set b(x: number): void,\n  c: 10,\n}\n\ndeclare class Foo {\n  get a(): number;\n  set b(x: number): void;\n  c: 10;\n}\n\nclass Bar {\n  get a() { return 4; }\n  set b(x: number) { this.c = x; }\n  c: number;\n  get ['d']() { return 'foo'; }\n  set ['d'](x: number) {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nvar f = {\n  get a() {\n    return 4;\n  },\n  set b(x: number) {\n    this.c = x;\n  },\n  c: 10,\n  get [\"d\"]() {\n    return \"foo\";\n  },\n  set [\"d\"](x: number) {},\n};\n\ntype T = {\n  get a(): number,\n  set b(x: number): void,\n  c: 10,\n};\n\ndeclare class Foo {\n  get a(): number;\n  set b(x: number): void;\n  c: 10;\n}\n\nclass Bar {\n  get a() {\n    return 4;\n  }\n  set b(x: number) {\n    this.c = x;\n  }\n  c: number;\n  get [\"d\"]() {\n    return \"foo\";\n  }\n  set [\"d\"](x: number) {}\n}");
}
