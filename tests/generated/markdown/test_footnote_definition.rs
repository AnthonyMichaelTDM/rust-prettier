#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_long_md_prose_wrapalways_format_1_55d42c0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]: this is a long long long long long long long long long long long long long paragraph.\n          this is a long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "[^hello]:\n    this is a long long long long long long long long long long long long long\n    paragraph.\n\n[^world]:\n    this is a long long long long long long long long long long long long long\n    paragraph. this is a long long long long long long long long long long long\n    long long paragraph.");
    Ok(())
}
#[test]
fn test_long_md_prose_wrapnever_format_1_55d42c0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]: this is a long long long long long long long long long long long long long paragraph.\n          this is a long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]: this is a long long long long long long long long long long long long long paragraph. this is a long long long long long long long long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_long_md_prose_wrappreserve_format_1_55d42c0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]: this is a long long long long long long long long long long long long long paragraph.\n          this is a long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]:\n    this is a long long long long long long long long long long long long long paragraph.\n    this is a long long long long long long long long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_long_md_tab_width_3_format_1_55d42c0f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .tab_width(3)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]: this is a long long long long long long long long long long long long long paragraph.\n          this is a long long long long long long long long long long long long long paragraph.") ? ;
    assert_eq ! (formatted , "[^hello]: this is a long long long long long long long long long long long long long paragraph.\n[^world]:\n    this is a long long long long long long long long long long long long long paragraph.\n    this is a long long long long long long long long long long long long long paragraph.");
    Ok(())
}
#[test]
fn test_multiline_md_prose_wrapalways_format_1_3c905cdb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^fn1]:\n\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`") ? ;
    assert_eq ! (formatted , "[^fn1]:\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]:\n    Here is a footnote which includes code. Here is a footnote which includes\n    code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`");
    Ok(())
}
#[test]
fn test_multiline_md_prose_wrapnever_format_1_3c905cdb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^fn1]:\n\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`") ? ;
    assert_eq ! (formatted , "[^fn1]:\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]:\n    Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`");
    Ok(())
}
#[test]
fn test_multiline_md_prose_wrappreserve_format_1_3c905cdb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^fn1]:\n\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`") ? ;
    assert_eq ! (formatted , "[^fn1]:\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]:\n    Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`");
    Ok(())
}
#[test]
fn test_multiline_md_tab_width_3_format_1_3c905cdb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .tab_width(3)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^fn1]:\n\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`") ? ;
    assert_eq ! (formatted , "[^fn1]:\n    > \\`\\`\\`rs\n    > fn main() {\n    >     println!(\"this is some Rust!\");\n    > }\n    > \\`\\`\\`\n\n[^fn2]: Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`\n\n[^fn2]:\n    Here is a footnote which includes code. Here is a footnote which includes code. Here is a footnote which includes code.\n\n    \\`\\`\\`rs\n    fn main() {\n        println!(\"this is some Rust!\");\n    }\n    \\`\\`\\`");
    Ok(())
}
#[test]
fn test_sibling_md_prose_wrapalways_format_1_d251e981() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: a\n[^a]: a") ? ;
    assert_eq ! (formatted , "[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]: a\n[^a]: a");
    Ok(())
}
#[test]
fn test_sibling_md_prose_wrapnever_format_1_d251e981() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: a\n[^a]: a") ? ;
    assert_eq ! (formatted , "[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]: a\n[^a]: a");
    Ok(())
}
#[test]
fn test_sibling_md_prose_wrappreserve_format_1_d251e981() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: a\n[^a]: a") ? ;
    assert_eq ! (formatted , "[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]: a\n[^a]: a");
    Ok(())
}
#[test]
fn test_sibling_md_tab_width_3_format_1_d251e981() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .tab_width(3)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: > 123\\\\\n      > 456\n[^a]: a\n[^a]: a\n[^a]: a") ? ;
    assert_eq ! (formatted , "[^a]: a\n[^a]: a\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: > 123\n[^a]: a\n[^a]: a\n[^a]: a\n\n---\n\n[^a]: a\n[^a]: a\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]:\n    > 123\\\\\n    > 456\n\n[^a]: a\n[^a]: a\n[^a]: a");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapalways_format_1_99e518c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("always")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[^hello]: world")?;
    assert_eq!(formatted, "[^hello]: world");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrapnever_format_1_99e518c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("never")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[^hello]: world")?;
    assert_eq!(formatted, "[^hello]: world");
    Ok(())
}
#[test]
fn test_simple_md_prose_wrappreserve_format_1_99e518c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .prose_wrap("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[^hello]: world")?;
    assert_eq!(formatted, "[^hello]: world");
    Ok(())
}
#[test]
fn test_simple_md_tab_width_3_format_1_99e518c6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("md")
        .print_width(80)
        .tab_width(3)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[^hello]: world")?;
    assert_eq!(formatted, "[^hello]: world");
    Ok(())
}
