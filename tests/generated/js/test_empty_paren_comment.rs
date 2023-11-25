#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_format_1_c5c2a6a5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class x {\n  /**\n  * Set of default settings to be applied to model fetch calls in DAO layer.\n  */\n  static get defaultSettings() {\n  }\n}") ? ;
    assert_eq ! (formatted , "class x {\n  /**\n   * Set of default settings to be applied to model fetch calls in DAO layer.\n   */\n  static get defaultSettings() {}\n}");
    Ok(())
}
#[test]
fn test_class_property_js_format_1_843e6730() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  f(/* ... */) {}\n  f() /* ... */ {}\n  f = (/* ... */) => {};\n  static f(/* ... */) {};\n  static f = (/* ... */) => {};\n  static f = function(/* ... */) {};\n  static f = function f(/* ... */) {};\n}") ? ;
    assert_eq ! (formatted , "class Foo {\n  f(/* ... */) {}\n  f() /* ... */ {}\n  f = (/* ... */) => {};\n  static f(/* ... */) {}\n  static f = (/* ... */) => {};\n  static f = function (/* ... */) {};\n  static f = function f(/* ... */) {};\n}");
    Ok(())
}
#[test]
fn test_empty_paren_comment_js_format_1_fdfe3756() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let f1 = (/* ... */) => {}\n(function (/* ... */) {})(/* ... */)\nfunction f2(/* ... */) {}\n\nconst obj = {\n  f(/* ... */) {},\n  f: (/* ... */) => {},\n  f: function(/* ... */) {},\n  f: function f(/* ... */) {}\n}\n\nf(/* ... */);\nf(a, /* ... */);\nf(a, /* ... */ b);\nf(/* ... */ a, b);\n\nlet f3 = () => import(a /* ... */);\nlet f4 = () => doThing(a, /* ... */ b);") ? ;
    assert_eq ! (formatted , "let f1 = (/* ... */) => {};\n(function (/* ... */) {})(/* ... */);\nfunction f2(/* ... */) {}\n\nconst obj = {\n  f(/* ... */) {},\n  f: (/* ... */) => {},\n  f: function (/* ... */) {},\n  f: function f(/* ... */) {},\n};\n\nf(/* ... */);\nf(a /* ... */);\nf(a, /* ... */ b);\nf(/* ... */ a, b);\n\nlet f3 = () => import(a /* ... */);\nlet f4 = () => doThing(a, /* ... */ b);");
    Ok(())
}
