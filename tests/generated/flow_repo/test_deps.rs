#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_js_format_1_2aeac91a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./C');")?;
    assert_eq!(formatted, "require(\"./C\");");
    Ok(())
}
#[test]
fn test_b_js_format_1_2aeac91a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./C');")?;
    assert_eq!(formatted, "require(\"./C\");");
    Ok(())
}
#[test]
fn test_c_js_format_1_4fd47685() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("require('./D');\nrequire('./E');\nrequire('./F');\nrequire('./G');")?;
    assert_eq!(
        formatted,
        "require(\"./D\");\nrequire(\"./E\");\nrequire(\"./F\");\nrequire(\"./G\");"
    );
    Ok(())
}
#[test]
fn test_d_js_format_1_5db832ba() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./I');")?;
    assert_eq!(formatted, "require(\"./I\");");
    Ok(())
}
#[test]
fn test_e_js_format_1_5db832ba() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./I');")?;
    assert_eq!(formatted, "require(\"./I\");");
    Ok(())
}
#[test]
fn test_f_js_format_1_00d979fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// empty")?;
    assert_eq!(formatted, "// empty");
    Ok(())
}
#[test]
fn test_g_js_format_1_6b7709ba() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./H');")?;
    assert_eq!(formatted, "require(\"./H\");");
    Ok(())
}
#[test]
fn test_h_js_format_1_00d979fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// empty")?;
    assert_eq!(formatted, "// empty");
    Ok(())
}
#[test]
fn test_i_js_format_1_3b5f1e3e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("require('./A');")?;
    assert_eq!(formatted, "require(\"./A\");");
    Ok(())
}
