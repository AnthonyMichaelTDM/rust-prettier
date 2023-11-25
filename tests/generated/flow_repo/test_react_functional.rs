#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_921c35dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\nfunction F(props: { foo: string }) {}\n<F />; // error: missing \\`foo\\`\n<F foo={0} />; // error: number ~> string\n<F foo=\"\" />; // ok\n\n// props subtyping is property-wise covariant\nfunction G(props: { foo: string|numner }) {}\n<G foo=\"\" />; // ok\n\nvar Z = 0;\n<Z />; // error, expected React component") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\nfunction F(props: { foo: string }) {}\n<F />; // error: missing \\`foo\\`\n<F foo={0} />; // error: number ~> string\n<F foo=\"\" />; // ok\n\n// props subtyping is property-wise covariant\nfunction G(props: { foo: string | numner }) {}\n<G foo=\"\" />; // ok\n\nvar Z = 0;\n<Z />; // error, expected React component");
}
