#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_bigint_key_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_bigint_key_js_format_1_355b539e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a = {1n: \"\"}\na = {1n() {}}\na = {get 1n() {}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a = { 1n: \"\" };\na = { 1n() {} };\na = { get 1n() {} };"
    );
}
#[test]
fn test_escape_sequence_key_js_format_1_517e89a7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #6235\nconst a = {\n  '\\\\u2139': 'why \"\\\\\\\\u2139\" is converted to \"i\"?',\n};\n\nconst b = {\n  \"\\\\x66\\\\x69\\\\x73\\\\x6b\\\\x65\\\\x72\": \"\\\\x66\\\\x69\\\\x73\\\\x6b\\\\x65\\\\x72\",\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #6235\nconst a = {\n  \"\\\\u2139\": 'why \"\\\\\\\\u2139\" is converted to \"i\"?',\n};\n\nconst b = {\n  \"\\\\x66\\\\x69\\\\x73\\\\x6b\\\\x65\\\\x72\": \"\\\\x66\\\\x69\\\\x73\\\\x6b\\\\x65\\\\x72\",\n};");
}
#[test]
fn test_expand_js_format_1_829b7b7e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const Component1 = ({ props }) => (\n  <Text>Test</Text>\n);\n\nconst Component2 = ({\n  props\n}) => (\n  <Text>Test</Text>\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Component1 = ({ props }) => <Text>Test</Text>;\n\nconst Component2 = ({ props }) => <Text>Test</Text>;");
}
#[test]
fn test_expression_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_expression_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_expression_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_expression_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_expression_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_expression_js_format_1_12f29dd7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("() => ({}\\`\\`);\n({})\\`\\`;\na = () => ({}).x;\n({} && a, b);\n({}::b, 0);\n({}::b()\\`\\`[''].c++ && 0 ? 0 : 0, 0);\n({}(), 0);\n({} = 0);\n(({} = 0), 1);\n\nconst a1 = {\n  someKey:\n    (shortName, shortName)\n};\n\nconst a2 = {\n  someKey:\n    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName, shortName)\n};\n\nconst a3 = {\n  someKey:\n    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName, longLongLongLongLongLongLongLongLongLongLongLongLongLongName, longLongLongLongLongLongLongLongLongLongLongLongLongLongName)\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "() => ({})\\`\\`;\n({})\\`\\`;\na = () => ({}).x;\n({}) && a, b;\n({})::b, 0;\n({})::b()\\`\\`[\"\"].c++ && 0 ? 0 : 0, 0;\n({})(), 0;\n({} = 0);\n({} = 0), 1;\n\nconst a1 = {\n  someKey: (shortName, shortName),\n};\n\nconst a2 = {\n  someKey:\n    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName, shortName),\n};\n\nconst a3 = {\n  someKey:\n    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName,\n    longLongLongLongLongLongLongLongLongLongLongLongLongLongName,\n    longLongLongLongLongLongLongLongLongLongLongLongLongLongName),\n};");
}
#[test]
fn test_getter_setter_js_format_1_49d16d6b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "({ set x(foo) {} });\n({ get x() { return 1 } });\n({ set x(a) {} });\n({ get x() {} });",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "({ set x(foo) {} });\n({\n  get x() {\n    return 1;\n  },\n});\n({ set x(a) {} });\n({ get x() {} });");
}
#[test]
fn test_method_js_format_1_2c3f870f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a = { f() {} }");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a = { f() {} };");
}
#[test]
fn test_range_js_format_1_a8655c14() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("group(\n  concat([\n    \"(\",\n    indent(\n      options.tabWidth,\n      concat([line, join(concat([\",\", line]), printed)])\n    ),\n    options.trailingComma ? \",\" : \"\",\n    line,\n    \")\"\n  ]),\n  {shouldBreak: true}\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "group(\n  concat([\n    \"(\",\n    indent(\n      options.tabWidth,\n      concat([line, join(concat([\",\", line]), printed)]),\n    ),\n    options.trailingComma ? \",\" : \"\",\n    line,\n    \")\",\n  ]),\n  { shouldBreak: true },\n);");
}
#[test]
fn test_right_break_js_format_1_5e631f1c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const blablah =\n  \"aldkfkladfskladklsfkladklfkaldfadfkdaf\" +\n  \"adlfasdklfkldsklfakldsfkladsfkadsfladsfa\" +\n  \"dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf\";\n\nconst k = {\n  blablah: \"aldkfkladfskladklsfkladklfkaldfadfkdaf\" +\n    \"adlfasdklfkldsklfakldsfkladsfkadsfladsfa\" +\n    \"dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf\"\n};\n\nsomethingThatsAReallyLongPropName =\n  this.props.cardType === AwesomizerCardEnum.SEEFIRST;\n\nconst o = {\n  somethingThatsAReallyLongPropName:\n    this.props.cardType === AwesomizerCardEnum.SEEFIRST,\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const blablah =\n  \"aldkfkladfskladklsfkladklfkaldfadfkdaf\" +\n  \"adlfasdklfkldsklfakldsfkladsfkadsfladsfa\" +\n  \"dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf\";\n\nconst k = {\n  blablah:\n    \"aldkfkladfskladklsfkladklfkaldfadfkdaf\" +\n    \"adlfasdklfkldsklfakldsfkladsfkadsfladsfa\" +\n    \"dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf\",\n};\n\nsomethingThatsAReallyLongPropName =\n  this.props.cardType === AwesomizerCardEnum.SEEFIRST;\n\nconst o = {\n  somethingThatsAReallyLongPropName:\n    this.props.cardType === AwesomizerCardEnum.SEEFIRST,\n};");
}
