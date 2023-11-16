#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_document_yml_format_1_0b375f97() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("# prettier-ignore\n---\naaaaa:\n           bbbbb\n...\naaaaa:\n           bbbbb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "# prettier-ignore\n---\naaaaa:\n           bbbbb\n---\naaaaa: bbbbb"
    );
}
#[test]
fn test_leading_comment_yml_format_1_4fc57b84() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("abc:     123\n# prettier-ignore\ndef:     456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "abc: 123\n# prettier-ignore\ndef:     456");
}
#[test]
fn test_middle_comment_yml_format_1_22103cb5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("abc:  !!str #   hello\n   123\n# prettier-ignore\ndef:  !!str #   hello\n   456");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "abc: !!str #   hello\n  123\n# prettier-ignore\ndef:  !!str #   hello\n   456"
    );
}
#[test]
fn test_trailing_comma_yml_format_1_c3f777fc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("[\n  # prettier-ignore\n             {  sss  },       # 123\n    {  qqqq },\n  \n  # prettier-ignore\n          [ccc    ]       # 777\n  , [ddd    ]\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  # prettier-ignore\n  {  sss  }, # 123\n  { qqqq },\n\n  # prettier-ignore\n  [ccc    ], # 777\n  [ddd],\n]");
}
#[test]
fn test_trailing_comment_yml_format_1_931584ad() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("abc:     123 # hello\n# prettier-ignore\ndef:     456 # hello");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "abc: 123 # hello\n# prettier-ignore\ndef:     456 # hello"
    );
}
