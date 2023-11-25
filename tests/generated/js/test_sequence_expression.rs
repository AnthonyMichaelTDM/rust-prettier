#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_default_js_format_1_140ca931() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (1,\n2);")?;
    assert_eq!(formatted, "export default (1, 2);");
    Ok(())
}
#[test]
fn test_ignore_js_format_1_03e308a4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("+\n  // prettier-ignore\n  (\n    (\n      first\n    )\n    ,\n    (\n      last\n    )\n  )\n;") ? ;
    assert_eq!(
        formatted,
        "+(\n  // prettier-ignore\n  ((\n      first\n    )\n    ,\n    (\n      last\n    ))\n);"
    );
    Ok(())
}
#[test]
fn test_parenthesized_js_format_1_041db68f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("console.log(\n  /* 1 */\n  (\n    /* 2 */\n    (\n      /* 3 */\n      first\n      /* 4 */\n    )\n    /* 5 */\n    ,\n    /* 6 */\n    (\n      /* 7 */\n      last\n      /* 8 */\n    )\n    /* 9 */\n  )\n  /* 10 */\n);") ? ;
    assert_eq ! (formatted , "console.log(\n  /* 1 */\n  /* 2 */\n  (/* 3 */\n  first,\n  /* 4 */\n  /* 5 */\n  /* 6 */\n  /* 7 */\n  last),\n  /* 8 */\n  /* 9 */\n  /* 10 */\n);");
    Ok(())
}
