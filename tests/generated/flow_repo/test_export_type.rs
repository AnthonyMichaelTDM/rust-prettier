#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cjs_with_types_js_format_1_ad20d98c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type talias4 = number;\nexport interface IFoo { prop: number };\n\nmodule.exports = {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nexport type talias4 = number;\nexport interface IFoo {\n  prop: number;\n}\n\nmodule.exports = {};");
}
#[test]
fn test_importer_js_format_1_7a2065a2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nimport type {\n  inlinedType1,\n  standaloneType1,\n  talias1,\n  talias3,\n} from \"./types_only\";\n\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = 'asdf'; // Error: string ~> number\n\nvar c: standaloneType1 = 42;\nvar d: standaloneType1 = 'asdf'; // Error: string ~> number\n\nvar e: talias1 = 42;\nvar f: talias1 = 'asdf'; // Error: string ~> number\n\nvar g: talias3 = 42;\nvar h: talias3 = 'asdf'; // Error: string ~> number\n\nimport type {talias4} from \"./cjs_with_types\";\nvar i: talias4 = 42;\nvar j: talias4 = 'asdf'; // Error: string ~> number\n\nimport {IFoo, IFoo2} from \"./types_only\";\n\nvar k: IFoo = {prop: 42};\nvar l: IFoo = {prop: 'asdf'}; // Error: {prop:string} ~> {prop:number}\n\nvar m: IFoo2 = {prop: 'asdf'};\nvar n: IFoo2 = {prop: 42}; // Error: {prop:number} ~> {prop:string}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nimport type {\n  inlinedType1,\n  standaloneType1,\n  talias1,\n  talias3,\n} from \"./types_only\";\n\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = \"asdf\"; // Error: string ~> number\n\nvar c: standaloneType1 = 42;\nvar d: standaloneType1 = \"asdf\"; // Error: string ~> number\n\nvar e: talias1 = 42;\nvar f: talias1 = \"asdf\"; // Error: string ~> number\n\nvar g: talias3 = 42;\nvar h: talias3 = \"asdf\"; // Error: string ~> number\n\nimport type { talias4 } from \"./cjs_with_types\";\nvar i: talias4 = 42;\nvar j: talias4 = \"asdf\"; // Error: string ~> number\n\nimport { IFoo, IFoo2 } from \"./types_only\";\n\nvar k: IFoo = { prop: 42 };\nvar l: IFoo = { prop: \"asdf\" }; // Error: {prop:string} ~> {prop:number}\n\nvar m: IFoo2 = { prop: \"asdf\" };\nvar n: IFoo2 = { prop: 42 }; // Error: {prop:number} ~> {prop:string}");
}
#[test]
fn test_types_only_js_format_1_a719f954() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type inlinedType1 = number;\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = 'asdf'; // Error: string ~> number\n\ntype standaloneType1 = number;\nexport type {standaloneType1};\n\ntype standaloneType2 = number;\nexport {standaloneType2}; // Error: Missing \\`type\\` keyword\n\nexport type {talias1, talias2 as talias3, IFoo2} from \"./types_only2\";\n\nexport interface IFoo { prop: number };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nexport type inlinedType1 = number;\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = \"asdf\"; // Error: string ~> number\n\ntype standaloneType1 = number;\nexport type { standaloneType1 };\n\ntype standaloneType2 = number;\nexport { standaloneType2 }; // Error: Missing \\`type\\` keyword\n\nexport type { talias1, talias2 as talias3, IFoo2 } from \"./types_only2\";\n\nexport interface IFoo {\n  prop: number;\n}");
}
#[test]
fn test_types_only_2_js_format_1_2b38d888() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type talias1 = number;\nexport type talias2 = number;\nexport interface IFoo2 { prop: string };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nexport type talias1 = number;\nexport type talias2 = number;\nexport interface IFoo2 {\n  prop: string;\n}");
}
