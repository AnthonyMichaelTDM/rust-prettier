#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_empty_props_less_format_1_89773cf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        ":root {\n  --empty:;\n     --one-space: ;\n  --two-space:  ;\n--many-space:       ;\n}",
    )?;
    assert_eq!(
        formatted,
        ":root {\n  --empty:;\n  --one-space: ;\n  --two-space:  ;\n  --many-space:       ;\n}"
    );
    Ok(())
}
#[test]
fn test_test_less_format_1_3a47d3ca() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("less")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/*\nThis test is copied from \\`postcss@8\\` release note\n\nhttps://github.com/postcss/postcss/releases/tag/8.0.0\n*/\n\n:root {\n  --empty: ;\n  --JSON: [1, \"2\", {\"three\": {\"a\":1}}, [4]];\n  --javascript: function(rule) { console.log(rule) };\n}\n\n@supports (--element(\".minwidth\", { \"minWidth\": 300 })) {\n  [--self] {\n    background: greenyellow;\n  }\n}") ? ;
    assert_eq ! (formatted , "/*\nThis test is copied from \\`postcss@8\\` release note\n\nhttps://github.com/postcss/postcss/releases/tag/8.0.0\n*/\n\n:root {\n  --empty: ;\n  --JSON: [1, \"2\", {\"three\": {\"a\": 1}}, [4]];\n  --javascript: function(rule) {console.log(rule)};\n}\n\n@supports (--element(\".minwidth\", {\"minWidth\": 300})) {\n  [--self] {\n    background: greenyellow;\n  }\n}");
    Ok(())
}
