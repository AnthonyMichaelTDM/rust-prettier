#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_trailing_comma_js_trailing_commaall_format_1_e4e44206() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ? ;
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
    Ok(())
}
#[test]
fn test_trailing_comma_js_trailing_commaes_5_format_1_e4e44206() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ? ;
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
    Ok(())
}
#[test]
fn test_trailing_comma_js_trailing_commanone_format_1_e4e44206() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ? ;
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string\n    } // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
    Ok(())
}
