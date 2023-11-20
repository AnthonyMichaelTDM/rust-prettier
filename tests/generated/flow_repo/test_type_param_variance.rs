#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_promise_js_format_1_a6ef3e84() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * At the moment, all type params are invariant with\n * the exception of the single param to the Promise class,\n * which is covariant.\n *\n * Explicit variance control via annotation is coming,\n * but not immediately. In the meantime, Promise's\n * participation in async/await makes certain kinds of\n * errors onerous (and nonobvious) without covariance.\n *\n * @flow\n */\n\nasync function foo(x: boolean): Promise<?{bar: string}> {\n  if (x) {\n    return {bar: 'baz'};  // OK, because of covariant type param\n  } else {\n    return null;\n  }\n}\n\nasync function run() {\n  console.log(await foo(true));\n  console.log(await foo(false));\n}\n\nrun()") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * At the moment, all type params are invariant with\n * the exception of the single param to the Promise class,\n * which is covariant.\n *\n * Explicit variance control via annotation is coming,\n * but not immediately. In the meantime, Promise's\n * participation in async/await makes certain kinds of\n * errors onerous (and nonobvious) without covariance.\n *\n * @flow\n */\n\nasync function foo(x: boolean): Promise<?{ bar: string }> {\n  if (x) {\n    return { bar: \"baz\" }; // OK, because of covariant type param\n  } else {\n    return null;\n  }\n}\n\nasync function run() {\n  console.log(await foo(true));\n  console.log(await foo(false));\n}\n\nrun();");
}
