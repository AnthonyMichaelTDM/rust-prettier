#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_computed_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_computed_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_computed_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_computed_js_format_1_53207496() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const key = \"a\";\nassert(#{ [key]: 1 } === #{ a: 1 })\nassert(#{ [key.toUpperCase()]: 1 } === #{ A: 1 })\n\nassert(#{ [true]: 1 } === #{ true: 1 })\nassert(#{ [true]: 1 } === #{ [\"true\"]: 1 })\n\nassert(#{ [1 + 1]: \"two\" } === #{ 2: \"two\" })\nassert(#{ [9 + 1]: \"ten\" } === #{ [\"10\"]: \"ten\" })") ? ;
    assert_eq ! (formatted , "const key = \"a\";\nassert(#{ [key]: 1 } === #{ a: 1 });\nassert(#{ [key.toUpperCase()]: 1 } === #{ A: 1 });\n\nassert(#{ [true]: 1 } === #{ true: 1 });\nassert(#{ [true]: 1 } === #{ [\"true\"]: 1 });\n\nassert(#{ [1 + 1]: \"two\" } === #{ 2: \"two\" });\nassert(#{ [9 + 1]: \"ten\" } === #{ [\"10\"]: \"ten\" });");
    Ok(())
}
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
fn test_destructuring_js_format_1_1953ad5c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const { a, b } = #{ a: 1, b: 2 };\nassert(a === 1);\nassert(b === 2);\n\nconst { a, ...rest } = #{ a: 1, b: 2, c: 3 };\nassert(a === 1);\nassert(typeof rest === \"object\");\nassert(rest.b === 2);\nassert(rest.c === 3);") ? ;
    assert_eq ! (formatted , "const { a, b } = #{ a: 1, b: 2 };\nassert(a === 1);\nassert(b === 2);\n\nconst { a, ...rest } = #{ a: 1, b: 2, c: 3 };\nassert(a === 1);\nassert(typeof rest === \"object\");\nassert(rest.b === 2);\nassert(rest.c === 3);");
    Ok(())
}
#[test]
fn test_record_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_js_format_1_43297747() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const record1 = #{\n    a: 1,\n    b: 2,\n    c: 3,\n};\n\nconst record2 = #{...record1, b: 5};\n\nassert(record1.a === 1);\nassert(record1[\"a\"] === 1);\nassert(record1 !== record2);\nassert(record2 === #{ a: 1, c: 3, b: 5 });\nassert(record1?.a === 1);\nassert(record1?.d === undefined);\nassert(record1?.d ?? 5 === 5);\nassert(record1.d?.a === undefined);") ? ;
    assert_eq ! (formatted , "const record1 = #{\n  a: 1,\n  b: 2,\n  c: 3,\n};\n\nconst record2 = #{ ...record1, b: 5 };\n\nassert(record1.a === 1);\nassert(record1[\"a\"] === 1);\nassert(record1 !== record2);\nassert(record2 === #{ a: 1, c: 3, b: 5 });\nassert(record1?.a === 1);\nassert(record1?.d === undefined);\nassert(record1?.d ?? 5 === 5);\nassert(record1.d?.a === undefined);");
    Ok(())
}
#[test]
fn test_shorthand_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_shorthand_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_shorthand_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_shorthand_js_format_1_3d7d4af7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const url = \"https://github.com/tc39/proposal-record-tuple\";\nconst record = #{ url }\nconsole.log(record.url) // https://github.com/tc39/proposal-record-tuple") ? ;
    assert_eq ! (formatted , "const url = \"https://github.com/tc39/proposal-record-tuple\";\nconst record = #{ url };\nconsole.log(record.url); // https://github.com/tc39/proposal-record-tuple");
    Ok(())
}
#[test]
fn test_spread_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_spread_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_spread_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_spread_js_format_1_9452233f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const formData = #{ title: \"Implement all the things\" }\nconst taskNow = #{ id: 42, status: \"WIP\", ...formData }\nconst taskLater = #{ ...taskNow, status: \"DONE\" }\n\n// A reminder: The ordering of keys in record literals does not affect equality (and is not retained)\nassert(taskLater === #{ status: \"DONE\", title: formData.title, id: 42 })") ? ;
    assert_eq ! (formatted , "const formData = #{ title: \"Implement all the things\" };\nconst taskNow = #{ id: 42, status: \"WIP\", ...formData };\nconst taskLater = #{ ...taskNow, status: \"DONE\" };\n\n// A reminder: The ordering of keys in record literals does not affect equality (and is not retained)\nassert(taskLater === #{ status: \"DONE\", title: formData.title, id: 42 });");
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
fn test_syntax_js_format_1_eaab55bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("#{}\n#{ a: 1, b: 2 }\n#{ a: 1, b: #[2, 3, #{ c: 4 }] }")?;
    assert_eq!(
        formatted,
        "#{};\n#{ a: 1, b: 2 };\n#{ a: 1, b: #[2, 3, #{ c: 4 }] };"
    );
    Ok(())
}
