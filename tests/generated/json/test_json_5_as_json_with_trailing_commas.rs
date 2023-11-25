#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nested_quotes_json_quote_propspreserve_format_1_641f866b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("json")
        .print_width(80)
        .quote_props("preserve")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\"allOn\": \"Single\", \"Line\": \"example\",\n\"noSpace\":true,\n  \"quote\": {\n    'singleQuote': 'exa\"mple',\n                  \"indented\": true,\n  },\n  \"phoneNumbers\": [\n    {\"type'\": \"home\",\n      \"number\\\\\"\": \"212 555-1234\"},\n    {\"type\": \"office\",\n      \"trailing\": \"commas by accident\"},\n  ],\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"allOn\": \"Single\",\n  \"Line\": \"example\",\n  \"noSpace\": true,\n  \"quote\": {\n    \"singleQuote\": \"exa\\\\\"mple\",\n    \"indented\": true,\n  },\n  \"phoneNumbers\": [\n    { \"type'\": \"home\", \"number\\\\\"\": \"212 555-1234\" },\n    { \"type\": \"office\", \"trailing\": \"commas by accident\" },\n  ],\n}");
}
