#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_atx_md_prose_wrapalways_format_1_f06fd005() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# h1\n\n## h2")?;
    assert_eq!(formatted, "# h1\n\n## h2");
    Ok(())
}
#[test]
fn test_hash_sign_md_prose_wrapalways_format_1_02e3f28e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("# Programming languages\n\n## C#\n\n## JavaScript")?;
    assert_eq!(
        formatted,
        "# Programming languages\n\n## C#\n\n## JavaScript"
    );
    Ok(())
}
#[test]
fn test_long_heading_md_prose_wrapalways_format_1_50bae9cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("# this is a long long long long long long long long long long long long long long heading.") ? ;
    assert_eq ! (formatted , "# this is a long long long long long long long long long long long long long long heading.");
    Ok(())
}
#[test]
fn test_mixed_md_prose_wrapalways_format_1_f2478882() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("## LaraCart - Laravel Shopping Cart Package (<a href=\"http://laracart.lukepolo.com/\">http://laracart.lukepolo.com</a>)") ? ;
    assert_eq ! (formatted , "## LaraCart - Laravel Shopping Cart Package (<a href=\"http://laracart.lukepolo.com/\">http://laracart.lukepolo.com</a>)");
    Ok(())
}
#[test]
fn test_setext_md_prose_wrapalways_format_1_3ef5e9cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("h1\n===\n\nh2\n---")?;
    assert_eq!(formatted, "# h1\n\n## h2");
    Ok(())
}
#[test]
fn test_unbreakable_md_prose_wrapalways_format_1_ee2c86a6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("### `reporters` [array<moduleName | [moduleName, options]>]")?;
    assert_eq!(
        formatted,
        "### `reporters` [array<moduleName | [moduleName, options]>]"
    );
    Ok(())
}
