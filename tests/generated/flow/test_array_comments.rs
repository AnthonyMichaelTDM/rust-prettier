#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_a8764dc1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export type FileMetaData = [\n  /* id */ string,\n  /* mtime */ number,\n  /* visited */ 0|1,\n  /* dependencies */ Array<string>,\n];\n\nexport type ModuleMetaData = [Path, /* type */ number];") ? ;
    assert_eq ! (formatted , "export type FileMetaData = [\n  /* id */ string,\n  /* mtime */ number,\n  /* visited */ 0 | 1,\n  /* dependencies */ Array<string>,\n];\n\nexport type ModuleMetaData = [Path, /* type */ number];");
    Ok(())
}
