#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_alias_key_yml_bracket_spacingfalse_format_1_d35d737d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
}
#[test]
fn test_alias_key_yml_tab_width_4_format_1_d35d737d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
}
#[test]
fn test_alias_key_yml_format_1_d35d737d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
}
#[test]
fn test_array_key_yml_bracket_spacingfalse_format_1_cc359025() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_key_yml_tab_width_4_format_1_cc359025() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
}
#[test]
fn test_array_key_yml_format_1_cc359025() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_key_array_value_yml_bracket_spacingfalse_format_1_836842c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_key_array_value_yml_tab_width_4_format_1_836842c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
}
#[test]
fn test_array_key_array_value_yml_format_1_836842c1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_plain_yml_bracket_spacingfalse_format_1_955bea8d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n]");
}
#[test]
fn test_array_plain_yml_tab_width_4_format_1_955bea8d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_plain_yml_format_1_955bea8d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n]");
}
#[test]
fn test_array_value_yml_bracket_spacingfalse_format_1_4cb47c30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_array_value_yml_tab_width_4_format_1_4cb47c30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
}
#[test]
fn test_array_value_yml_format_1_4cb47c30() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
}
#[test]
fn test_comment_between_yml_bracket_spacingfalse_format_1_75e84a2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n  ? 123\n  # comment\n  : 456,\n]");
}
#[test]
fn test_comment_between_yml_tab_width_4_format_1_75e84a2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n    ? 123\n    # comment\n    : 456,\n]");
}
#[test]
fn test_comment_between_yml_format_1_75e84a2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n  ? 123\n  # comment\n  : 456,\n]");
}
#[test]
fn test_comment_trailing_yml_bracket_spacingfalse_format_1_31addbc0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n  123, # comment\n]");
}
#[test]
fn test_comment_trailing_yml_tab_width_4_format_1_31addbc0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n    123, # comment\n]");
}
#[test]
fn test_comment_trailing_yml_format_1_31addbc0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n  123, # comment\n]");
}
#[test]
fn test_empty_yml_bracket_spacingfalse_format_1_8b8d4984() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[]");
}
#[test]
fn test_empty_yml_tab_width_4_format_1_8b8d4984() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[]");
}
#[test]
fn test_empty_yml_format_1_8b8d4984() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[]");
}
#[test]
fn test_empty_item_colon_yml_bracket_spacingfalse_format_1_c02c75c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: ]");
}
#[test]
fn test_empty_item_colon_yml_tab_width_4_format_1_c02c75c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: ]");
}
#[test]
fn test_empty_item_colon_yml_format_1_c02c75c9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: ]");
}
#[test]
fn test_empty_line_yml_bracket_spacingfalse_format_1_e65da9f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
}
#[test]
fn test_empty_line_yml_tab_width_4_format_1_e65da9f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
}
#[test]
fn test_empty_line_yml_format_1_e65da9f7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
}
#[test]
fn test_empty_line_collapse_yml_bracket_spacingfalse_format_1_99f6a69b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[aaa, bbb]");
}
#[test]
fn test_empty_line_collapse_yml_tab_width_4_format_1_99f6a69b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .tab_width(4)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[aaa, bbb]");
}
#[test]
fn test_empty_line_collapse_yml_format_1_99f6a69b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[aaa, bbb]");
}
#[test]
fn test_long_key_yml_bracket_spacingfalse_format_1_6a3eebdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_key_yml_tab_width_4_format_1_6a3eebdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .tab_width(4)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_key_yml_format_1_6a3eebdd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_key_long_value_yml_bracket_spacingfalse_format_1_d0bb165d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_key_long_value_yml_tab_width_4_format_1_d0bb165d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_key_long_value_yml_format_1_d0bb165d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_plain_yml_bracket_spacingfalse_format_1_6c6f2b0c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_plain_yml_tab_width_4_format_1_6c6f2b0c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_plain_yml_format_1_6c6f2b0c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_value_yml_bracket_spacingfalse_format_1_fe9ed687() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .bracket_spacing(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_value_yml_tab_width_4_format_1_fe9ed687() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .tab_width(4)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_long_value_yml_format_1_fe9ed687() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
}
#[test]
fn test_middle_comment_yml_bracket_spacingfalse_format_1_fc15724e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set # comment\n[]");
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_fc15724e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set # comment\n[]");
}
#[test]
fn test_middle_comment_yml_format_1_fc15724e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set # comment\n[]");
}
#[test]
fn test_middle_comments_yml_bracket_spacingfalse_format_1_45de99d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_45de99d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .tab_width(4)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
}
#[test]
fn test_middle_comments_yml_format_1_45de99d7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
}
#[test]
fn test_next_empty_line_yml_bracket_spacingfalse_format_1_266f7600() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n]");
}
#[test]
fn test_next_empty_line_yml_tab_width_4_format_1_266f7600() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .tab_width(4)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n    c: 123,\n]");
}
#[test]
fn test_next_empty_line_yml_format_1_266f7600() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n]");
}
#[test]
fn test_props_yml_bracket_spacingfalse_format_1_ef8627ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set &anchor [1]");
}
#[test]
fn test_props_yml_tab_width_4_format_1_ef8627ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set &anchor [1]");
}
#[test]
fn test_props_yml_format_1_ef8627ad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "!!set &anchor [1]");
}
#[test]
fn test_props_in_map_yml_bracket_spacingfalse_format_1_7886b543() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .bracket_spacing(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!set &anchor [1]");
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_7886b543() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!set &anchor [1]");
}
#[test]
fn test_props_in_map_yml_format_1_7886b543() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a: !!set &anchor [1]");
}
#[test]
fn test_short_key_yml_bracket_spacingfalse_format_1_6b181bab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
}
#[test]
fn test_short_key_yml_tab_width_4_format_1_6b181bab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
}
#[test]
fn test_short_key_yml_format_1_6b181bab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
}
#[test]
fn test_short_key_short_value_yml_bracket_spacingfalse_format_1_b945f02e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
}
#[test]
fn test_short_key_short_value_yml_tab_width_4_format_1_b945f02e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .tab_width(4)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
}
#[test]
fn test_short_key_short_value_yml_format_1_b945f02e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
}
#[test]
fn test_short_plain_yml_bracket_spacingfalse_format_1_a83ceeb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .bracket_spacing(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1, 2, 3]");
}
#[test]
fn test_short_plain_yml_tab_width_4_format_1_a83ceeb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1, 2, 3]");
}
#[test]
fn test_short_plain_yml_format_1_a83ceeb2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[1, 2, 3]");
}
#[test]
fn test_short_value_yml_bracket_spacingfalse_format_1_a0bb1606() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["yaml"])
        .bracket_spacing(false)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: 1, : 2, : 3]");
}
#[test]
fn test_short_value_yml_tab_width_4_format_1_a0bb1606() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: 1, : 2, : 3]");
}
#[test]
fn test_short_value_yml_format_1_a0bb1606() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["yaml"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[: 1, : 2, : 3]");
}
