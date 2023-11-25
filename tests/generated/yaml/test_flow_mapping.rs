#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_alias_key_yml_bracket_spacingfalse_format_1_bacc9171() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{&123 foo, *123 : 456}")?;
    assert_eq!(formatted, "{&123 foo, *123 : 456}");
    Ok(())
}
#[test]
fn test_alias_key_yml_tab_width_4_format_1_bacc9171() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{&123 foo, *123 : 456}")?;
    assert_eq!(formatted, "{ &123 foo, *123 : 456 }");
    Ok(())
}
#[test]
fn test_alias_key_yml_format_1_bacc9171() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{&123 foo, *123 : 456}")?;
    assert_eq!(formatted, "{ &123 foo, *123 : 456 }");
    Ok(())
}
#[test]
fn test_array_key_yml_bracket_spacingfalse_format_1_f73a8c32() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n}");
    Ok(())
}
#[test]
fn test_array_key_yml_tab_width_4_format_1_f73a8c32() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_array_key_yml_format_1_f73a8c32() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],? [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n}");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_bracket_spacingfalse_format_1_7df4c1b6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_tab_width_4_format_1_7df4c1b6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n    ? [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ]\n    : [\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n          longlonglonglonglonglonglonglonglonglonglong,\n      ],\n}");
    Ok(())
}
#[test]
fn test_array_key_array_value_yml_format_1_7df4c1b6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  ? [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ]\n  : [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_array_plain_yml_bracket_spacingfalse_format_1_cea947c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n}");
    Ok(())
}
#[test]
fn test_array_plain_yml_tab_width_4_format_1_cea947c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n    [\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n        longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_array_plain_yml_format_1_cea947c9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],[longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n  [\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglong,\n  ],\n}");
    Ok(())
}
#[test]
fn test_array_value_yml_bracket_spacingfalse_format_1_cd2b9cb0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{a: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],b: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],c: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  a:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  b:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  c:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_array_value_yml_tab_width_4_format_1_cd2b9cb0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{a: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],b: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],c: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n    a:\n        [\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n        ],\n    b:\n        [\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n        ],\n    c:\n        [\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n            longlonglonglonglonglonglonglonglonglonglong,\n        ],\n}");
    Ok(())
}
#[test]
fn test_array_value_yml_format_1_cd2b9cb0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{a: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],b: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong],c: [longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong, longlonglonglonglonglonglonglonglonglonglong]}") ? ;
    assert_eq ! (formatted , "{\n  a:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  b:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n  c:\n    [\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n      longlonglonglonglonglonglonglonglonglonglong,\n    ],\n}");
    Ok(())
}
#[test]
fn test_comment_between_yml_bracket_spacingfalse_format_1_13363d0b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n? 123\n# comment\n: 456\n}")?;
    assert_eq!(formatted, "{\n  ? 123\n  # comment\n  : 456,\n}");
    Ok(())
}
#[test]
fn test_comment_between_yml_tab_width_4_format_1_13363d0b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n? 123\n# comment\n: 456\n}")?;
    assert_eq!(formatted, "{\n    ? 123\n    # comment\n    : 456,\n}");
    Ok(())
}
#[test]
fn test_comment_between_yml_format_1_13363d0b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n? 123\n# comment\n: 456\n}")?;
    assert_eq!(formatted, "{\n  ? 123\n  # comment\n  : 456,\n}");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_bracket_spacingfalse_format_1_9ccf3005() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{123, # comment\n}")?;
    assert_eq!(formatted, "{\n  123, # comment\n}");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_tab_width_4_format_1_9ccf3005() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{123, # comment\n}")?;
    assert_eq!(formatted, "{\n    123, # comment\n}");
    Ok(())
}
#[test]
fn test_comment_trailing_yml_format_1_9ccf3005() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{123, # comment\n}")?;
    assert_eq!(formatted, "{\n  123, # comment\n}");
    Ok(())
}
#[test]
fn test_empty_yml_bracket_spacingfalse_format_1_5ad5cc4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{}")?;
    assert_eq!(formatted, "{}");
    Ok(())
}
#[test]
fn test_empty_yml_tab_width_4_format_1_5ad5cc4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{}")?;
    assert_eq!(formatted, "{}");
    Ok(())
}
#[test]
fn test_empty_yml_format_1_5ad5cc4d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{}")?;
    assert_eq!(formatted, "{}");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_bracket_spacingfalse_format_1_7ec795ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ : }")?;
    assert_eq!(formatted, "{: }");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_tab_width_4_format_1_7ec795ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ : }")?;
    assert_eq!(formatted, "{ : }");
    Ok(())
}
#[test]
fn test_empty_item_colon_yml_format_1_7ec795ea() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ : }")?;
    assert_eq!(formatted, "{ : }");
    Ok(())
}
#[test]
fn test_empty_line_yml_bracket_spacingfalse_format_1_a85be7af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb }") ? ;
    assert_eq ! (formatted , "{\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n}");
    Ok(())
}
#[test]
fn test_empty_line_yml_tab_width_4_format_1_a85be7af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb }") ? ;
    assert_eq ! (formatted , "{\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n}");
    Ok(())
}
#[test]
fn test_empty_line_yml_format_1_a85be7af() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{ aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, \n\nbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb }") ? ;
    assert_eq ! (formatted , "{\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,\n\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb,\n}");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_bracket_spacingfalse_format_1_5285a77a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ aaa, \n\nbbb }")?;
    assert_eq!(formatted, "{aaa, bbb}");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_tab_width_4_format_1_5285a77a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ aaa, \n\nbbb }")?;
    assert_eq!(formatted, "{ aaa, bbb }");
    Ok(())
}
#[test]
fn test_empty_line_collapse_yml_format_1_5285a77a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{ aaa, \n\nbbb }")?;
    assert_eq!(formatted, "{ aaa, bbb }");
    Ok(())
}
#[test]
fn test_long_key_yml_bracket_spacingfalse_format_1_00ceeb12() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_key_yml_tab_width_4_format_1_00ceeb12() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_key_yml_format_1_00ceeb12() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,? longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_bracket_spacingfalse_format_1_6cdfc861() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_tab_width_4_format_1_6cdfc861() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_long_key_long_value_yml_format_1_6cdfc861() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_long_plain_yml_bracket_spacingfalse_format_1_54f749ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_plain_yml_tab_width_4_format_1_54f749ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n    longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_plain_yml_format_1_54f749ee() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3}") ? ;
    assert_eq ! (formatted , "{\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,\n  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,\n}");
    Ok(())
}
#[test]
fn test_long_value_yml_bracket_spacingfalse_format_1_2820d57d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n  1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_long_value_yml_tab_width_4_format_1_2820d57d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n    1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n    3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_long_value_yml_format_1_2820d57d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong}") ? ;
    assert_eq ! (formatted , "{\n  1: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  2: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n  3: longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglong,\n}");
    Ok(())
}
#[test]
fn test_middle_comment_yml_bracket_spacingfalse_format_1_70fc5beb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map #comment\n{}")?;
    assert_eq!(formatted, "!!map #comment\n{}");
    Ok(())
}
#[test]
fn test_middle_comment_yml_tab_width_4_format_1_70fc5beb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map #comment\n{}")?;
    assert_eq!(formatted, "!!map #comment\n{}");
    Ok(())
}
#[test]
fn test_middle_comment_yml_format_1_70fc5beb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map #comment\n{}")?;
    assert_eq!(formatted, "!!map #comment\n{}");
    Ok(())
}
#[test]
fn test_middle_comments_yml_bracket_spacingfalse_format_1_bb9680b8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment 1\n# comment 2\n{}")?;
    assert_eq!(formatted, "!!map\n# comment 1\n# comment 2\n{}");
    Ok(())
}
#[test]
fn test_middle_comments_yml_tab_width_4_format_1_bb9680b8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment 1\n# comment 2\n{}")?;
    assert_eq!(formatted, "!!map\n# comment 1\n# comment 2\n{}");
    Ok(())
}
#[test]
fn test_middle_comments_yml_format_1_bb9680b8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map # comment 1\n# comment 2\n{}")?;
    assert_eq!(formatted, "!!map\n# comment 1\n# comment 2\n{}");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_bracket_spacingfalse_format_1_3e1edef1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n}") ? ;
    assert_eq ! (formatted , "{\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n}");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_tab_width_4_format_1_3e1edef1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n}") ? ;
    assert_eq ! (formatted , "{\n    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n    bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n    c: 123,\n}");
    Ok(())
}
#[test]
fn test_next_empty_line_yml_format_1_3e1edef1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\naaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123, bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123, \n\nc: 123\n}") ? ;
    assert_eq ! (formatted , "{\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa: 123,\n  bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb: 123,\n\n  c: 123,\n}");
    Ok(())
}
#[test]
fn test_props_yml_bracket_spacingfalse_format_1_5e7feb57() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map &anchor {a: 1}")?;
    assert_eq!(formatted, "!!map &anchor {a: 1}");
    Ok(())
}
#[test]
fn test_props_yml_tab_width_4_format_1_5e7feb57() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map &anchor {a: 1}")?;
    assert_eq!(formatted, "!!map &anchor { a: 1 }");
    Ok(())
}
#[test]
fn test_props_yml_format_1_5e7feb57() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("!!map &anchor {a: 1}")?;
    assert_eq!(formatted, "!!map &anchor { a: 1 }");
    Ok(())
}
#[test]
fn test_props_in_map_yml_bracket_spacingfalse_format_1_16191fe8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!map &anchor {a: 1}")?;
    assert_eq!(formatted, "a: !!map &anchor {a: 1}");
    Ok(())
}
#[test]
fn test_props_in_map_yml_tab_width_4_format_1_16191fe8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!map &anchor {a: 1}")?;
    assert_eq!(formatted, "a: !!map &anchor { a: 1 }");
    Ok(())
}
#[test]
fn test_props_in_map_yml_format_1_16191fe8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("a: !!map &anchor {a: 1}")?;
    assert_eq!(formatted, "a: !!map &anchor { a: 1 }");
    Ok(())
}
#[test]
fn test_short_key_yml_bracket_spacingfalse_format_1_dc9bb665() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{? 1,? 2,? 3}")?;
    assert_eq!(formatted, "{1, 2, 3}");
    Ok(())
}
#[test]
fn test_short_key_yml_tab_width_4_format_1_dc9bb665() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{? 1,? 2,? 3}")?;
    assert_eq!(formatted, "{ 1, 2, 3 }");
    Ok(())
}
#[test]
fn test_short_key_yml_format_1_dc9bb665() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{? 1,? 2,? 3}")?;
    assert_eq!(formatted, "{ 1, 2, 3 }");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_bracket_spacingfalse_format_1_ef558fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: a,2: b,3: c}")?;
    assert_eq!(formatted, "{1: a, 2: b, 3: c}");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_tab_width_4_format_1_ef558fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: a,2: b,3: c}")?;
    assert_eq!(formatted, "{ 1: a, 2: b, 3: c }");
    Ok(())
}
#[test]
fn test_short_key_short_value_yml_format_1_ef558fcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: a,2: b,3: c}")?;
    assert_eq!(formatted, "{ 1: a, 2: b, 3: c }");
    Ok(())
}
#[test]
fn test_short_plain_yml_bracket_spacingfalse_format_1_2ffb5454() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1,2,3}")?;
    assert_eq!(formatted, "{1, 2, 3}");
    Ok(())
}
#[test]
fn test_short_plain_yml_tab_width_4_format_1_2ffb5454() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1,2,3}")?;
    assert_eq!(formatted, "{ 1, 2, 3 }");
    Ok(())
}
#[test]
fn test_short_plain_yml_format_1_2ffb5454() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1,2,3}")?;
    assert_eq!(formatted, "{ 1, 2, 3 }");
    Ok(())
}
#[test]
fn test_short_value_yml_bracket_spacingfalse_format_1_ff51024b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: 1,2: 2,3: 3}")?;
    assert_eq!(formatted, "{1: 1, 2: 2, 3: 3}");
    Ok(())
}
#[test]
fn test_short_value_yml_tab_width_4_format_1_ff51024b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: 1,2: 2,3: 3}")?;
    assert_eq!(formatted, "{ 1: 1, 2: 2, 3: 3 }");
    Ok(())
}
#[test]
fn test_short_value_yml_format_1_ff51024b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{1: 1,2: 2,3: 3}")?;
    assert_eq!(formatted, "{ 1: 1, 2: 2, 3: 3 }");
    Ok(())
}
#[test]
fn test_very_long_value_yml_bracket_spacingfalse_format_1_a24a60e5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_spacing(false)
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\nx: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\n}") ? ;
    assert_eq ! (formatted , "{\n  x: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890,\n}");
    Ok(())
}
#[test]
fn test_very_long_value_yml_tab_width_4_format_1_a24a60e5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .tab_width(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\nx: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\n}") ? ;
    assert_eq ! (formatted , "{\n    x: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890,\n}");
    Ok(())
}
#[test]
fn test_very_long_value_yml_format_1_a24a60e5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("yml")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\nx: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890\n}") ? ;
    assert_eq ! (formatted , "{\n  x: 12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890,\n}");
    Ok(())
}
