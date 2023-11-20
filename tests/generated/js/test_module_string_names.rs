#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_module_string_names_export_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_module_string_names_export_js_format_1_f78283ae() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export { smile as \"smile1\" } from \"./emojis.js\";\nexport { \"smile\" as smile2 } from \"./emojis.js\";\nexport { \"smile\" as \"smile3\" } from \"./emojis.js\";\nexport { foo1, bar as \"foo2\" } from \"./emojis.js\";\nexport { \"學而時習之，不亦說乎？\", \"吾道一以貫之。\" as \"忠恕。\" } from \"Confucius\";\nexport { \"smile4\" } from \"./emojis.js\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export { smile as \"smile1\" } from \"./emojis.js\";\nexport { \"smile\" as smile2 } from \"./emojis.js\";\nexport { \"smile\" as \"smile3\" } from \"./emojis.js\";\nexport { foo1, bar as \"foo2\" } from \"./emojis.js\";\nexport {\n  \"學而時習之，不亦說乎？\",\n  \"吾道一以貫之。\" as \"忠恕。\",\n} from \"Confucius\";\nexport { \"smile4\" } from \"./emojis.js\";");
}
#[test]
fn test_module_string_names_import_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_module_string_names_import_js_format_1_22c3226c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { \"default\" as quotation1 } from \"Confucius\";\nimport { \"foo\" as bar, \"default\" as qux } from \"module-a\";\nimport { \"學而時習之，不亦說乎？\" as quotation2 } from \"Confucius\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { \"default\" as quotation1 } from \"Confucius\";\nimport { \"foo\" as bar, \"default\" as qux } from \"module-a\";\nimport { \"學而時習之，不亦說乎？\" as quotation2 } from \"Confucius\";");
}
