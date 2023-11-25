#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_1_ts_format_1_20030d1f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("f<<<T>(x)")?;
    assert_eq!(formatted, "f << (<T>x);");
    Ok(())
}
#[test]
fn test_2_ts_format_1_13c41675() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("f<<T>(v: T) => void>();")?;
    assert_eq!(formatted, "f<<T>(v: T) => void>();");
    Ok(())
}
#[test]
fn test_3_ts_babel_ts_format_1_0a7afc32() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(class extends f<<T>(v: T) => void> {});")?;
    assert_eq!(formatted, "(class extends f<<T>(v: T) => void> {});");
    Ok(())
}
#[test]
fn test_3_ts_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_4_ts_format_1_f946803f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("(@f<<T>(v: T) => void>() class {});")?;
    assert_eq!(formatted, "(\n  @f<<T>(v: T) => void>()\n  class {}\n);");
    Ok(())
}
#[test]
fn test_5_tsx_babel_ts_format_1_c913ce2f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<Component<<T>(v: T) => void> />")?;
    assert_eq!(formatted, "<Component<<T>(v: T) => void> />;");
    Ok(())
}
#[test]
fn test_5_tsx_typescript_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_6_ts_format_1_b61083f9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("new f<<T>(v: T) => void>();")?;
    assert_eq!(formatted, "new f<<T>(v: T) => void>();");
    Ok(())
}
