#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_type_argument_src_ts_format_1_f326af0d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("src.ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("Observable.empty<never>();")?;
    assert_eq!(formatted, "Observable.empty<never>();");
    Ok(())
}
