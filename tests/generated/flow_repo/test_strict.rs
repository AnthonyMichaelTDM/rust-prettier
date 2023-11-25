#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_annot_js_format_1_67f48657() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var A = require('./unknown_class');\n\nclass B extends A {\n  foo(x:A):A { return x; }\n  bar(x) { }\n  qux<X,Y>(x:X,y:Y):X { return x;}\n}\n\nmodule.exports = B;") ? ;
    assert_eq ! (formatted , "var A = require(\"./unknown_class\");\n\nclass B extends A {\n  foo(x: A): A {\n    return x;\n  }\n  bar(x) {}\n  qux<X, Y>(x: X, y: Y): X {\n    return x;\n  }\n}\n\nmodule.exports = B;");
    Ok(())
}
#[test]
fn test_fun_js_format_1_c2ee41af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n// progressively annotate:\n\n//function f(x) { return x; }\nfunction f(x:number) { return x; }\n//function f(x:number):string { return x; }\n\nvar x:string = f(0);\n\nmodule.exports = f;") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n// progressively annotate:\n\n//function f(x) { return x; }\nfunction f(x: number) {\n  return x;\n}\n//function f(x:number):string { return x; }\n\nvar x: string = f(0);\n\nmodule.exports = f;");
    Ok(())
}
#[test]
fn test_obj_js_format_1_3c7bccf7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\n// progressively annotate:\n\nvar o = { x: 0 }\n//var o: {x: number;} = { x: 0 }\n\nvar x:string = o.x;\n\nmodule.exports = o;") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\n// progressively annotate:\n\nvar o = { x: 0 };\n//var o: {x: number;} = { x: 0 }\n\nvar x: string = o.x;\n\nmodule.exports = o;");
    Ok(())
}
