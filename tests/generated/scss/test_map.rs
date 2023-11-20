#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_key_values_scss_trailing_commaes_5_format_1_6a7c6abf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  ('key': 'value',),\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  ('list'),\n  ('list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list'):\n  ('list'),\n  ('list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list'):\n  ('key': 'value',),\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  1,\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n  ):\n    (\n      \"key\": \"value\",\n    ),\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n  ):\n    (\n      \"list\",\n    ),\n  (\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n  ):\n    (\n      \"list\",\n    ),\n  (\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n  ):\n    (\n      \"key\": \"value\",\n    ),\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n  ):\n    1,\n);");
}
#[test]
fn test_key_values_scss_trailing_commanone_format_1_6a7c6abf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  ('key': 'value',),\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  ('list'),\n  ('list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list'):\n  ('list'),\n  ('list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list', 'list'):\n  ('key': 'value',),\n  ('key': 'value', 'key': 'value','key': 'value','key': 'value','key': 'value',):\n  1,\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\"\n  ):\n    (\n      \"key\": \"value\"\n    ),\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\"\n  ):\n    (\n      \"list\"\n    ),\n  (\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\"\n  ):\n    (\n      \"list\"\n    ),\n  (\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\",\n    \"list\"\n  ):\n    (\n      \"key\": \"value\"\n    ),\n  (\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\",\n    \"key\": \"value\"\n  ):\n    1\n);");
}
#[test]
fn test_keys_scss_trailing_commaes_5_format_1_566e5ada() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n  'string': 'hello world',\n  ('list'): 'hello world',\n  ('key': 'value'): 'hello world',\n  ('list', 'long long long long long long long long long long long long long list'): 'hello world',\n  ('key': 'value','long long long long long long long long long long long long long map': 'value',): 'hello world',\n);\n\n// #10000\n$map: (\n\t('my list'): 'hello world',\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  \"string\": \"hello world\",\n  (\"list\"): \"hello world\",\n  (\"key\": \"value\"): \"hello world\",\n  (\n    \"list\",\n    \"long long long long long long long long long long long long long list\",\n  ):\n    \"hello world\",\n  (\n    \"key\": \"value\",\n    \"long long long long long long long long long long long long long map\":\n      \"value\",\n  ):\n    \"hello world\",\n);\n\n// #10000\n$map: (\n  (\"my list\"): \"hello world\",\n);");
}
#[test]
fn test_keys_scss_trailing_commanone_format_1_566e5ada() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["scss"])
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("$map: (\n  'string': 'hello world',\n  ('list'): 'hello world',\n  ('key': 'value'): 'hello world',\n  ('list', 'long long long long long long long long long long long long long list'): 'hello world',\n  ('key': 'value','long long long long long long long long long long long long long map': 'value',): 'hello world',\n);\n\n// #10000\n$map: (\n\t('my list'): 'hello world',\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "$map: (\n  \"string\": \"hello world\",\n  (\"list\"): \"hello world\",\n  (\"key\": \"value\"): \"hello world\",\n  (\n    \"list\",\n    \"long long long long long long long long long long long long long list\"\n  ):\n    \"hello world\",\n  (\n    \"key\": \"value\",\n    \"long long long long long long long long long long long long long map\":\n      \"value\"\n  ):\n    \"hello world\"\n);\n\n// #10000\n$map: (\n  (\"my list\"): \"hello world\"\n);");
}
