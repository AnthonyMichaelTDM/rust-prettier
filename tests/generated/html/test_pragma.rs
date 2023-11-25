#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_no_pragma_html_insert_pragmatrue_format_1_25429724() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .insert_pragma(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @not-a-pragma -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "<!-- @format -->\n\n<!-- @not-a-pragma -->\n<!doctype html>\n<html></html>"
    );
    Ok(())
}
#[test]
fn test_no_pragma_html_require_pragmatrue_format_1_25429724() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .require_pragma(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @not-a-pragma -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "\n\n<!-- @not-a-pragma -->\n<!doctype html>\n<html>\n            </html>"
    );
    Ok(())
}
#[test]
fn test_with_pragma_html_insert_pragmatrue_format_1_e4f75b98() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .insert_pragma(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @format -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "<!-- @format -->\n<!doctype html>\n<html></html>"
    );
    Ok(())
}
#[test]
fn test_with_pragma_html_require_pragmatrue_format_1_e4f75b98() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .require_pragma(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @format -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "<!-- @format -->\n<!doctype html>\n<html></html>"
    );
    Ok(())
}
#[test]
fn test_with_pragma_2_html_insert_pragmatrue_format_1_2aea0308() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .insert_pragma(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @prettier -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "<!-- @prettier -->\n<!doctype html>\n<html></html>"
    );
    Ok(())
}
#[test]
fn test_with_pragma_2_html_require_pragmatrue_format_1_2aea0308() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .require_pragma(true)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("\n\n<!-- @prettier -->\n<!doctype html>\n<html>\n            </html>")?;
    assert_eq!(
        formatted,
        "<!-- @prettier -->\n<!doctype html>\n<html></html>"
    );
    Ok(())
}
