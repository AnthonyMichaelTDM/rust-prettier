#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_block_comment_json_trailing_commaall_format_1_5836cb39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ /*comment*/ \"K\": \"V\" }");
}
#[test]
fn test_block_comment_json_trailing_commaall_format_2_5836cb39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ /*comment*/ K: \"V\" }");
}
#[test]
fn test_block_comment_json_trailing_commaes_5_format_1_5836cb39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ /*comment*/ \"K\": \"V\" }");
}
#[test]
fn test_block_comment_json_trailing_commaes_5_format_2_5836cb39() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{/*comment*/\"K\":\"V\"}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ /*comment*/ K: \"V\" }");
}
#[test]
fn test_bottom_block_comment_json_trailing_commaall_format_1_4a397bb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 /* block-comment */");
}
#[test]
fn test_bottom_block_comment_json_trailing_commaall_format_2_4a397bb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 /* block-comment */");
}
#[test]
fn test_bottom_block_comment_json_trailing_commaes_5_format_1_4a397bb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 /* block-comment */");
}
#[test]
fn test_bottom_block_comment_json_trailing_commaes_5_format_2_4a397bb4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 /* block-comment */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 /* block-comment */");
}
#[test]
fn test_bottom_line_comment_json_trailing_commaall_format_1_d129fbd0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 // line-comment");
}
#[test]
fn test_bottom_line_comment_json_trailing_commaall_format_2_d129fbd0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 // line-comment");
}
#[test]
fn test_bottom_line_comment_json_trailing_commaes_5_format_1_d129fbd0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 // line-comment");
}
#[test]
fn test_bottom_line_comment_json_trailing_commaes_5_format_2_d129fbd0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("1 // line-comment");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "1 // line-comment");
}
#[test]
fn test_line_comment_json_trailing_commaall_format_1_f0c11fcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  //comment\n  \"K\": \"V\"\n}");
}
#[test]
fn test_line_comment_json_trailing_commaall_format_2_f0c11fcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  //comment\n  K: \"V\",\n}");
}
#[test]
fn test_line_comment_json_trailing_commaes_5_format_1_f0c11fcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  //comment\n  \"K\": \"V\"\n}");
}
#[test]
fn test_line_comment_json_trailing_commaes_5_format_2_f0c11fcf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("{\n  //comment\n  \"K\":\"V\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  //comment\n  K: \"V\",\n}");
}
#[test]
fn test_top_block_comment_json_trailing_commaall_format_1_60a0a710() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* comment */ {\n  \"foo\": \"bar\"\n}");
}
#[test]
fn test_top_block_comment_json_trailing_commaall_format_2_60a0a710() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* comment */ {\n  foo: \"bar\",\n}");
}
#[test]
fn test_top_block_comment_json_trailing_commaes_5_format_1_60a0a710() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* comment */ {\n  \"foo\": \"bar\"\n}");
}
#[test]
fn test_top_block_comment_json_trailing_commaes_5_format_2_60a0a710() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* comment */{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* comment */ {\n  foo: \"bar\",\n}");
}
#[test]
fn test_top_line_comment_json_trailing_commaall_format_1_1ce16c8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}"
    );
}
#[test]
fn test_top_line_comment_json_trailing_commaall_format_2_1ce16c8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  foo: \"bar\",\n}"
    );
}
#[test]
fn test_top_line_comment_json_trailing_commaes_5_format_1_1ce16c8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}"
    );
}
#[test]
fn test_top_line_comment_json_trailing_commaes_5_format_2_1ce16c8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// comment 1\n// comment 2\n{\n  \"foo\": \"bar\"\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment 1\n// comment 2\n{\n  foo: \"bar\",\n}"
    );
}
