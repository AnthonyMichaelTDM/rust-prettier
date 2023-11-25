#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_alias_key_yml_bracket_spacingfalse_format_1_d35d737d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]")?;
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
    Ok(())
}
#[test]
fn test_alias_key_yml_tab_width_4_format_1_d35d737d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]")?;
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
    Ok(())
}
#[test]
fn test_alias_key_yml_format_1_d35d737d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[&123 foo, *123 : 456]")?;
    assert_eq!(formatted, "[&123 foo, *123 : 456]");
    Ok(())
}
#[test]
fn test_array_key_yml_bracket_spacingfalse_format_1_cc359025() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_key_yml_tab_width_4_format_1_cc359025() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
    Ok(())
}
#[test]
fn test_array_key_yml_format_1_cc359025() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_bracket_spacingfalse_format_1_836842c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_tab_width_4_format_1_836842c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_format_1_836842c1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_plain_yml_bracket_spacingfalse_format_1_955bea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n]");
    Ok(())
}
#[test]
fn test_array_plain_yml_tab_width_4_format_1_955bea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_plain_yml_format_1_955bea8d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n]");
    Ok(())
}
#[test]
fn test_array_value_yml_bracket_spacingfalse_format_1_4cb47c30() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_array_value_yml_tab_width_4_format_1_4cb47c30() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n]");
    Ok(())
}
#[test]
fn test_array_value_yml_format_1_4cb47c30() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]]") ? ;
    assert_eq ! (formatted , "[\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n]");
    Ok(())
}
#[test]
fn test_comment_between_yml_bracket_spacingfalse_format_1_75e84a2c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]")?;
    assert_eq!(formatted, "[\n  ? 123\n  # comment\n  : 456,\n]");
    Ok(())
}
#[test]
fn test_comment_between_yml_tab_width_4_format_1_75e84a2c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]")?;
    assert_eq!(formatted, "[\n    ? 123\n    # comment\n    : 456,\n]");
    Ok(())
}
#[test]
fn test_comment_between_yml_format_1_75e84a2c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n? 123\n# comment\n: 456\n]")?;
    assert_eq!(formatted, "[\n  ? 123\n  # comment\n  : 456,\n]");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_bracket_spacingfalse_format_1_31addbc0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]")?;
    assert_eq!(formatted, "[\n  123, # comment\n]");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_tab_width_4_format_1_31addbc0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]")?;
    assert_eq!(formatted, "[\n    123, # comment\n]");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_format_1_31addbc0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[123, # comment\n]")?;
    assert_eq!(formatted, "[\n  123, # comment\n]");
    Ok(())
}
#[test]
fn test_empty_yml_bracket_spacingfalse_format_1_8b8d4984() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]")?;
    assert_eq!(formatted, "[]");
    Ok(())
}
#[test]
fn test_empty_yml_tab_width_4_format_1_8b8d4984() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]")?;
    assert_eq!(formatted, "[]");
    Ok(())
}
#[test]
fn test_empty_yml_format_1_8b8d4984() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[]")?;
    assert_eq!(formatted, "[]");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_bracket_spacingfalse_format_1_c02c75c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]")?;
    assert_eq!(formatted, "[: ]");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_tab_width_4_format_1_c02c75c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]")?;
    assert_eq!(formatted, "[: ]");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_format_1_c02c75c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ : ]")?;
    assert_eq!(formatted, "[: ]");
    Ok(())
}
#[test]
fn test_empty_line_yml_bracket_spacingfalse_format_1_e65da9f7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ? ;
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
    Ok(())
}
#[test]
fn test_empty_line_yml_tab_width_4_format_1_e65da9f7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ? ;
    assert_eq ! (formatted , "[\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
    Ok(())
}
#[test]
fn test_empty_line_yml_format_1_e65da9f7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ]") ? ;
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n]");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_bracket_spacingfalse_format_1_99f6a69b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]")?;
    assert_eq!(formatted, "[aaa, bbb]");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_tab_width_4_format_1_99f6a69b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]")?;
    assert_eq!(formatted, "[aaa, bbb]");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_format_1_99f6a69b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[ aaa, \n\nbbb ]")?;
    assert_eq!(formatted, "[aaa, bbb]");
    Ok(())
}
#[test]
fn test_long_key_yml_bracket_spacingfalse_format_1_6a3eebdd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_key_yml_tab_width_4_format_1_6a3eebdd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_key_yml_format_1_6a3eebdd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  ? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_bracket_spacingfalse_format_1_d0bb165d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_tab_width_4_format_1_d0bb165d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_format_1_d0bb165d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_plain_yml_bracket_spacingfalse_format_1_6c6f2b0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_plain_yml_tab_width_4_format_1_6c6f2b0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_plain_yml_format_1_6c6f2b0c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_value_yml_bracket_spacingfalse_format_1_fe9ed687() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_value_yml_tab_width_4_format_1_fe9ed687() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_long_value_yml_format_1_fe9ed687() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong]") ? ;
    assert_eq ! (formatted , "[\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  : longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n]");
    Ok(())
}
#[test]
fn test_middle_comment_yml_bracket_spacingfalse_format_1_fc15724e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]")?;
    assert_eq!(formatted, "!!set # comment\n[]");
    Ok(())
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_fc15724e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]")?;
    assert_eq!(formatted, "!!set # comment\n[]");
    Ok(())
}
#[test]
fn test_middle_comment_yml_format_1_fc15724e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment\n[]")?;
    assert_eq!(formatted, "!!set # comment\n[]");
    Ok(())
}
#[test]
fn test_middle_comments_yml_bracket_spacingfalse_format_1_45de99d7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]")?;
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
    Ok(())
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_45de99d7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]")?;
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
    Ok(())
}
#[test]
fn test_middle_comments_yml_format_1_45de99d7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set # comment 1\n# comment 2\n[]")?;
    assert_eq!(formatted, "!!set\n# comment 1\n# comment 2\n[]");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_bracket_spacingfalse_format_1_266f7600() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ? ;
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n]");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_tab_width_4_format_1_266f7600() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ? ;
    assert_eq ! (formatted , "[\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n    c: 123,\n]");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_format_1_266f7600() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n]") ? ;
    assert_eq ! (formatted , "[\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n]");
    Ok(())
}
#[test]
fn test_props_yml_bracket_spacingfalse_format_1_ef8627ad() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]")?;
    assert_eq!(formatted, "!!set &anchor [1]");
    Ok(())
}
#[test]
fn test_props_yml_tab_width_4_format_1_ef8627ad() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]")?;
    assert_eq!(formatted, "!!set &anchor [1]");
    Ok(())
}
#[test]
fn test_props_yml_format_1_ef8627ad() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!set &anchor [1]")?;
    assert_eq!(formatted, "!!set &anchor [1]");
    Ok(())
}
#[test]
fn test_props_in_map_yml_bracket_spacingfalse_format_1_7886b543() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]")?;
    assert_eq!(formatted, "a: !!set &anchor [1]");
    Ok(())
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_7886b543() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]")?;
    assert_eq!(formatted, "a: !!set &anchor [1]");
    Ok(())
}
#[test]
fn test_props_in_map_yml_format_1_7886b543() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!set &anchor [1]")?;
    assert_eq!(formatted, "a: !!set &anchor [1]");
    Ok(())
}
#[test]
fn test_short_key_yml_bracket_spacingfalse_format_1_6b181bab() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]")?;
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
    Ok(())
}
#[test]
fn test_short_key_yml_tab_width_4_format_1_6b181bab() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]")?;
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
    Ok(())
}
#[test]
fn test_short_key_yml_format_1_6b181bab() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[? 1,? 2,? 3]")?;
    assert_eq!(formatted, "[? 1, ? 2, ? 3]");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_bracket_spacingfalse_format_1_b945f02e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]")?;
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_tab_width_4_format_1_b945f02e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]")?;
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_format_1_b945f02e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1: a,2: b,3: c]")?;
    assert_eq!(formatted, "[1: a, 2: b, 3: c]");
    Ok(())
}
#[test]
fn test_short_plain_yml_bracket_spacingfalse_format_1_a83ceeb2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]")?;
    assert_eq!(formatted, "[1, 2, 3]");
    Ok(())
}
#[test]
fn test_short_plain_yml_tab_width_4_format_1_a83ceeb2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]")?;
    assert_eq!(formatted, "[1, 2, 3]");
    Ok(())
}
#[test]
fn test_short_plain_yml_format_1_a83ceeb2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[1,2,3]")?;
    assert_eq!(formatted, "[1, 2, 3]");
    Ok(())
}
#[test]
fn test_short_value_yml_bracket_spacingfalse_format_1_a0bb1606() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]")?;
    assert_eq!(formatted, "[: 1, : 2, : 3]");
    Ok(())
}
#[test]
fn test_short_value_yml_tab_width_4_format_1_a0bb1606() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]")?;
    assert_eq!(formatted, "[: 1, : 2, : 3]");
    Ok(())
}
#[test]
fn test_short_value_yml_format_1_a0bb1606() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[: 1,: 2,: 3]")?;
    assert_eq!(formatted, "[: 1, : 2, : 3]");
    Ok(())
}
