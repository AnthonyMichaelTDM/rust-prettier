#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_destructuring_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_js_format_1_7b706c9f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const [a, b] = #[1, 2];\nassert(a === 1);\nassert(b === 2);\n\nconst [a, ...rest] = #[1, 2, 3];\nassert(a === 1);\nassert(Array.isArray(rest));\nassert(rest[0] === 2);\nassert(rest[1] === 3);") ? ;
    assert_eq ! (formatted , "const [a, b] = #[1, 2];\nassert(a === 1);\nassert(b === 2);\n\nconst [a, ...rest] = #[1, 2, 3];\nassert(a === 1);\nassert(Array.isArray(rest));\nassert(rest[0] === 2);\nassert(rest[1] === 3);");
    Ok(())
}
#[test]
fn test_invalid_tuple_holes_js_babel_estree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_invalid_tuple_holes_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_invalid_tuple_holes_js_babel_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_invalid_tuple_holes_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_invalid_tuple_holes_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_syntax_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_syntax_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_syntax_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_syntax_js_format_1_8a4d2bdf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#[]\n#[1, 2]\n#[1, 2, #{ a: 3 }]")?;
    assert_eq!(formatted, "#[];\n#[1, 2];\n#[1, 2, #{ a: 3 }];");
    Ok(())
}
#[test]
fn test_tuple_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_js_format_1_97a05993() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const tuple1 = #[1, 2, 3];\n\nassert(tuple1[0] === 1);\n\nconst tuple2 = tuple1.with(0, 2);\nassert(tuple1 !== tuple2);\nassert(tuple2 === #[2, 2, 3]);\n\nconst tuple3 = #[1, ...tuple2];\nassert(tuple3 === #[1, 2, 2, 3]);\n\nconst tuple4 = tuple3.pushed(4);\nassert(tuple4 === #[1, 2, 2, 3, 4]);\n\nassert(tuple4.first() === 1);\nconst tuple5 = tuple4.popped();\nassert(tuple5 === #[2, 2, 3, 4]);") ? ;
    assert_eq ! (formatted , "const tuple1 = #[1, 2, 3];\n\nassert(tuple1[0] === 1);\n\nconst tuple2 = tuple1.with(0, 2);\nassert(tuple1 !== tuple2);\nassert(tuple2 === #[2, 2, 3]);\n\nconst tuple3 = #[1, ...tuple2];\nassert(tuple3 === #[1, 2, 2, 3]);\n\nconst tuple4 = tuple3.pushed(4);\nassert(tuple4 === #[1, 2, 2, 3, 4]);\n\nassert(tuple4.first() === 1);\nconst tuple5 = tuple4.popped();\nassert(tuple5 === #[2, 2, 3, 4]);");
    Ok(())
}
#[test]
fn test_tuple_trailing_comma_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_trailing_comma_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_trailing_comma_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_trailing_comma_js_format_1_f1d11605() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("#[1,]")?;
    assert_eq!(formatted, "#[1];");
    Ok(())
}
