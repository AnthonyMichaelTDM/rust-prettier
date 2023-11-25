#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dest_js_format_1_945b66f7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport {source} from \"./test\";\n\nvar a: number = source.num;\nvar b: string = source.num; // Error: num ~> string\n\nvar c: string = source.str;\nvar d: number = source.str; // Ignored error: num ~> string") ? ;
    assert_eq ! (formatted , "// @flow\n\nimport { source } from \"./test\";\n\nvar a: number = source.num;\nvar b: string = source.num; // Error: num ~> string\n\nvar c: string = source.str;\nvar d: number = source.str; // Ignored error: num ~> string");
    Ok(())
}
#[test]
fn test_source_js_format_1_c687bd04() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nexport var str = 'asdf';\nexport var num = 42;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport var str = \"asdf\";\nexport var num = 42;"
    );
    Ok(())
}
#[test]
fn test_test_js_format_1_653fb46a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nexport * as source from \"./source\";")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport * as source from \"./source\";"
    );
    Ok(())
}
