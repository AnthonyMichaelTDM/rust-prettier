use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_8d61d2d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {}\nconst args = [3, 4];\n\nfoo(1, 2); // 2 errors\nfoo(\n  1, // error\n  2, // error\n);\nfoo(...args); // 2 errors\n\nfoo.call(null, 1, 2); // 2 errors\nfoo.call(null, ...args); // 2 errors\nfoo.call(null, 1, 2, ...args); // 4 errors\n\nfoo.apply(null, [1, 2]); // 2 errors\nfoo.apply(null, args); // 2 errors\n\nfoo.bind(null, 1, 2); // 2 errors\nfoo.bind(null, ...args); // 2 errors\nfoo.bind(null, 1, 2, ...args); // 4 errors\n\nnew foo(1, 2); // 2 errors\nnew foo(...args); // 2 errors\nnew foo(1, 2, ...args); // 4 errors") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function foo() {}\nconst args = [3, 4];\n\nfoo(1, 2); // 2 errors\nfoo(\n  1, // error\n  2, // error\n);\nfoo(...args); // 2 errors\n\nfoo.call(null, 1, 2); // 2 errors\nfoo.call(null, ...args); // 2 errors\nfoo.call(null, 1, 2, ...args); // 4 errors\n\nfoo.apply(null, [1, 2]); // 2 errors\nfoo.apply(null, args); // 2 errors\n\nfoo.bind(null, 1, 2); // 2 errors\nfoo.bind(null, ...args); // 2 errors\nfoo.bind(null, 1, 2, ...args); // 4 errors\n\nnew foo(1, 2); // 2 errors\nnew foo(...args); // 2 errors\nnew foo(1, 2, ...args); // 4 errors");
}
