#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cjs_with_types_js_format_1_ad20d98c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type talias4 = number;\nexport interface IFoo { prop: number };\n\nmodule.exports = {}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nexport type talias4 = number;\nexport interface IFoo {\n  prop: number;\n}\n\nmodule.exports = {};");
    Ok(())
}
#[test]
fn test_importer_js_format_1_7a2065a2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nimport type {\n  inlinedType1,\n  standaloneType1,\n  talias1,\n  talias3,\n} from \"./types_only\";\n\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = 'asdf'; // Error: string ~> number\n\nvar c: standaloneType1 = 42;\nvar d: standaloneType1 = 'asdf'; // Error: string ~> number\n\nvar e: talias1 = 42;\nvar f: talias1 = 'asdf'; // Error: string ~> number\n\nvar g: talias3 = 42;\nvar h: talias3 = 'asdf'; // Error: string ~> number\n\nimport type {talias4} from \"./cjs_with_types\";\nvar i: talias4 = 42;\nvar j: talias4 = 'asdf'; // Error: string ~> number\n\nimport {IFoo, IFoo2} from \"./types_only\";\n\nvar k: IFoo = {prop: 42};\nvar l: IFoo = {prop: 'asdf'}; // Error: {prop:string} ~> {prop:number}\n\nvar m: IFoo2 = {prop: 'asdf'};\nvar n: IFoo2 = {prop: 42}; // Error: {prop:number} ~> {prop:string}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nimport type {\n  inlinedType1,\n  standaloneType1,\n  talias1,\n  talias3,\n} from \"./types_only\";\n\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = \"asdf\"; // Error: string ~> number\n\nvar c: standaloneType1 = 42;\nvar d: standaloneType1 = \"asdf\"; // Error: string ~> number\n\nvar e: talias1 = 42;\nvar f: talias1 = \"asdf\"; // Error: string ~> number\n\nvar g: talias3 = 42;\nvar h: talias3 = \"asdf\"; // Error: string ~> number\n\nimport type { talias4 } from \"./cjs_with_types\";\nvar i: talias4 = 42;\nvar j: talias4 = \"asdf\"; // Error: string ~> number\n\nimport { IFoo, IFoo2 } from \"./types_only\";\n\nvar k: IFoo = { prop: 42 };\nvar l: IFoo = { prop: \"asdf\" }; // Error: {prop:string} ~> {prop:number}\n\nvar m: IFoo2 = { prop: \"asdf\" };\nvar n: IFoo2 = { prop: 42 }; // Error: {prop:number} ~> {prop:string}");
    Ok(())
}
#[test]
fn test_types_only_js_format_1_a719f954() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type inlinedType1 = number;\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = 'asdf'; // Error: string ~> number\n\ntype standaloneType1 = number;\nexport type {standaloneType1};\n\ntype standaloneType2 = number;\nexport {standaloneType2}; // Error: Missing \\`type\\` keyword\n\nexport type {talias1, talias2 as talias3, IFoo2} from \"./types_only2\";\n\nexport interface IFoo { prop: number };") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nexport type inlinedType1 = number;\nvar a: inlinedType1 = 42;\nvar b: inlinedType1 = \"asdf\"; // Error: string ~> number\n\ntype standaloneType1 = number;\nexport type { standaloneType1 };\n\ntype standaloneType2 = number;\nexport { standaloneType2 }; // Error: Missing \\`type\\` keyword\n\nexport type { talias1, talias2 as talias3, IFoo2 } from \"./types_only2\";\n\nexport interface IFoo {\n  prop: number;\n}");
    Ok(())
}
#[test]
fn test_types_only_2_js_format_1_2b38d888() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nexport type talias1 = number;\nexport type talias2 = number;\nexport interface IFoo2 { prop: string };") ? ;
    assert_eq ! (formatted , "/* @flow */\n\nexport type talias1 = number;\nexport type talias2 = number;\nexport interface IFoo2 {\n  prop: string;\n}");
    Ok(())
}
