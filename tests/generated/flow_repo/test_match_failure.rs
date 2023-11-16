#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_disjoint_union_js_format_1_6defbe45() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ntype Shape =\n  {type: 'rectangle', width: number, height: number} |\n  {type: 'circle', radius: number};\n\nfunction area(shape: Shape): number {\n  if (shape.type === 'square') { // TODO: this should be an error\n    return shape.width * shape.height;\n  } else if (shape.type === 'circle') {\n    return Math.PI * Math.pow(shape.radius, 2);\n  }\n  throw \"unreachable\"; // TODO: this shouldn't be needed\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\ntype Shape =\n  | { type: \"rectangle\", width: number, height: number }\n  | { type: \"circle\", radius: number };\n\nfunction area(shape: Shape): number {\n  if (shape.type === \"square\") {\n    // TODO: this should be an error\n    return shape.width * shape.height;\n  } else if (shape.type === \"circle\") {\n    return Math.PI * Math.pow(shape.radius, 2);\n  }\n  throw \"unreachable\"; // TODO: this shouldn't be needed\n}");
}
#[test]
fn test_enum_js_format_1_7f9a6bf1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype Binary = 0 | 1;\n\nfunction stringifyBinary(binary: Binary): string {\n  if (binary === 0) {\n    return 'zero';\n  } else if (binary === 2) { // oops\n    return 'one';\n  }\n  throw \"unreachable\"; // TODO: this shouldn't be needed\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\ntype Binary = 0 | 1;\n\nfunction stringifyBinary(binary: Binary): string {\n  if (binary === 0) {\n    return \"zero\";\n  } else if (binary === 2) {\n    // oops\n    return \"one\";\n  }\n  throw \"unreachable\"; // TODO: this shouldn't be needed\n}");
}
