#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_asi_ts_format_1_c59b5c02() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("let x\n!function() {};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let x;\n!function () {};");
}
#[test]
fn test_definite_ts_format_1_06c18525() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class MyComponent {\nngModel!: ng.INgModelController;\n}\n\nconst x!: string = '';\n\nvar y!: MyComponent") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class MyComponent {\n  ngModel!: ng.INgModelController;\n}\n\nconst x!: string = \"\";\n\nvar y!: MyComponent;");
}
#[test]
fn test_without_annotation_ts_format_1_2d3f9f34() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  a!\n  #b!\n  static c!\n  [d]! = 1\n  'e'!\n}\n\nlet a! = x\nconst b! = x\nvar c/* */! = x\nexport const d! = x") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  a!;\n  #b!;\n  static c!;\n  [d]! = 1;\n  \"e\"!;\n}\n\nlet a! = x;\nconst b! = x;\nvar c! /* */ = x;\nexport const d! = x;");
}
