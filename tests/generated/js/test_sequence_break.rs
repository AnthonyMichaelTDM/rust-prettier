#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_js_format_1_8c03da88() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f = (argument1, argument2, argument3) =>\n  (doSomethingWithArgument(argument1), doSomethingWithArgument(argument2),argument1);\n(function(){\n\treturn aLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName;\n});\naLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName;\na.then(() => (aLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName));\nfor (aLongIdentifierName = 0, aLongIdentifierName = 0, aLongIdentifierName = 0, aLongIdentifierName = 0; test; update) {}\n(a = b ? c : function() { return 0; }),\n  (a = b ? c : function() { return 0; }),\n  (a = b ? c : function() { return 0; }),\n  (a = b ? c : function() { return 0; }),\n  (a = b ? c : function() { return 0; });") ? ;
    assert_eq ! (formatted , "const f = (argument1, argument2, argument3) => (\n  doSomethingWithArgument(argument1),\n  doSomethingWithArgument(argument2),\n  argument1\n);\n(function () {\n  return (\n    aLongIdentifierName,\n    aLongIdentifierName,\n    aLongIdentifierName,\n    aLongIdentifierName\n  );\n});\naLongIdentifierName,\n  aLongIdentifierName,\n  aLongIdentifierName,\n  aLongIdentifierName;\na.then(\n  () => (\n    aLongIdentifierName,\n    aLongIdentifierName,\n    aLongIdentifierName,\n    aLongIdentifierName\n  ),\n);\nfor (\n  aLongIdentifierName = 0,\n    aLongIdentifierName = 0,\n    aLongIdentifierName = 0,\n    aLongIdentifierName = 0;\n  test;\n  update\n) {}\n(a = b\n  ? c\n  : function () {\n      return 0;\n    }),\n  (a = b\n    ? c\n    : function () {\n        return 0;\n      }),\n  (a = b\n    ? c\n    : function () {\n        return 0;\n      }),\n  (a = b\n    ? c\n    : function () {\n        return 0;\n      }),\n  (a = b\n    ? c\n    : function () {\n        return 0;\n      });");
    Ok(())
}
