#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_promise_js_format_1_fcdac82b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Here the definition of Promise<T> is routed to the class Promise\n * in the user-specified library libs/Promise.js\n *\n * In such situations we must desugar async/await primitives\n * to the shadowed library definition.\n *\n * @flow\n */\n\nasync function foo(x: boolean): Promise<?{bar: string}> {\n  if (x) {\n    return {bar: 'baz'};  // OK, because of covariant type param\n  } else {\n    return null;\n  }\n}\n\nasync function run() {\n  console.log(await foo(true));\n  console.log(await foo(false));\n}\n\nrun()") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Here the definition of Promise<T> is routed to the class Promise\n * in the user-specified library libs/Promise.js\n *\n * In such situations we must desugar async/await primitives\n * to the shadowed library definition.\n *\n * @flow\n */\n\nasync function foo(x: boolean): Promise<?{ bar: string }> {\n  if (x) {\n    return { bar: \"baz\" }; // OK, because of covariant type param\n  } else {\n    return null;\n  }\n}\n\nasync function run() {\n  console.log(await foo(true));\n  console.log(await foo(false));\n}\n\nrun();");
}
