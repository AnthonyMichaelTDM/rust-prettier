#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_as_as_js_format_1_da07b00e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nimport { foo as as } from \"foo\";\nimport { as as as } from \"foo\";\nimport { as as foo } from \"foo\";") ? ;
    assert_eq ! (formatted , "// @flow\nimport { foo as as } from \"foo\";\nimport { as as as } from \"foo\";\nimport { as as foo } from \"foo\";");
    Ok(())
}
#[test]
fn test_type_import_as_as_js_format_1_e7c4881a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import {type foo as as} from \"foo\";")?;
    assert_eq!(formatted, "import { type foo as as } from \"foo\";");
    Ok(())
}
