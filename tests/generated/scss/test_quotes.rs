#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_escape_in_string_scss_single_quotetrue_format_1_7265aa0b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$description: \"Lorem ipsum dolor sit \\\\\"amet\\\\\", consectetur adipiscing elit, \" +\n  \"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$description: 'Lorem ipsum dolor sit \"amet\", consectetur adipiscing elit, ' +\n  'sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.';");
}
#[test]
fn test_escape_in_string_scss_format_1_7265aa0b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$description: \"Lorem ipsum dolor sit \\\\\"amet\\\\\", consectetur adipiscing elit, \" +\n  \"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\";") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$description: 'Lorem ipsum dolor sit \"amet\", consectetur adipiscing elit, ' +\n  \"sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\";");
}
#[test]
fn test_forward_with_scss_single_quotetrue_format_1_77d4cc60() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@forward 'foo.scss' with ($components: red);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@forward 'foo.scss' with (\n  $components: red\n);"
    );
}
#[test]
fn test_forward_with_scss_format_1_77d4cc60() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("@forward 'foo.scss' with ($components: red);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "@forward \"foo.scss\" with (\n  $components: red\n);"
    );
}
#[test]
fn test_quotes_scss_single_quotetrue_format_1_9520646f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@use \"library\";\n\n@use \"library\" with (\n  $black: #222,\n  $border-radius: 0.1rem\n  $font-family: \"Helvetica, sans-serif\"\n);\n\n@use \"library\" as *;\n\n@use \"library\" as f;\n\n@use \"sass:map\";\n\n@forward \"library\";\n\n@forward \"library\" show border, $border-color;\n\n@forward \"library\" hide gradient;\n\n@forward \"library\" as btn-*;\n\n@forward \"library\" as btn*;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@use 'library';\n\n@use 'library' with (\n  $black: #222,\n  $border-radius: 0.1rem $font-family: 'Helvetica, sans-serif'\n);\n\n@use 'library' as *;\n\n@use 'library' as f;\n\n@use 'sass:map';\n\n@forward 'library';\n\n@forward 'library' show border, $border-color;\n\n@forward 'library' hide gradient;\n\n@forward 'library' as btn-*;\n\n@forward 'library' as btn*;");
}
#[test]
fn test_quotes_scss_format_1_9520646f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("@use \"library\";\n\n@use \"library\" with (\n  $black: #222,\n  $border-radius: 0.1rem\n  $font-family: \"Helvetica, sans-serif\"\n);\n\n@use \"library\" as *;\n\n@use \"library\" as f;\n\n@use \"sass:map\";\n\n@forward \"library\";\n\n@forward \"library\" show border, $border-color;\n\n@forward \"library\" hide gradient;\n\n@forward \"library\" as btn-*;\n\n@forward \"library\" as btn*;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@use \"library\";\n\n@use \"library\" with (\n  $black: #222,\n  $border-radius: 0.1rem $font-family: \"Helvetica, sans-serif\"\n);\n\n@use \"library\" as *;\n\n@use \"library\" as f;\n\n@use \"sass:map\";\n\n@forward \"library\";\n\n@forward \"library\" show border, $border-color;\n\n@forward \"library\" hide gradient;\n\n@forward \"library\" as btn-*;\n\n@forward \"library\" as btn*;");
}
