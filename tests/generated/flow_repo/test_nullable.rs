use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_maybe_js_format_1_bdb03925() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\n// unwrapping nested maybes should work\n(('foo': ?(?string)): ?string); // ok\n((123: ?(?number)): ?string); // error (only num ~> string)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\n// unwrapping nested maybes should work\n((\"foo\": ??string): ?string); // ok\n((123: ??number): ?string); // error (only num ~> string)");
}
#[test]
fn test_nullable_js_format_1_181d2d62() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo():string { return null; }\n\nfunction bar():?string { return null; }\n\nfunction qux(x:string) { }\n\nfunction corge(x:number) { }\n\nvar x = bar(); // x: ?string\nif (x != null) qux(x); // x: ?string | null\nif (x != null) corge(x); // x: ?string | null\n\nfunction grault() { x = null; }\nif (x != null) {\n  grault(); qux(x);\n}\n\nvar array_of_nullable: Array<?number> = [null, 3];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(): string {\n  return null;\n}\n\nfunction bar(): ?string {\n  return null;\n}\n\nfunction qux(x: string) {}\n\nfunction corge(x: number) {}\n\nvar x = bar(); // x: ?string\nif (x != null) qux(x); // x: ?string | null\nif (x != null) corge(x); // x: ?string | null\n\nfunction grault() {\n  x = null;\n}\nif (x != null) {\n  grault();\n  qux(x);\n}\n\nvar array_of_nullable: Array<?number> = [null, 3];");
}
#[test]
fn test_simple_nullable_js_format_1_3384e72b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo(x:?string) {}\nfunction bar(x:?number) {}\nfoo('hmm');\nbar('hmm');\n\nfunction fn(data: ?{}) {}\nfn({some: 'literal'});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo(x: ?string) {}\nfunction bar(x: ?number) {}\nfoo(\"hmm\");\nbar(\"hmm\");\n\nfunction fn(data: ?{}) {}\nfn({ some: \"literal\" });");
}
