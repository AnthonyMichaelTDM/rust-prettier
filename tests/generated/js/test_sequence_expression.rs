#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_export_default_js_format_1_140ca931() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("export default (1,\n2);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export default (1, 2);");
}
#[test]
fn test_ignore_js_format_1_03e308a4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("+\n  // prettier-ignore\n  (\n    (\n      first\n    )\n    ,\n    (\n      last\n    )\n  )\n;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "+(\n  // prettier-ignore\n  ((\n      first\n    )\n    ,\n    (\n      last\n    ))\n);"
    );
}
#[test]
fn test_parenthesized_js_format_1_041db68f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("console.log(\n  /* 1 */\n  (\n    /* 2 */\n    (\n      /* 3 */\n      first\n      /* 4 */\n    )\n    /* 5 */\n    ,\n    /* 6 */\n    (\n      /* 7 */\n      last\n      /* 8 */\n    )\n    /* 9 */\n  )\n  /* 10 */\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "console.log(\n  /* 1 */\n  /* 2 */\n  (/* 3 */\n  first,\n  /* 4 */\n  /* 5 */\n  /* 6 */\n  /* 7 */\n  last),\n  /* 8 */\n  /* 9 */\n  /* 10 */\n);");
}
