#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_issue_518_js_format_1_af5a50a9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function doSomethingAsync(): Promise<void> {\n  return new Promise((resolve, reject) => {\n    resolve(); // OK to leave out arg, same as resolve(undefined)\n\n    var anotherVoidPromise: Promise<void> = Promise.resolve();\n    resolve(anotherVoidPromise);\n  });\n}\n\n// simpler repro to show that too few args are fine when expecting void\nfunction foo(x: void) { }\nfoo();") ? ;
    assert_eq ! (formatted , "function doSomethingAsync(): Promise<void> {\n  return new Promise((resolve, reject) => {\n    resolve(); // OK to leave out arg, same as resolve(undefined)\n\n    var anotherVoidPromise: Promise<void> = Promise.resolve();\n    resolve(anotherVoidPromise);\n  });\n}\n\n// simpler repro to show that too few args are fine when expecting void\nfunction foo(x: void) {}\nfoo();");
    Ok(())
}
#[test]
fn test_undefined_js_format_1_ef782513() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function foo() {\n    var x;\n    x.foo();\n}\n\nfunction bar() {\n    var x:?{ bar():void; };\n    if (x) x.bar();\n}\n\nfunction qux(x?: number, y:string = \"\", z) { }") ? ;
    assert_eq ! (formatted , "function foo() {\n  var x;\n  x.foo();\n}\n\nfunction bar() {\n  var x: ?{ bar(): void };\n  if (x) x.bar();\n}\n\nfunction qux(x?: number, y: string = \"\", z) {}");
    Ok(())
}
#[test]
fn test_undefined_2_js_format_1_e16b4a31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  function(x: number) {\n    var id;\n    var name = id ? 'John' : undefined;\n    (name: boolean); // error, string or void\n\n    const bar = [\n      undefined,\n      'bar',\n    ];\n    (bar[x]: boolean); // error, string or void\n  },\n\n  function(x: number) {\n    var undefined = 'foo';\n    (undefined: string); // ok\n\n    var x;\n    if (x !== undefined) {\n      x[0]; // should error, could be void\n    }\n\n    const bar = [\n      undefined,\n      'bar',\n    ];\n    (bar[x]: boolean); // error, string only\n  },\n];") ? ;
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  function (x: number) {\n    var id;\n    var name = id ? \"John\" : undefined;\n    (name: boolean); // error, string or void\n\n    const bar = [undefined, \"bar\"];\n    (bar[x]: boolean); // error, string or void\n  },\n\n  function (x: number) {\n    var undefined = \"foo\";\n    (undefined: string); // ok\n\n    var x;\n    if (x !== undefined) {\n      x[0]; // should error, could be void\n    }\n\n    const bar = [undefined, \"bar\"];\n    (bar[x]: boolean); // error, string only\n  },\n];");
    Ok(())
}
