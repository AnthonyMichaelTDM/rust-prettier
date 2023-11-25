#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_shebang_js_format_1_f0d14fbc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node\nfunction a() {}")?;
    assert_eq!(formatted, "#!/usr/bin/env node\nfunction a() {}");
    Ok(())
}
#[test]
fn test_shebang_newline_js_format_1_7bc29edc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node\n\nfunction a() {}")?;
    assert_eq!(formatted, "#!/usr/bin/env node\n\nfunction a() {}");
    Ok(())
}
#[test]
fn test_snippet_empty_file_with_shebang_format_1_38ea2608() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env nod")?;
    assert_eq!(formatted, "#!/usr/bin/env node");
    Ok(())
}
#[test]
fn test_snippet_empty_file_with_shebang_format_2_3e2dca7d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#!/usr/bin/env node")?;
    assert_eq!(formatted, "#!/usr/bin/env node");
    Ok(())
}
