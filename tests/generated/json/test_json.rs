#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_json_trailing_commaall_format_1_e3104b1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("[\n  [\n1,null],\n  [1,null,],\n  [null,],\n  [0,],\n  [false,],\n  ['',]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[1, null], [1, null], [null], [0], [false], [\"\"]]"
    );
}
#[test]
fn test_array_json_trailing_commaall_format_2_e3104b1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("[\n  [\n1,null],\n  [1,null,],\n  [null,],\n  [0,],\n  [false,],\n  ['',]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[1, null], [1, null], [null], [0], [false], [\"\"]]"
    );
}
#[test]
fn test_array_json_trailing_commaes_5_format_1_e3104b1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("[\n  [\n1,null],\n  [1,null,],\n  [null,],\n  [0,],\n  [false,],\n  ['',]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[1, null], [1, null], [null], [0], [false], [\"\"]]"
    );
}
#[test]
fn test_array_json_trailing_commaes_5_format_2_e3104b1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("[\n  [\n1,null],\n  [1,null,],\n  [null,],\n  [0,],\n  [false,],\n  ['',]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[[1, null], [1, null], [null], [0], [false], [\"\"]]"
    );
}
#[test]
fn test_array_json_format_1_e3104b1d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("[\n  [\n1,null],\n  [1,null,],\n  [null,],\n  [0,],\n  [false,],\n  ['',]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  [\n    1,\n    null\n  ],\n  [\n    1,\n    null\n  ],\n  [\n    null\n  ],\n  [\n    0\n  ],\n  [\n    false\n  ],\n  [\n    \"\"\n  ]\n]");
}
#[test]
fn test_boolean_json_trailing_commaall_format_1_b9954ca3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("true");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "true");
}
#[test]
fn test_boolean_json_trailing_commaall_format_2_b9954ca3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("true");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "true");
}
#[test]
fn test_boolean_json_trailing_commaes_5_format_1_b9954ca3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .trailing_comma("es5")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("true");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "true");
}
#[test]
fn test_boolean_json_trailing_commaes_5_format_2_b9954ca3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("true");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "true");
}
#[test]
fn test_boolean_json_format_1_b9954ca3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("true");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "true");
}
#[test]
fn test_json_5_json_trailing_commaall_format_1_dfb12874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("all")
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{\n  '//': 'JSON5 allow \\`Infinity\\` and \\`NaN\\`',\n  numbers: [\n    Infinity,\n    -Infinity,\n    NaN,\n  ],\n  Infinity: NaN,\n  NaN: Infinity,\n  NaN: -Infinity,\n},\n{\n  '//': 'JSON5 numbers',\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n},\n{\n  '//': 'JSON5 strings',\nsingleQuotes: 'I can use \"double quotes\" here',\n  lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"JSON5 allow \\`Infinity\\` and \\`NaN\\`\",\n    \"numbers\": [Infinity, -Infinity, NaN],\n    \"Infinity\": NaN,\n    \"NaN\": Infinity,\n    \"NaN\": -Infinity\n  },\n  {\n    \"//\": \"JSON5 numbers\",\n    \"hexadecimal\": 0xdecaf,\n    \"leadingDecimalPoint\": 0.8675309,\n    \"andTrailing\": 8675309,\n    \"positiveSign\": +1\n  },\n  {\n    \"//\": \"JSON5 strings\",\n    \"singleQuotes\": \"I can use \\\\\"double quotes\\\\\" here\",\n    \"lineBreaks\": \"Look, Mom! \\\\\nNo \\\\\\\\n's!\"\n  }\n]");
}
#[test]
fn test_json_5_json_trailing_commaall_format_2_dfb12874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["json5"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{\n  '//': 'JSON5 allow \\`Infinity\\` and \\`NaN\\`',\n  numbers: [\n    Infinity,\n    -Infinity,\n    NaN,\n  ],\n  Infinity: NaN,\n  NaN: Infinity,\n  NaN: -Infinity,\n},\n{\n  '//': 'JSON5 numbers',\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n},\n{\n  '//': 'JSON5 strings',\nsingleQuotes: 'I can use \"double quotes\" here',\n  lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"JSON5 allow \\`Infinity\\` and \\`NaN\\`\",\n    numbers: [Infinity, -Infinity, NaN],\n    Infinity: NaN,\n    NaN: Infinity,\n    NaN: -Infinity,\n  },\n  {\n    \"//\": \"JSON5 numbers\",\n    hexadecimal: 0xdecaf,\n    leadingDecimalPoint: 0.8675309,\n    andTrailing: 8675309,\n    positiveSign: +1,\n  },\n  {\n    \"//\": \"JSON5 strings\",\n    singleQuotes: 'I can use \"double quotes\" here',\n    lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n  },\n]");
}
#[test]
fn test_json_5_json_trailing_commaes_5_format_1_dfb12874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .print_width(80)
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{\n  '//': 'JSON5 allow \\`Infinity\\` and \\`NaN\\`',\n  numbers: [\n    Infinity,\n    -Infinity,\n    NaN,\n  ],\n  Infinity: NaN,\n  NaN: Infinity,\n  NaN: -Infinity,\n},\n{\n  '//': 'JSON5 numbers',\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n},\n{\n  '//': 'JSON5 strings',\nsingleQuotes: 'I can use \"double quotes\" here',\n  lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"JSON5 allow \\`Infinity\\` and \\`NaN\\`\",\n    \"numbers\": [Infinity, -Infinity, NaN],\n    \"Infinity\": NaN,\n    \"NaN\": Infinity,\n    \"NaN\": -Infinity\n  },\n  {\n    \"//\": \"JSON5 numbers\",\n    \"hexadecimal\": 0xdecaf,\n    \"leadingDecimalPoint\": 0.8675309,\n    \"andTrailing\": 8675309,\n    \"positiveSign\": +1\n  },\n  {\n    \"//\": \"JSON5 strings\",\n    \"singleQuotes\": \"I can use \\\\\"double quotes\\\\\" here\",\n    \"lineBreaks\": \"Look, Mom! \\\\\nNo \\\\\\\\n's!\"\n  }\n]");
}
#[test]
fn test_json_5_json_trailing_commaes_5_format_2_dfb12874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{\n  '//': 'JSON5 allow \\`Infinity\\` and \\`NaN\\`',\n  numbers: [\n    Infinity,\n    -Infinity,\n    NaN,\n  ],\n  Infinity: NaN,\n  NaN: Infinity,\n  NaN: -Infinity,\n},\n{\n  '//': 'JSON5 numbers',\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n},\n{\n  '//': 'JSON5 strings',\nsingleQuotes: 'I can use \"double quotes\" here',\n  lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"JSON5 allow \\`Infinity\\` and \\`NaN\\`\",\n    numbers: [Infinity, -Infinity, NaN],\n    Infinity: NaN,\n    NaN: Infinity,\n    NaN: -Infinity,\n  },\n  {\n    \"//\": \"JSON5 numbers\",\n    hexadecimal: 0xdecaf,\n    leadingDecimalPoint: 0.8675309,\n    andTrailing: 8675309,\n    positiveSign: +1,\n  },\n  {\n    \"//\": \"JSON5 strings\",\n    singleQuotes: 'I can use \"double quotes\" here',\n    lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n  },\n]");
}
#[test]
fn test_json_5_json_format_1_dfb12874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{\n  '//': 'JSON5 allow \\`Infinity\\` and \\`NaN\\`',\n  numbers: [\n    Infinity,\n    -Infinity,\n    NaN,\n  ],\n  Infinity: NaN,\n  NaN: Infinity,\n  NaN: -Infinity,\n},\n{\n  '//': 'JSON5 numbers',\n  hexadecimal: 0xdecaf,\n  leadingDecimalPoint: .8675309, andTrailing: 8675309.,\n  positiveSign: +1,\n},\n{\n  '//': 'JSON5 strings',\nsingleQuotes: 'I can use \"double quotes\" here',\n  lineBreaks: \"Look, Mom! \\\\\nNo \\\\\\\\n's!\",\n}\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"JSON5 allow \\`Infinity\\` and \\`NaN\\`\",\n    \"numbers\": [\n      Infinity,\n      -Infinity,\n      NaN\n    ],\n    \"Infinity\": NaN,\n    \"NaN\": Infinity,\n    \"NaN\": -Infinity\n  },\n  {\n    \"//\": \"JSON5 numbers\",\n    \"hexadecimal\": 912559,\n    \"leadingDecimalPoint\": 0.8675309,\n    \"andTrailing\": 8675309,\n    \"positiveSign\": 1\n  },\n  {\n    \"//\": \"JSON5 strings\",\n    \"singleQuotes\": \"I can use \\\\\"double quotes\\\\\" here\",\n    \"lineBreaks\": \"Look, Mom! No \\\\\\\\n's!\"\n  }\n]");
}
#[test]
fn test_json_6_json_trailing_commaall_format_1_75bfd23c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{'//': 'Keyword \\`undefined\\`',\n    \"data\": [\n      {undefined: undefined},\n      undefined,\n      [undefined, ]\n    ]\n  },\n\n{'//': 'back-tick quoted strings',\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      {'as-object-value': \\`foo\\`}\n    ]\n  },\n\n{'//': 'String escapes ',\n    \"data\": [\n      '\\\\0', '\\\\xFF', '\\\\u00FF', \\`\\\\u{1F409}\\`\n    ]\n  },\n\n{'//': 'Numbers',\n    \"data\": [\n      0o123, 0b101010, 1e5, 123_456, 0xDeeD_Beef,\n      0123,\n      -Infinity, -NaN,\n      +1, -1,\n      +0o123, +0b101010, +1e5, +123_456, +0xDeeD_Beef,\n      -0o123, -0b101010, -1e5, -123_456, -0xDeeD_Beef,\n    ]\n  },\n\n{'//': 'empty members',\n    data: [\n      [,],\n      [1, , 2,,,,],\n      [1, , 2],\n      [1, , 2,]\n    ]\n  }\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"Keyword \\`undefined\\`\",\n    \"data\": [{ \"undefined\": undefined }, undefined, [undefined]]\n  },\n\n  {\n    \"//\": \"back-tick quoted strings\",\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      { \"as-object-value\": \\`foo\\` }\n    ]\n  },\n\n  { \"//\": \"String escapes \", \"data\": [\"\\\\0\", \"\\\\xFF\", \"\\\\u00FF\", \\`\\\\u{1F409}\\`] },\n\n  {\n    \"//\": \"Numbers\",\n    \"data\": [\n      0o123,\n      0b101010,\n      1e5,\n      123_456,\n      0xdeed_beef,\n      0123,\n      -Infinity,\n      -NaN,\n      +1,\n      -1,\n      +0o123,\n      +0b101010,\n      +1e5,\n      +123_456,\n      +0xdeed_beef,\n      -0o123,\n      -0b101010,\n      -1e5,\n      -123_456,\n      -0xdeed_beef\n    ]\n  },\n\n  { \"//\": \"empty members\", \"data\": [[,], [1, , 2, , , ,], [1, , 2], [1, , 2]] }\n]");
}
#[test]
fn test_json_6_json_trailing_commaall_format_2_75bfd23c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{'//': 'Keyword \\`undefined\\`',\n    \"data\": [\n      {undefined: undefined},\n      undefined,\n      [undefined, ]\n    ]\n  },\n\n{'//': 'back-tick quoted strings',\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      {'as-object-value': \\`foo\\`}\n    ]\n  },\n\n{'//': 'String escapes ',\n    \"data\": [\n      '\\\\0', '\\\\xFF', '\\\\u00FF', \\`\\\\u{1F409}\\`\n    ]\n  },\n\n{'//': 'Numbers',\n    \"data\": [\n      0o123, 0b101010, 1e5, 123_456, 0xDeeD_Beef,\n      0123,\n      -Infinity, -NaN,\n      +1, -1,\n      +0o123, +0b101010, +1e5, +123_456, +0xDeeD_Beef,\n      -0o123, -0b101010, -1e5, -123_456, -0xDeeD_Beef,\n    ]\n  },\n\n{'//': 'empty members',\n    data: [\n      [,],\n      [1, , 2,,,,],\n      [1, , 2],\n      [1, , 2,]\n    ]\n  }\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"Keyword \\`undefined\\`\",\n    data: [{ undefined: undefined }, undefined, [undefined]],\n  },\n\n  {\n    \"//\": \"back-tick quoted strings\",\n    data: [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      { \"as-object-value\": \\`foo\\` },\n    ],\n  },\n\n  { \"//\": \"String escapes \", data: [\"\\\\0\", \"\\\\xFF\", \"\\\\u00FF\", \\`\\\\u{1F409}\\`] },\n\n  {\n    \"//\": \"Numbers\",\n    data: [\n      0o123,\n      0b101010,\n      1e5,\n      123_456,\n      0xdeed_beef,\n      0123,\n      -Infinity,\n      -NaN,\n      +1,\n      -1,\n      +0o123,\n      +0b101010,\n      +1e5,\n      +123_456,\n      +0xdeed_beef,\n      -0o123,\n      -0b101010,\n      -1e5,\n      -123_456,\n      -0xdeed_beef,\n    ],\n  },\n\n  { \"//\": \"empty members\", data: [[,], [1, , 2, , , ,], [1, , 2], [1, , 2]] },\n]");
}
#[test]
fn test_json_6_json_trailing_commaes_5_format_1_75bfd23c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .trailing_comma("es5")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{'//': 'Keyword \\`undefined\\`',\n    \"data\": [\n      {undefined: undefined},\n      undefined,\n      [undefined, ]\n    ]\n  },\n\n{'//': 'back-tick quoted strings',\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      {'as-object-value': \\`foo\\`}\n    ]\n  },\n\n{'//': 'String escapes ',\n    \"data\": [\n      '\\\\0', '\\\\xFF', '\\\\u00FF', \\`\\\\u{1F409}\\`\n    ]\n  },\n\n{'//': 'Numbers',\n    \"data\": [\n      0o123, 0b101010, 1e5, 123_456, 0xDeeD_Beef,\n      0123,\n      -Infinity, -NaN,\n      +1, -1,\n      +0o123, +0b101010, +1e5, +123_456, +0xDeeD_Beef,\n      -0o123, -0b101010, -1e5, -123_456, -0xDeeD_Beef,\n    ]\n  },\n\n{'//': 'empty members',\n    data: [\n      [,],\n      [1, , 2,,,,],\n      [1, , 2],\n      [1, , 2,]\n    ]\n  }\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"Keyword \\`undefined\\`\",\n    \"data\": [{ \"undefined\": undefined }, undefined, [undefined]]\n  },\n\n  {\n    \"//\": \"back-tick quoted strings\",\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      { \"as-object-value\": \\`foo\\` }\n    ]\n  },\n\n  { \"//\": \"String escapes \", \"data\": [\"\\\\0\", \"\\\\xFF\", \"\\\\u00FF\", \\`\\\\u{1F409}\\`] },\n\n  {\n    \"//\": \"Numbers\",\n    \"data\": [\n      0o123,\n      0b101010,\n      1e5,\n      123_456,\n      0xdeed_beef,\n      0123,\n      -Infinity,\n      -NaN,\n      +1,\n      -1,\n      +0o123,\n      +0b101010,\n      +1e5,\n      +123_456,\n      +0xdeed_beef,\n      -0o123,\n      -0b101010,\n      -1e5,\n      -123_456,\n      -0xdeed_beef\n    ]\n  },\n\n  { \"//\": \"empty members\", \"data\": [[,], [1, , 2, , , ,], [1, , 2], [1, , 2]] }\n]");
}
#[test]
fn test_json_6_json_trailing_commaes_5_format_2_75bfd23c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .print_width(80)
        .parsers(vec!["json5"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{'//': 'Keyword \\`undefined\\`',\n    \"data\": [\n      {undefined: undefined},\n      undefined,\n      [undefined, ]\n    ]\n  },\n\n{'//': 'back-tick quoted strings',\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      {'as-object-value': \\`foo\\`}\n    ]\n  },\n\n{'//': 'String escapes ',\n    \"data\": [\n      '\\\\0', '\\\\xFF', '\\\\u00FF', \\`\\\\u{1F409}\\`\n    ]\n  },\n\n{'//': 'Numbers',\n    \"data\": [\n      0o123, 0b101010, 1e5, 123_456, 0xDeeD_Beef,\n      0123,\n      -Infinity, -NaN,\n      +1, -1,\n      +0o123, +0b101010, +1e5, +123_456, +0xDeeD_Beef,\n      -0o123, -0b101010, -1e5, -123_456, -0xDeeD_Beef,\n    ]\n  },\n\n{'//': 'empty members',\n    data: [\n      [,],\n      [1, , 2,,,,],\n      [1, , 2],\n      [1, , 2,]\n    ]\n  }\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"Keyword \\`undefined\\`\",\n    data: [{ undefined: undefined }, undefined, [undefined]],\n  },\n\n  {\n    \"//\": \"back-tick quoted strings\",\n    data: [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      { \"as-object-value\": \\`foo\\` },\n    ],\n  },\n\n  { \"//\": \"String escapes \", data: [\"\\\\0\", \"\\\\xFF\", \"\\\\u00FF\", \\`\\\\u{1F409}\\`] },\n\n  {\n    \"//\": \"Numbers\",\n    data: [\n      0o123,\n      0b101010,\n      1e5,\n      123_456,\n      0xdeed_beef,\n      0123,\n      -Infinity,\n      -NaN,\n      +1,\n      -1,\n      +0o123,\n      +0b101010,\n      +1e5,\n      +123_456,\n      +0xdeed_beef,\n      -0o123,\n      -0b101010,\n      -1e5,\n      -123_456,\n      -0xdeed_beef,\n    ],\n  },\n\n  { \"//\": \"empty members\", data: [[,], [1, , 2, , , ,], [1, , 2], [1, , 2]] },\n]");
}
#[test]
fn test_json_6_json_format_1_75bfd23c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json-stringify"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n{'//': 'Keyword \\`undefined\\`',\n    \"data\": [\n      {undefined: undefined},\n      undefined,\n      [undefined, ]\n    ]\n  },\n\n{'//': 'back-tick quoted strings',\n    \"data\": [\n      \\`\\`,\n      \\`foo\\`,\n      \\`\n  multiple-line\n\\`,\n      \\`\\\\u{1F409}\\\\\\`'\"\\\\\\${}\\`,\n      {'as-object-value': \\`foo\\`}\n    ]\n  },\n\n{'//': 'String escapes ',\n    \"data\": [\n      '\\\\0', '\\\\xFF', '\\\\u00FF', \\`\\\\u{1F409}\\`\n    ]\n  },\n\n{'//': 'Numbers',\n    \"data\": [\n      0o123, 0b101010, 1e5, 123_456, 0xDeeD_Beef,\n      0123,\n      -Infinity, -NaN,\n      +1, -1,\n      +0o123, +0b101010, +1e5, +123_456, +0xDeeD_Beef,\n      -0o123, -0b101010, -1e5, -123_456, -0xDeeD_Beef,\n    ]\n  },\n\n{'//': 'empty members',\n    data: [\n      [,],\n      [1, , 2,,,,],\n      [1, , 2],\n      [1, , 2,]\n    ]\n  }\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  {\n    \"//\": \"Keyword \\`undefined\\`\",\n    \"data\": [\n      {\n        \"undefined\": undefined\n      },\n      undefined,\n      [\n        undefined\n      ]\n    ]\n  },\n  {\n    \"//\": \"back-tick quoted strings\",\n    \"data\": [\n      \"\",\n      \"foo\",\n      \"\\\\n  multiple-line\\\\n\",\n      \"🐉\\`'\\\\\"\\${}\",\n      {\n        \"as-object-value\": \"foo\"\n      }\n    ]\n  },\n  {\n    \"//\": \"String escapes \",\n    \"data\": [\n      \"\\\\u0000\",\n      \"ÿ\",\n      \"ÿ\",\n      \"🐉\"\n    ]\n  },\n  {\n    \"//\": \"Numbers\",\n    \"data\": [\n      83,\n      42,\n      100000,\n      123456,\n      3740122863,\n      83,\n      -Infinity,\n      -NaN,\n      1,\n      -1,\n      83,\n      42,\n      100000,\n      123456,\n      3740122863,\n      -83,\n      -42,\n      -100000,\n      -123456,\n      -3740122863\n    ]\n  },\n  {\n    \"//\": \"empty members\",\n    \"data\": [\n      [\n        null\n      ],\n      [\n        1,\n        null,\n        2,\n        null,\n        null,\n        null\n      ],\n      [\n        1,\n        null,\n        2\n      ],\n      [\n        1,\n        null,\n        2\n      ]\n    ]\n  }\n]");
}
#[test]
fn test_key_value_json_trailing_commaall_format_1_2c11060e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}");
}
#[test]
fn test_key_value_json_trailing_commaall_format_2_2c11060e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .trailing_comma("all")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  string: \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n  stringstringstringstringstringstringstringstring: \"stringstringstringstringstringstringstringstring\",\n  stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring: \"string\",\n}");
}
#[test]
fn test_key_value_json_trailing_commaes_5_format_1_2c11060e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}");
}
#[test]
fn test_key_value_json_trailing_commaes_5_format_2_2c11060e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .trailing_comma("es5")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  string: \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n  stringstringstringstringstringstringstringstring: \"stringstringstringstringstringstringstringstring\",\n  stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring: \"string\",\n}");
}
#[test]
fn test_key_value_json_format_1_2c11060e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n    \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"string\": \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstring\": \"stringstringstringstringstringstringstringstring\",\n  \"stringstringstringstringstringstringstringstringstringstringstringstringstringstringstring\": \"string\"\n}");
}
#[test]
fn test_multi_line_json_trailing_commaall_format_1_119b0e00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",\n1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ \"key1\": [true, false, null], \"key2\": { \"key3\": [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_multi_line_json_trailing_commaall_format_2_119b0e00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",\n1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ key1: [true, false, null], key2: { key3: [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_multi_line_json_trailing_commaes_5_format_1_119b0e00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .print_width(80)
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",\n1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ \"key1\": [true, false, null], \"key2\": { \"key3\": [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_multi_line_json_trailing_commaes_5_format_2_119b0e00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",\n1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ key1: [true, false, null], key2: { key3: [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_multi_line_json_format_1_119b0e00() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",\n1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"key1\": [\n    true,\n    false,\n    null\n  ],\n  \"key2\": {\n    \"key3\": [\n      1,\n      2,\n      \"3\",\n      10000000000,\n      0.001\n    ]\n  }\n}");
}
#[test]
fn test_null_json_trailing_commaall_format_1_ec41de1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("null");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "null");
}
#[test]
fn test_null_json_trailing_commaall_format_2_ec41de1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("null");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "null");
}
#[test]
fn test_null_json_trailing_commaes_5_format_1_ec41de1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("null");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "null");
}
#[test]
fn test_null_json_trailing_commaes_5_format_2_ec41de1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("null");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "null");
}
#[test]
fn test_null_json_format_1_ec41de1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json-stringify"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("null");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "null");
}
#[test]
fn test_number_json_trailing_commaall_format_1_465981b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0");
}
#[test]
fn test_number_json_trailing_commaall_format_2_465981b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0");
}
#[test]
fn test_number_json_trailing_commaes_5_format_1_465981b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0");
}
#[test]
fn test_number_json_trailing_commaes_5_format_2_465981b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json5"])
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0");
}
#[test]
fn test_number_json_format_1_465981b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("0");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "0");
}
#[test]
fn test_pass_1_json_trailing_commaall_format_1_15e874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    \"JSON Test Pattern pass1\",\n    {\"object with 1 member\":[\"array with 1 element\"]},\n    {},\n    [],\n    -42,\n    true,\n    false,\n    null,\n    {\n        \"integer\": 1234567890,\n        \"real\": -9876.543210,\n        \"e\": 0.123456789e-12,\n        \"E\": 1.234567890E+34,\n        \"\":  23456789012E66,\n        \"zero\": 0,\n        \"one\": 1,\n        \"space\": \" \",\n        \"quote\": \"\\\\\"\",\n        \"backslash\": \"\\\\\\\\\",\n        \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n        \"slash\": \"/ & \\\\/\",\n        \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n        \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n        \"digit\": \"0123456789\",\n        \"0123456789\": \"digit\",\n        \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n        \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n        \"true\": true,\n        \"false\": false,\n        \"null\": null,\n        \"array\":[  ],\n        \"object\":{  },\n        \"address\": \"50 St. James Street\",\n        \"url\": \"http://www.JSON.org/\",\n        \"comment\": \"// /* <!-- --\",\n        \"# -- --> */\": \" \",\n        \" s p a c e d \" :[1,2 , 3\n\n,\n\n4 , 5        ,          6           ,7        ],\"compact\":[1,2,3,4,5,6,7],\n        \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n        \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n        \"\\\\/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\"\n: \"A key can be any string\"\n    },\n    0.5 ,98.6\n,\n99.44\n,\n\n1066,\n1e1,\n0.1e1,\n1e-1,\n1e00,2e+00,2e-00\n,\"rosebud\"]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"JSON Test Pattern pass1\",\n  { \"object with 1 member\": [\"array with 1 element\"] },\n  {},\n  [],\n  -42,\n  true,\n  false,\n  null,\n  {\n    \"integer\": 1234567890,\n    \"real\": -9876.54321,\n    \"e\": 0.123456789e-12,\n    \"E\": 1.23456789e34,\n    \"\": 23456789012e66,\n    \"zero\": 0,\n    \"one\": 1,\n    \"space\": \" \",\n    \"quote\": \"\\\\\"\",\n    \"backslash\": \"\\\\\\\\\",\n    \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n    \"slash\": \"/ & /\",\n    \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n    \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n    \"digit\": \"0123456789\",\n    \"0123456789\": \"digit\",\n    \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n    \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n    \"true\": true,\n    \"false\": false,\n    \"null\": null,\n    \"array\": [],\n    \"object\": {},\n    \"address\": \"50 St. James Street\",\n    \"url\": \"http://www.JSON.org/\",\n    \"comment\": \"// /* <!-- --\",\n    \"# -- --> */\": \" \",\n    \" s p a c e d \": [\n      1, 2, 3,\n\n      4, 5, 6, 7\n    ],\n    \"compact\": [1, 2, 3, 4, 5, 6, 7],\n    \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n    \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n    \"/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\": \"A key can be any string\"\n  },\n  0.5,\n  98.6,\n  99.44,\n\n  1066,\n  1e1,\n  0.1e1,\n  1e-1,\n  1,\n  2,\n  2,\n  \"rosebud\"\n]");
}
#[test]
fn test_pass_1_json_trailing_commaall_format_2_15e874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .trailing_comma("all")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    \"JSON Test Pattern pass1\",\n    {\"object with 1 member\":[\"array with 1 element\"]},\n    {},\n    [],\n    -42,\n    true,\n    false,\n    null,\n    {\n        \"integer\": 1234567890,\n        \"real\": -9876.543210,\n        \"e\": 0.123456789e-12,\n        \"E\": 1.234567890E+34,\n        \"\":  23456789012E66,\n        \"zero\": 0,\n        \"one\": 1,\n        \"space\": \" \",\n        \"quote\": \"\\\\\"\",\n        \"backslash\": \"\\\\\\\\\",\n        \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n        \"slash\": \"/ & \\\\/\",\n        \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n        \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n        \"digit\": \"0123456789\",\n        \"0123456789\": \"digit\",\n        \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n        \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n        \"true\": true,\n        \"false\": false,\n        \"null\": null,\n        \"array\":[  ],\n        \"object\":{  },\n        \"address\": \"50 St. James Street\",\n        \"url\": \"http://www.JSON.org/\",\n        \"comment\": \"// /* <!-- --\",\n        \"# -- --> */\": \" \",\n        \" s p a c e d \" :[1,2 , 3\n\n,\n\n4 , 5        ,          6           ,7        ],\"compact\":[1,2,3,4,5,6,7],\n        \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n        \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n        \"\\\\/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\"\n: \"A key can be any string\"\n    },\n    0.5 ,98.6\n,\n99.44\n,\n\n1066,\n1e1,\n0.1e1,\n1e-1,\n1e00,2e+00,2e-00\n,\"rosebud\"]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"JSON Test Pattern pass1\",\n  { \"object with 1 member\": [\"array with 1 element\"] },\n  {},\n  [],\n  -42,\n  true,\n  false,\n  null,\n  {\n    integer: 1234567890,\n    real: -9876.54321,\n    e: 0.123456789e-12,\n    E: 1.23456789e34,\n    \"\": 23456789012e66,\n    zero: 0,\n    one: 1,\n    space: \" \",\n    quote: '\"',\n    backslash: \"\\\\\\\\\",\n    controls: \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n    slash: \"/ & /\",\n    alpha: \"abcdefghijklmnopqrstuvwyz\",\n    ALPHA: \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n    digit: \"0123456789\",\n    \"0123456789\": \"digit\",\n    special: \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n    hex: \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n    true: true,\n    false: false,\n    null: null,\n    array: [],\n    object: {},\n    address: \"50 St. James Street\",\n    url: \"http://www.JSON.org/\",\n    comment: \"// /* <!-- --\",\n    \"# -- --> */\": \" \",\n    \" s p a c e d \": [\n      1, 2, 3,\n\n      4, 5, 6, 7,\n    ],\n    compact: [1, 2, 3, 4, 5, 6, 7],\n    jsontext: '{\"object with 1 member\":[\"array with 1 element\"]}',\n    quotes: \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n    \"/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\": \"A key can be any string\",\n  },\n  0.5,\n  98.6,\n  99.44,\n\n  1066,\n  1e1,\n  0.1e1,\n  1e-1,\n  1,\n  2,\n  2,\n  \"rosebud\",\n]");
}
#[test]
fn test_pass_1_json_trailing_commaes_5_format_1_15e874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    \"JSON Test Pattern pass1\",\n    {\"object with 1 member\":[\"array with 1 element\"]},\n    {},\n    [],\n    -42,\n    true,\n    false,\n    null,\n    {\n        \"integer\": 1234567890,\n        \"real\": -9876.543210,\n        \"e\": 0.123456789e-12,\n        \"E\": 1.234567890E+34,\n        \"\":  23456789012E66,\n        \"zero\": 0,\n        \"one\": 1,\n        \"space\": \" \",\n        \"quote\": \"\\\\\"\",\n        \"backslash\": \"\\\\\\\\\",\n        \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n        \"slash\": \"/ & \\\\/\",\n        \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n        \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n        \"digit\": \"0123456789\",\n        \"0123456789\": \"digit\",\n        \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n        \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n        \"true\": true,\n        \"false\": false,\n        \"null\": null,\n        \"array\":[  ],\n        \"object\":{  },\n        \"address\": \"50 St. James Street\",\n        \"url\": \"http://www.JSON.org/\",\n        \"comment\": \"// /* <!-- --\",\n        \"# -- --> */\": \" \",\n        \" s p a c e d \" :[1,2 , 3\n\n,\n\n4 , 5        ,          6           ,7        ],\"compact\":[1,2,3,4,5,6,7],\n        \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n        \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n        \"\\\\/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\"\n: \"A key can be any string\"\n    },\n    0.5 ,98.6\n,\n99.44\n,\n\n1066,\n1e1,\n0.1e1,\n1e-1,\n1e00,2e+00,2e-00\n,\"rosebud\"]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"JSON Test Pattern pass1\",\n  { \"object with 1 member\": [\"array with 1 element\"] },\n  {},\n  [],\n  -42,\n  true,\n  false,\n  null,\n  {\n    \"integer\": 1234567890,\n    \"real\": -9876.54321,\n    \"e\": 0.123456789e-12,\n    \"E\": 1.23456789e34,\n    \"\": 23456789012e66,\n    \"zero\": 0,\n    \"one\": 1,\n    \"space\": \" \",\n    \"quote\": \"\\\\\"\",\n    \"backslash\": \"\\\\\\\\\",\n    \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n    \"slash\": \"/ & /\",\n    \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n    \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n    \"digit\": \"0123456789\",\n    \"0123456789\": \"digit\",\n    \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n    \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n    \"true\": true,\n    \"false\": false,\n    \"null\": null,\n    \"array\": [],\n    \"object\": {},\n    \"address\": \"50 St. James Street\",\n    \"url\": \"http://www.JSON.org/\",\n    \"comment\": \"// /* <!-- --\",\n    \"# -- --> */\": \" \",\n    \" s p a c e d \": [\n      1, 2, 3,\n\n      4, 5, 6, 7\n    ],\n    \"compact\": [1, 2, 3, 4, 5, 6, 7],\n    \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n    \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n    \"/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\": \"A key can be any string\"\n  },\n  0.5,\n  98.6,\n  99.44,\n\n  1066,\n  1e1,\n  0.1e1,\n  1e-1,\n  1,\n  2,\n  2,\n  \"rosebud\"\n]");
}
#[test]
fn test_pass_1_json_trailing_commaes_5_format_2_15e874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    \"JSON Test Pattern pass1\",\n    {\"object with 1 member\":[\"array with 1 element\"]},\n    {},\n    [],\n    -42,\n    true,\n    false,\n    null,\n    {\n        \"integer\": 1234567890,\n        \"real\": -9876.543210,\n        \"e\": 0.123456789e-12,\n        \"E\": 1.234567890E+34,\n        \"\":  23456789012E66,\n        \"zero\": 0,\n        \"one\": 1,\n        \"space\": \" \",\n        \"quote\": \"\\\\\"\",\n        \"backslash\": \"\\\\\\\\\",\n        \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n        \"slash\": \"/ & \\\\/\",\n        \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n        \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n        \"digit\": \"0123456789\",\n        \"0123456789\": \"digit\",\n        \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n        \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n        \"true\": true,\n        \"false\": false,\n        \"null\": null,\n        \"array\":[  ],\n        \"object\":{  },\n        \"address\": \"50 St. James Street\",\n        \"url\": \"http://www.JSON.org/\",\n        \"comment\": \"// /* <!-- --\",\n        \"# -- --> */\": \" \",\n        \" s p a c e d \" :[1,2 , 3\n\n,\n\n4 , 5        ,          6           ,7        ],\"compact\":[1,2,3,4,5,6,7],\n        \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n        \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n        \"\\\\/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\"\n: \"A key can be any string\"\n    },\n    0.5 ,98.6\n,\n99.44\n,\n\n1066,\n1e1,\n0.1e1,\n1e-1,\n1e00,2e+00,2e-00\n,\"rosebud\"]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"JSON Test Pattern pass1\",\n  { \"object with 1 member\": [\"array with 1 element\"] },\n  {},\n  [],\n  -42,\n  true,\n  false,\n  null,\n  {\n    integer: 1234567890,\n    real: -9876.54321,\n    e: 0.123456789e-12,\n    E: 1.23456789e34,\n    \"\": 23456789012e66,\n    zero: 0,\n    one: 1,\n    space: \" \",\n    quote: '\"',\n    backslash: \"\\\\\\\\\",\n    controls: \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n    slash: \"/ & /\",\n    alpha: \"abcdefghijklmnopqrstuvwyz\",\n    ALPHA: \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n    digit: \"0123456789\",\n    \"0123456789\": \"digit\",\n    special: \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n    hex: \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n    true: true,\n    false: false,\n    null: null,\n    array: [],\n    object: {},\n    address: \"50 St. James Street\",\n    url: \"http://www.JSON.org/\",\n    comment: \"// /* <!-- --\",\n    \"# -- --> */\": \" \",\n    \" s p a c e d \": [\n      1, 2, 3,\n\n      4, 5, 6, 7,\n    ],\n    compact: [1, 2, 3, 4, 5, 6, 7],\n    jsontext: '{\"object with 1 member\":[\"array with 1 element\"]}',\n    quotes: \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n    \"/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\": \"A key can be any string\",\n  },\n  0.5,\n  98.6,\n  99.44,\n\n  1066,\n  1e1,\n  0.1e1,\n  1e-1,\n  1,\n  2,\n  2,\n  \"rosebud\",\n]");
}
#[test]
fn test_pass_1_json_format_1_15e874f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json-stringify"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    \"JSON Test Pattern pass1\",\n    {\"object with 1 member\":[\"array with 1 element\"]},\n    {},\n    [],\n    -42,\n    true,\n    false,\n    null,\n    {\n        \"integer\": 1234567890,\n        \"real\": -9876.543210,\n        \"e\": 0.123456789e-12,\n        \"E\": 1.234567890E+34,\n        \"\":  23456789012E66,\n        \"zero\": 0,\n        \"one\": 1,\n        \"space\": \" \",\n        \"quote\": \"\\\\\"\",\n        \"backslash\": \"\\\\\\\\\",\n        \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n        \"slash\": \"/ & \\\\/\",\n        \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n        \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n        \"digit\": \"0123456789\",\n        \"0123456789\": \"digit\",\n        \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n        \"hex\": \"\\\\u0123\\\\u4567\\\\u89AB\\\\uCDEF\\\\uabcd\\\\uef4A\",\n        \"true\": true,\n        \"false\": false,\n        \"null\": null,\n        \"array\":[  ],\n        \"object\":{  },\n        \"address\": \"50 St. James Street\",\n        \"url\": \"http://www.JSON.org/\",\n        \"comment\": \"// /* <!-- --\",\n        \"# -- --> */\": \" \",\n        \" s p a c e d \" :[1,2 , 3\n\n,\n\n4 , 5        ,          6           ,7        ],\"compact\":[1,2,3,4,5,6,7],\n        \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n        \"quotes\": \"&#34; \\\\u0022 %22 0x22 034 &#x22;\",\n        \"\\\\/\\\\\\\\\\\\\"\\\\uCAFE\\\\uBABE\\\\uAB98\\\\uFCDE\\\\ubcda\\\\uef4A\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\"\n: \"A key can be any string\"\n    },\n    0.5 ,98.6\n,\n99.44\n,\n\n1066,\n1e1,\n0.1e1,\n1e-1,\n1e00,2e+00,2e-00\n,\"rosebud\"]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"JSON Test Pattern pass1\",\n  {\n    \"object with 1 member\": [\n      \"array with 1 element\"\n    ]\n  },\n  {},\n  [],\n  -42,\n  true,\n  false,\n  null,\n  {\n    \"integer\": 1234567890,\n    \"real\": -9876.54321,\n    \"e\": 1.23456789e-13,\n    \"E\": 1.23456789e+34,\n    \"\": 2.3456789012e+76,\n    \"zero\": 0,\n    \"one\": 1,\n    \"space\": \" \",\n    \"quote\": \"\\\\\"\",\n    \"backslash\": \"\\\\\\\\\",\n    \"controls\": \"\\\\b\\\\f\\\\n\\\\r\\\\t\",\n    \"slash\": \"/ & /\",\n    \"alpha\": \"abcdefghijklmnopqrstuvwyz\",\n    \"ALPHA\": \"ABCDEFGHIJKLMNOPQRSTUVWYZ\",\n    \"digit\": \"0123456789\",\n    \"0123456789\": \"digit\",\n    \"special\": \"\\`1~!@#$%^&*()_+-={':[,]}|;.</>?\",\n    \"hex\": \"ģ䕧覫췯ꯍ\u{ef4a}\",\n    \"true\": true,\n    \"false\": false,\n    \"null\": null,\n    \"array\": [],\n    \"object\": {},\n    \"address\": \"50 St. James Street\",\n    \"url\": \"http://www.JSON.org/\",\n    \"comment\": \"// /* <!-- --\",\n    \"# -- --> */\": \" \",\n    \" s p a c e d \": [\n      1,\n      2,\n      3,\n      4,\n      5,\n      6,\n      7\n    ],\n    \"compact\": [\n      1,\n      2,\n      3,\n      4,\n      5,\n      6,\n      7\n    ],\n    \"jsontext\": \"{\\\\\"object with 1 member\\\\\":[\\\\\"array with 1 element\\\\\"]}\",\n    \"quotes\": \"&#34; \\\\\" %22 0x22 034 &#x22;\",\n    \"/\\\\\\\\\\\\\"쫾몾ꮘﳞ볚\u{ef4a}\\\\b\\\\f\\\\n\\\\r\\\\t\\`1~!@#$%^&*()_+-=[]{}|;:',./<>?\": \"A key can be any string\"\n  },\n  0.5,\n  98.6,\n  99.44,\n  1066,\n  10,\n  1,\n  0.1,\n  1,\n  2,\n  2,\n  \"rosebud\"\n]");
}
#[test]
fn test_positive_number_json_trailing_commaall_format_1_0443f321() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+123");
}
#[test]
fn test_positive_number_json_trailing_commaall_format_2_0443f321() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+123");
}
#[test]
fn test_positive_number_json_trailing_commaes_5_format_1_0443f321() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+123");
}
#[test]
fn test_positive_number_json_trailing_commaes_5_format_2_0443f321() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json5"])
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "+123");
}
#[test]
fn test_positive_number_json_format_1_0443f321() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("+123");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "123");
}
#[test]
fn test_property_key_json_trailing_commaall_format_1_0b8424b6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    a: '',\n    null: '',\n    true: '',\n    \"string\": \"\",\n    0: '',\n    1e2: '',\n    1.0e+2: '',\n    .10e+2: '',\n    1e-2: '',\n    .1e-2: '',\n    0.1e+2: '',\n    1.0: '',\n    1.00000: '',\n    .1: '',\n    .100000: '',\n    0.1: '',\n    0.100000: '',\n    999999999999999999999999999999: '',\n    .000000000000000000000000000001: '',\n    0.000000000000000000000000000001: '',\n    1e999999999999999999999999999999: '',\n    1_2_3: '',\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"a\": \"\",\n  \"null\": \"\",\n  \"true\": \"\",\n  \"string\": \"\",\n  \"0\": \"\",\n  1e2: \"\",\n  1.0e2: \"\",\n  0.1e2: \"\",\n  1e-2: \"\",\n  0.1e-2: \"\",\n  0.1e2: \"\",\n  1.0: \"\",\n  1.0: \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  999999999999999999999999999999: \"\",\n  0.000000000000000000000000000001: \"\",\n  0.000000000000000000000000000001: \"\",\n  1e999999999999999999999999999999: \"\",\n  1_2_3: \"\"\n}");
}
#[test]
fn test_property_key_json_trailing_commaall_format_2_0b8424b6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("all")
        .print_width(80)
        .parsers(vec!["json5"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    a: '',\n    null: '',\n    true: '',\n    \"string\": \"\",\n    0: '',\n    1e2: '',\n    1.0e+2: '',\n    .10e+2: '',\n    1e-2: '',\n    .1e-2: '',\n    0.1e+2: '',\n    1.0: '',\n    1.00000: '',\n    .1: '',\n    .100000: '',\n    0.1: '',\n    0.100000: '',\n    999999999999999999999999999999: '',\n    .000000000000000000000000000001: '',\n    0.000000000000000000000000000001: '',\n    1e999999999999999999999999999999: '',\n    1_2_3: '',\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  a: \"\",\n  null: \"\",\n  true: \"\",\n  string: \"\",\n  0: \"\",\n  1e2: \"\",\n  1.0e2: \"\",\n  0.1e2: \"\",\n  1e-2: \"\",\n  0.1e-2: \"\",\n  0.1e2: \"\",\n  1.0: \"\",\n  1.0: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  999999999999999999999999999999: \"\",\n  0.000000000000000000000000000001: \"\",\n  0.000000000000000000000000000001: \"\",\n  1e999999999999999999999999999999: \"\",\n  1_2_3: \"\",\n}");
}
#[test]
fn test_property_key_json_trailing_commaes_5_format_1_0b8424b6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    a: '',\n    null: '',\n    true: '',\n    \"string\": \"\",\n    0: '',\n    1e2: '',\n    1.0e+2: '',\n    .10e+2: '',\n    1e-2: '',\n    .1e-2: '',\n    0.1e+2: '',\n    1.0: '',\n    1.00000: '',\n    .1: '',\n    .100000: '',\n    0.1: '',\n    0.100000: '',\n    999999999999999999999999999999: '',\n    .000000000000000000000000000001: '',\n    0.000000000000000000000000000001: '',\n    1e999999999999999999999999999999: '',\n    1_2_3: '',\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"a\": \"\",\n  \"null\": \"\",\n  \"true\": \"\",\n  \"string\": \"\",\n  \"0\": \"\",\n  1e2: \"\",\n  1.0e2: \"\",\n  0.1e2: \"\",\n  1e-2: \"\",\n  0.1e-2: \"\",\n  0.1e2: \"\",\n  1.0: \"\",\n  1.0: \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  999999999999999999999999999999: \"\",\n  0.000000000000000000000000000001: \"\",\n  0.000000000000000000000000000001: \"\",\n  1e999999999999999999999999999999: \"\",\n  1_2_3: \"\"\n}");
}
#[test]
fn test_property_key_json_trailing_commaes_5_format_2_0b8424b6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json5"])
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    a: '',\n    null: '',\n    true: '',\n    \"string\": \"\",\n    0: '',\n    1e2: '',\n    1.0e+2: '',\n    .10e+2: '',\n    1e-2: '',\n    .1e-2: '',\n    0.1e+2: '',\n    1.0: '',\n    1.00000: '',\n    .1: '',\n    .100000: '',\n    0.1: '',\n    0.100000: '',\n    999999999999999999999999999999: '',\n    .000000000000000000000000000001: '',\n    0.000000000000000000000000000001: '',\n    1e999999999999999999999999999999: '',\n    1_2_3: '',\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  a: \"\",\n  null: \"\",\n  true: \"\",\n  string: \"\",\n  0: \"\",\n  1e2: \"\",\n  1.0e2: \"\",\n  0.1e2: \"\",\n  1e-2: \"\",\n  0.1e-2: \"\",\n  0.1e2: \"\",\n  1.0: \"\",\n  1.0: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  0.1: \"\",\n  999999999999999999999999999999: \"\",\n  0.000000000000000000000000000001: \"\",\n  0.000000000000000000000000000001: \"\",\n  1e999999999999999999999999999999: \"\",\n  1_2_3: \"\",\n}");
}
#[test]
fn test_property_key_json_format_1_0b8424b6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json-stringify"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("{\n    a: '',\n    null: '',\n    true: '',\n    \"string\": \"\",\n    0: '',\n    1e2: '',\n    1.0e+2: '',\n    .10e+2: '',\n    1e-2: '',\n    .1e-2: '',\n    0.1e+2: '',\n    1.0: '',\n    1.00000: '',\n    .1: '',\n    .100000: '',\n    0.1: '',\n    0.100000: '',\n    999999999999999999999999999999: '',\n    .000000000000000000000000000001: '',\n    0.000000000000000000000000000001: '',\n    1e999999999999999999999999999999: '',\n    1_2_3: '',\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"a\": \"\",\n  \"null\": \"\",\n  \"true\": \"\",\n  \"string\": \"\",\n  \"0\": \"\",\n  \"100\": \"\",\n  \"100\": \"\",\n  \"10\": \"\",\n  \"0.01\": \"\",\n  \"0.001\": \"\",\n  \"10\": \"\",\n  \"1\": \"\",\n  \"1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"0.1\": \"\",\n  \"1e+30\": \"\",\n  \"1e-30\": \"\",\n  \"1e-30\": \"\",\n  \"Infinity\": \"\",\n  \"123\": \"\"\n}");
}
#[test]
fn test_single_line_json_trailing_commaall_format_1_d51b2a4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ \"key1\": [true, false, null], \"key2\": { \"key3\": [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_single_line_json_trailing_commaall_format_2_d51b2a4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ key1: [true, false, null], key2: { key3: [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_single_line_json_trailing_commaes_5_format_1_d51b2a4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ \"key1\": [true, false, null], \"key2\": { \"key3\": [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_single_line_json_trailing_commaes_5_format_2_d51b2a4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "{ key1: [true, false, null], key2: { key3: [1, 2, \"3\", 1e10, 1e-3] } }"
    );
}
#[test]
fn test_single_line_json_format_1_d51b2a4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json-stringify"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("{\"key1\":[true,false,null],\"key2\":{\"key3\":[1,2,\"3\",1e10,1e-3]}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  \"key1\": [\n    true,\n    false,\n    null\n  ],\n  \"key2\": {\n    \"key3\": [\n      1,\n      2,\n      \"3\",\n      10000000000,\n      0.001\n    ]\n  }\n}");
}
#[test]
fn test_single_quote_json_trailing_commaall_format_1_a7db9251() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'hello'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"hello\"");
}
#[test]
fn test_single_quote_json_trailing_commaall_format_2_a7db9251() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json5"])
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'hello'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"hello\"");
}
#[test]
fn test_single_quote_json_trailing_commaes_5_format_1_a7db9251() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'hello'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"hello\"");
}
#[test]
fn test_single_quote_json_trailing_commaes_5_format_2_a7db9251() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .trailing_comma("es5")
        .parsers(vec!["json5"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'hello'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"hello\"");
}
#[test]
fn test_single_quote_json_format_1_a7db9251() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("'hello'");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"hello\"");
}
#[test]
fn test_string_json_trailing_commaall_format_1_ad9f2787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("all")
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"string\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"string\"");
}
#[test]
fn test_string_json_trailing_commaall_format_2_ad9f2787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"string\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"string\"");
}
#[test]
fn test_string_json_trailing_commaes_5_format_1_ad9f2787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .trailing_comma("es5")
        .parsers(vec!["json"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"string\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"string\"");
}
#[test]
fn test_string_json_trailing_commaes_5_format_2_ad9f2787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["json5"])
        .trailing_comma("es5")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"string\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"string\"");
}
#[test]
fn test_string_json_format_1_ad9f2787() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["json-stringify"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("\"string\"");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"string\"");
}
