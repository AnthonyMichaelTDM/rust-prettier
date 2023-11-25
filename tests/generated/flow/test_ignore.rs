#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_ignore_js_format_1_26c794ac() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function a() {\n  // Incorrectly indented on purpose\n      function f</* prettier-ignore */ T    :    B>(\n        a : Array  <   number   > // prettier-ignore\n      ) {\n\n        call(\n          f(         1          )\n          // prettier-ignore\n        )\n      }\n}") ? ;
    assert_eq ! (formatted , "function a() {\n  // Incorrectly indented on purpose\n  function f</* prettier-ignore */ T    :    B>(\n    a : Array  <   number   >, // prettier-ignore\n  ) {\n    call(\n      f(         1          ),\n      // prettier-ignore\n    );\n  }\n}");
    Ok(())
}
#[test]
fn test_type_cast_expression_js_format_1_e251dae2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("transform(\n  // prettier-ignore\n  (pointTransformer: T)\n);\n\ntransform(\n  // prettier-ignore\n  (/* comment */pointTransformer: T /* comment */)\n);\n\ntransform(\n  /* prettier-ignore */(/* prettier-ignore */pointTransformer: (Point => Point))\n);") ? ;
    assert_eq ! (formatted , "transform(\n  // prettier-ignore\n  (pointTransformer: T),\n);\n\ntransform(\n  // prettier-ignore\n  (/* comment */pointTransformer: T /* comment */),\n);\n\ntransform(\n  /* prettier-ignore */ (/* prettier-ignore */pointTransformer: (Point => Point)),\n);");
    Ok(())
}
