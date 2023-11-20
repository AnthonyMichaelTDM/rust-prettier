#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_trailing_comma_js_trailing_commaall_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
#[test]
fn test_trailing_comma_js_trailing_commaes_5_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
#[test]
fn test_trailing_comma_js_trailing_commanone_format_1_e4e44206() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("none")
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string,\n    }, // <== this one\n  ];\n  shop: string;\n  distance: number;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export interface ShopQueryResult {\n  chic: boolean;\n  location: number[];\n  menus: Menu[];\n  openingDays: number[];\n  closingDays: [\n    {\n      from: string,\n      to: string\n    } // <== this one\n  ];\n  shop: string;\n  distance: number;\n}");
}
