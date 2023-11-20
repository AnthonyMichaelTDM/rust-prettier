#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_cross_array_json_format_1_0a490f3c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(51)
        .range_start(12)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | [{\n   2 | a: [\n   3 | 1,\n>  4 |  2,  3,4,5,6,      7,\n     |  ^^^^^^^^^^^^^^^^^^^^\n>  5 | 8\n     | ^\n>  6 | ],\n     | ^\n>  7 | b: [1, 2,      3, 4],\n     | ^^^^^^^^^^^^^\n   8 | c: [1,     2]\n   9 | }\n  10 | ,{a:      2}]\n  11 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[{\n  \"a\": [1, 2, 3, 4, 5, 6, 7, 8],\n  \"b\": [1, 2, 3, 4],\n  \"c\": [1, 2]\n}\n,{a:      2}]");
}
#[test]
fn test_cross_array_2_json_format_1_37b43434() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(36)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | {\n  2 | a: [\n  3 | 1,\n> 4 |  2,  3,4,5,6,   [1, 2,      3, 4],   7,\n    |  ^^^^^^^^^^^^^^^^^^^^^^^^^\n  5 | 8\n  6 | ],\n  7 | c: [1,     2]\n  8 | }\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\na: [1, 2, 3, 4, 5, 6, [1, 2, 3, 4], 7, 8],\nc: [1,     2]\n}"
    );
}
#[test]
fn test_cross_object_json_format_1_0aa5b467() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(31)
        .range_start(12)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | [{a:\n> 2 | { \"b\": 2, \"c\": 3 },\n    |        ^^^^^^^^^^^^\n> 3 | b: {d:4},\n    | ^^^^^^\n  4 | c: {d:     6}\n  5 | },\n  6 | {a:           1}]\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[{ \"a\": { \"b\": 2, \"c\": 3 }, \"b\": { \"d\": 4 }, \"c\": { \"d\": 6 } },\n{a:           1}]");
}
#[test]
fn test_cross_object_2_json_format_1_7e8b7579() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(28)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | {a:\n> 2 | { \"b\": 2, \"c\": 3,\n    |        ^^^^^^^^^^\n> 3 | d: {d:4}, },\n    | ^^^^^^\n  4 | c: {d:     6}\n  5 | }\n  6 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{a:\n{ \"b\": 2, \"c\": 3, \"d\": { \"d\": 4 } },\nc: {d:     6}\n}"
    );
}
#[test]
fn test_identifier_json_format_1_35484790() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(4)
        .range_start(1)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | {\"a\":1, \"b\":2}\n    |  ^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\"a\":1, \"b\":2}");
}
#[test]
fn test_identifier_2_json_format_1_9265b21b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(3)
        .range_start(2)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | Infinity\n    |   ^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "Infinit");
}
#[test]
fn test_inside_array_json_format_1_475cda71() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(30)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | {\n  2 | a: [\n  3 | 1,\n> 4 |  2,  3,4,5,6,      7,\n    |  ^^^^^^^^^^^^^^^^^^^\n  5 | 8\n  6 | ],\n  7 | b: [1, 2,      3, 4]\n  8 | }\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{\na: [1, 2, 3, 4, 5, 6, 7, 8],\nb: [1, 2,      3, 4]\n}"
    );
}
#[test]
fn test_inside_object_json_format_1_4a72980b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(20)
        .range_start(11)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | {a:\n> 2 | { \"b\": 2, \"c\": 3 },\n    |        ^^^^^^^^^\n  3 | b: {d:4}\n  4 | }\n  5 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{a:\n{ \"b\": 2, \"c\": 3 },\nb: {d:4}\n}");
}
#[test]
fn test_issue_4009_json_format_1_21f1fa1c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(19)
        .range_start(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | {\n    |  ^\n> 2 |     \"foo\": \"bar\"\n    | ^^^^^^^^^^^^^^^^\n> 3 | }\n    | ^\n  4 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\n  \"foo\": \"bar\"\n}");
}
#[test]
fn test_issue_7116_json_format_1_47c7a53f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(7)
        .range_start(4)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | {\"b\": 2}\n    |     ^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ \"b\": 2 }");
}
#[test]
fn test_issue_2297_json_format_1_ea66acb5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(6)
        .range_start(5)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | {\"a\":1, \"b\":2}\n    |      ^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{\"a\":1, \"b\":2}");
}
#[test]
fn test_number_json_format_1_7beded6d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(16)
        .range_start(7)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("> 1 | { \"b\": 2, \"c\": 3 }\n    |        ^^^^^^^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{ \"b\": 2, \"c\": 3 }");
}
#[test]
fn test_template_literal_json_format_1_b960597d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(7)
        .range_start(6)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | [\n> 2 | {a:\\`a\\`,n:\n    |     ^\n  3 | ''},\n  4 | {a:\\`a\\`,n:\n  5 | ''}\n  6 | ]\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[\n{a:\\`a\\`,n:\n''},\n{a:\\`a\\`,n:\n''}\n]");
}
#[test]
fn test_template_literal_2_json_format_1_0cdedaf7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(7)
        .range_start(4)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | [\n> 2 | {a:\\`a\\`,n:\n    |   ^^^\n  3 | ''},\n  4 | {a:\\`a\\`,n:\n  5 | ''}\n  6 | ]\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[\n{ \"a\": \\`a\\`, \"n\": \"\" },\n{a:\\`a\\`,n:\n''}\n]"
    );
}
#[test]
fn test_unary_expression_json_format_1_0d2d11b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(15)
        .range_start(9)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | {a:\n   2 | [\n   3 | 1,\n>  4 | -2,\n     | ^^^\n>  5 | -3\n     | ^^\n   6 |\n   7 | ],\n   8 | b: [1,    2]\n   9 | }\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "{a:\n[1, -2, -3],\nb: [1,    2]\n}");
}
#[test]
fn test_unary_expression_2_json_format_1_c0211c49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .range_end(9)
        .range_start(2)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  1 | -\n> 2 | 2.00000\n    | ^^^^^^^\n  3 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "-\n2.0");
}
