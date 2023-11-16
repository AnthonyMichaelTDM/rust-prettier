#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_block_comment_js_format_1_3f1b8171() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/* @flow */\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @flow */\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_block_comment_2_js_format_1_84c08625() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/*                     @noflow */\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/*                     @noflow */\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_filename_js_flow_format_1_59180d29() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("foo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo = { \"1\": bar }; // \"1\" should quoted");
}
#[test]
fn test_inline_comment_js_format_1_03e1b039() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// @flow\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flow\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_inline_comment_2_js_format_1_587077dd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("//                     @noflow\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "//                     @noflow\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_inline_comment_3_js_format_1_41a32dfb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("//  another comment\n//                     @flow\n\nfoo = {\"1\":bar} // \"1\" should quoted") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "//  another comment\n//                     @flow\n\nfoo = { \"1\": bar }; // \"1\" should quoted");
}
#[test]
fn test_like_a_pragma_js_format_1_46ac9249() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("// by fisker@flow.prettier.com :)\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// by fisker@flow.prettier.com :)\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_not_flow_js_format_1_452ab1aa() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("foo = {\"1\":bar} // \"1\" should unquoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo = { 1: bar }; // \"1\" should unquoted");
}
#[test]
fn test_not_flow_2_js_format_1_c5c1f8a1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("// @flowflow\n\nfoo = {\"1\":bar} // \"1\" should unquoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// @flowflow\n\nfoo = { 1: bar }; // \"1\" should unquoted"
    );
}
#[test]
fn test_not_flow_3_js_format_1_f6f50f7f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("a = \"  @flow \"\nfoo = {\"1\":bar} // \"1\" should unquoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a = \"  @flow \";\nfoo = { 1: bar }; // \"1\" should unquoted"
    );
}
#[test]
fn test_not_flow_4_js_format_1_ded15db7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("a = \"  // @flow \"\nfoo = {\"1\":bar} // \"1\" should unquoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "a = \"  // @flow \";\nfoo = { 1: bar }; // \"1\" should unquoted"
    );
}
#[test]
fn test_not_flow_5_js_format_1_0da8af9b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("#!/usr/bin/env @flow\nfoo = {\"1\":bar} // \"1\" should unquoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "#!/usr/bin/env @flow\nfoo = { 1: bar }; // \"1\" should unquoted"
    );
}
#[test]
fn test_pragma_js_format_1_c8b7d2cf() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("/**\n * @flow\n */\n\nfoo = {\"1\":bar} // \"1\" should quoted");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n * @flow\n */\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_pragma_2_js_format_1_77b72f10() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "/**\n *                      @noflow\n */\n\nfoo = {\"1\":bar} // \"1\" should quoted",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/**\n *                      @noflow\n */\n\nfoo = { \"1\": bar }; // \"1\" should quoted"
    );
}
#[test]
fn test_pragma_react_js_format_1_da8de405() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/**\n * Copyright (c) Facebook, Inc. and its affiliates.\n *\n * This source code is licensed under the MIT license found in the\n * LICENSE file in the root directory of this source tree.\n *\n * @flow\n */\n\n// DocBlock above is copied from https://github.com/facebook/react/blob/8da0da0937af154b775b243c9d28b6aa50db696b/packages/react-dom/index.js#L1-L8\n\nfoo = {\"1\":bar} // \"1\" should quoted") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Copyright (c) Facebook, Inc. and its affiliates.\n *\n * This source code is licensed under the MIT license found in the\n * LICENSE file in the root directory of this source tree.\n *\n * @flow\n */\n\n// DocBlock above is copied from https://github.com/facebook/react/blob/8da0da0937af154b775b243c9d28b6aa50db696b/packages/react-dom/index.js#L1-L8\n\nfoo = { \"1\": bar }; // \"1\" should quoted");
}
#[test]
fn test_shebang_inline_comment_js_format_1_52b014c4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("#!/usr/bin/env node\n//  another comment\n//                     @flow\n\nfoo = {\"1\":bar} // \"1\" should quoted") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "#!/usr/bin/env node\n//  another comment\n//                     @flow\n\nfoo = { \"1\": bar }; // \"1\" should quoted");
}
#[test]
fn test_shebang_pragma_js_format_1_ec1bef51() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("#!/usr/bin/env node\n/**\n * @format\n *                      @noflow\n */\n\nfoo = {\"1\":bar} // \"1\" should quoted") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "#!/usr/bin/env node\n/**\n * @format\n *                      @noflow\n */\n\nfoo = { \"1\": bar }; // \"1\" should quoted");
}
