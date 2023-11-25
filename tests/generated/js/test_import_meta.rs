#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_import_meta_js_format_1_b805bc8c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const x = import.meta;\nconst url = import.meta.url;\nimport.meta;\nimport.meta.url;\nimport.meta.couldBeMutable = true;") ? ;
    assert_eq ! (formatted , "const x = import.meta;\nconst url = import.meta.url;\nimport.meta;\nimport.meta.url;\nimport.meta.couldBeMutable = true;");
    Ok(())
}
