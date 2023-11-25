#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_call_js_format_1_42204e30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("new (factory())()\nnew factory()()\n\nnew (factory())(factory())");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "new (factory())();\nnew factory()();\n\nnew (factory())(factory());"
    );
}
#[test]
fn test_new_expression_js_format_1_e213d5a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("new (memoize.Cache || MapCache)\nnew (typeof this == \"function\" ? this : Dict())\nnew (createObj()).prop(a());\nnew (x()\\`\\`.y)();\nnew e[f().x].y();\nnew e[f()].y();\nnew (a().b)();\nnew (a().b().c)();\nnew (a\\`\\`());") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "new (memoize.Cache || MapCache)();\nnew (typeof this == \"function\" ? this : Dict())();\nnew (createObj().prop)(a());\nnew (x()\\`\\`.y)();\nnew e[f().x].y();\nnew e[f()].y();\nnew (a().b)();\nnew (a().b().c)();\nnew (a\\`\\`())();");
}
#[test]
fn test_with_member_expression_js_format_1_a0494e7a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function functionName() {\n  // indent to make the line break\n  if (true) {\n    this._aVeryLongVariableNameToForceLineBreak = new this.Promise(\n      (resolve, reject) => {\n        // do something\n      }\n    );\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function functionName() {\n  // indent to make the line break\n  if (true) {\n    this._aVeryLongVariableNameToForceLineBreak = new this.Promise(\n      (resolve, reject) => {\n        // do something\n      },\n    );\n  }\n}");
}
