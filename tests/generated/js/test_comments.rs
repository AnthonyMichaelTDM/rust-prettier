#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_arrow_js_semifalse_format_1_97fc8604() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const fn = (/*event, data*/) => doSomething();\n\nconst fn2 = (/*event, data*/) => doSomething(anything);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const fn = (/*event, data*/) => doSomething()\n\nconst fn2 = (/*event, data*/) => doSomething(anything)");
}
#[test]
fn test_arrow_js_format_1_97fc8604() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const fn = (/*event, data*/) => doSomething();\n\nconst fn2 = (/*event, data*/) => doSomething(anything);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const fn = (/*event, data*/) => doSomething();\n\nconst fn2 = (/*event, data*/) => doSomething(anything);");
}
#[test]
fn test_assignment_pattern_js_semifalse_format_1_51c94a24() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const { a /* comment */ = 1 } = b;\n\nconst { c = 1 /* comment */ } = d;\n\nlet {d //comment\n= b} = c") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const { a /* comment */ = 1 } = b\n\nconst { c = 1 /* comment */ } = d\n\nlet {\n  d = b, //comment\n} = c");
}
#[test]
fn test_assignment_pattern_js_format_1_51c94a24() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const { a /* comment */ = 1 } = b;\n\nconst { c = 1 /* comment */ } = d;\n\nlet {d //comment\n= b} = c") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const { a /* comment */ = 1 } = b;\n\nconst { c = 1 /* comment */ } = d;\n\nlet {\n  d = b, //comment\n} = c;");
}
#[test]
fn test_before_comma_js_semifalse_format_1_8f422647() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "const foo = {\n  a: 'a' /* comment for this line */,\n\n  /* Section B */\n  b: 'b',\n};",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = {\n  a: \"a\" /* comment for this line */,\n\n  /* Section B */\n  b: \"b\",\n}");
}
#[test]
fn test_before_comma_js_format_1_8f422647() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "const foo = {\n  a: 'a' /* comment for this line */,\n\n  /* Section B */\n  b: 'b',\n};",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo = {\n  a: \"a\" /* comment for this line */,\n\n  /* Section B */\n  b: \"b\",\n};");
}
#[test]
fn test_binary_expressions_js_semifalse_format_1_6b8fdf61() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function addition() {\n\t0\n\t// Comment\n\t+ x\n}\n\nfunction multiplication() {\n\t0\n\t// Comment\n\t* x\n}\n\nfunction division() {\n\t0\n\t// Comment\n\t/ x\n}\n\nfunction substraction() {\n\t0\n\t// Comment\n\t- x\n}\n\nfunction remainder() {\n\t0\n\t// Comment\n\t% x\n}\n\nfunction exponentiation() {\n\t0\n\t// Comment\n\t** x\n}\n\nfunction leftShift() {\n\t0\n\t// Comment\n\t<< x\n}\n\nfunction rightShift() {\n\t0\n\t// Comment\n\t>> x\n}\n\nfunction unsignedRightShift() {\n\t0\n\t// Comment\n\t>>> x\n}\n\nfunction bitwiseAnd() {\n\t0\n\t// Comment\n\t& x\n}\n\nfunction bitwiseOr() {\n\t0\n\t// Comment\n\t| x\n}\n\nfunction bitwiseXor() {\n\t0\n\t// Comment\n\t^ x\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function addition() {\n  0 +\n    // Comment\n    x\n}\n\nfunction multiplication() {\n  0 *\n    // Comment\n    x\n}\n\nfunction division() {\n  0 /\n    // Comment\n    x\n}\n\nfunction substraction() {\n  0 -\n    // Comment\n    x\n}\n\nfunction remainder() {\n  0 %\n    // Comment\n    x\n}\n\nfunction exponentiation() {\n  0 **\n    // Comment\n    x\n}\n\nfunction leftShift() {\n  0 <<\n    // Comment\n    x\n}\n\nfunction rightShift() {\n  0 >>\n    // Comment\n    x\n}\n\nfunction unsignedRightShift() {\n  0 >>>\n    // Comment\n    x\n}\n\nfunction bitwiseAnd() {\n  0 &\n    // Comment\n    x\n}\n\nfunction bitwiseOr() {\n  0 |\n    // Comment\n    x\n}\n\nfunction bitwiseXor() {\n  0 ^\n    // Comment\n    x\n}");
}
#[test]
fn test_binary_expressions_js_format_1_6b8fdf61() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function addition() {\n\t0\n\t// Comment\n\t+ x\n}\n\nfunction multiplication() {\n\t0\n\t// Comment\n\t* x\n}\n\nfunction division() {\n\t0\n\t// Comment\n\t/ x\n}\n\nfunction substraction() {\n\t0\n\t// Comment\n\t- x\n}\n\nfunction remainder() {\n\t0\n\t// Comment\n\t% x\n}\n\nfunction exponentiation() {\n\t0\n\t// Comment\n\t** x\n}\n\nfunction leftShift() {\n\t0\n\t// Comment\n\t<< x\n}\n\nfunction rightShift() {\n\t0\n\t// Comment\n\t>> x\n}\n\nfunction unsignedRightShift() {\n\t0\n\t// Comment\n\t>>> x\n}\n\nfunction bitwiseAnd() {\n\t0\n\t// Comment\n\t& x\n}\n\nfunction bitwiseOr() {\n\t0\n\t// Comment\n\t| x\n}\n\nfunction bitwiseXor() {\n\t0\n\t// Comment\n\t^ x\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function addition() {\n  0 +\n    // Comment\n    x;\n}\n\nfunction multiplication() {\n  0 *\n    // Comment\n    x;\n}\n\nfunction division() {\n  0 /\n    // Comment\n    x;\n}\n\nfunction substraction() {\n  0 -\n    // Comment\n    x;\n}\n\nfunction remainder() {\n  0 %\n    // Comment\n    x;\n}\n\nfunction exponentiation() {\n  0 **\n    // Comment\n    x;\n}\n\nfunction leftShift() {\n  0 <<\n    // Comment\n    x;\n}\n\nfunction rightShift() {\n  0 >>\n    // Comment\n    x;\n}\n\nfunction unsignedRightShift() {\n  0 >>>\n    // Comment\n    x;\n}\n\nfunction bitwiseAnd() {\n  0 &\n    // Comment\n    x;\n}\n\nfunction bitwiseOr() {\n  0 |\n    // Comment\n    x;\n}\n\nfunction bitwiseXor() {\n  0 ^\n    // Comment\n    x;\n}");
}
#[test]
fn test_binary_expressions_block_comments_js_semifalse_format_1_5faa2614() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = b || /** Comment */\nc;\n\na = b /** Comment */ ||\nc;\n\na = b || /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\nc;\n\na = b || /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b && /** Comment */\nc;\n\na = b /** Comment */ &&\nc;\n\na = b && /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\nc;\n\na = b && /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b + /** Comment */\nc;\n\na = b /** Comment */ +\nc;\n\na = b + /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\nc;\n\na = b + /** TODO this is a very very very very long comment that makes it go > 80 columns */ c") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = b /** Comment */ || c\n\na = b /** Comment */ || c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\n  c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\n  c\n\na =\n  b ||\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c\n\na = b /** Comment */ && c\n\na = b /** Comment */ && c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\n  c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\n  c\n\na =\n  b &&\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c\n\na = b /** Comment */ + c\n\na = b /** Comment */ + c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\n  c\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\n  c\n\na =\n  b +\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c");
}
#[test]
fn test_binary_expressions_block_comments_js_format_1_5faa2614() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = b || /** Comment */\nc;\n\na = b /** Comment */ ||\nc;\n\na = b || /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\nc;\n\na = b || /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b && /** Comment */\nc;\n\na = b /** Comment */ &&\nc;\n\na = b && /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\nc;\n\na = b && /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b + /** Comment */\nc;\n\na = b /** Comment */ +\nc;\n\na = b + /** TODO this is a very very very very long comment that makes it go > 80 columns */\nc;\n\na = b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\nc;\n\na = b + /** TODO this is a very very very very long comment that makes it go > 80 columns */ c") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a = b /** Comment */ || c;\n\na = b /** Comment */ || c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\n  c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||\n  c;\n\na =\n  b ||\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b /** Comment */ && c;\n\na = b /** Comment */ && c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\n  c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&\n  c;\n\na =\n  b &&\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;\n\na = b /** Comment */ + c;\n\na = b /** Comment */ + c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\n  c;\n\na =\n  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +\n  c;\n\na =\n  b +\n  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;");
}
#[test]
fn test_binary_expressions_parens_js_semifalse_format_1_f44ba045() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("Math.min(\n  (\n    /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the\n     * deployment of v0.38.0. To see the error, remove this comment and\n     * run flow */\n    document.body.scrollHeight -\n    (window.scrollY + window.innerHeight)\n  ) - devsite_footer_height,\n  0,\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Math.min(\n  /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the\n   * deployment of v0.38.0. To see the error, remove this comment and\n   * run flow */\n  document.body.scrollHeight -\n    (window.scrollY + window.innerHeight) -\n    devsite_footer_height,\n  0,\n)");
}
#[test]
fn test_binary_expressions_parens_js_format_1_f44ba045() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("Math.min(\n  (\n    /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the\n     * deployment of v0.38.0. To see the error, remove this comment and\n     * run flow */\n    document.body.scrollHeight -\n    (window.scrollY + window.innerHeight)\n  ) - devsite_footer_height,\n  0,\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Math.min(\n  /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the\n   * deployment of v0.38.0. To see the error, remove this comment and\n   * run flow */\n  document.body.scrollHeight -\n    (window.scrollY + window.innerHeight) -\n    devsite_footer_height,\n  0,\n);");
}
#[test]
fn test_binary_expressions_single_comments_js_semifalse_format_1_2ac75752() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = b || // Comment\nc;\n\na = b || // TODO this is a very very very very long comment that makes it go > 80 columns\nc;\n\na = b && // Comment\nc;\n\na = b && // TODO this is a very very very very long comment that makes it go > 80 columns\nc;\n\na = b + // Comment\nc;\n\na = b + // TODO this is a very very very very long comment that makes it go > 80 columns\nc") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a =\n  b || // Comment\n  c\n\na =\n  b || // TODO this is a very very very very long comment that makes it go > 80 columns\n  c\n\na =\n  b && // Comment\n  c\n\na =\n  b && // TODO this is a very very very very long comment that makes it go > 80 columns\n  c\n\na =\n  b + // Comment\n  c\n\na =\n  b + // TODO this is a very very very very long comment that makes it go > 80 columns\n  c");
}
#[test]
fn test_binary_expressions_single_comments_js_format_1_2ac75752() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a = b || // Comment\nc;\n\na = b || // TODO this is a very very very very long comment that makes it go > 80 columns\nc;\n\na = b && // Comment\nc;\n\na = b && // TODO this is a very very very very long comment that makes it go > 80 columns\nc;\n\na = b + // Comment\nc;\n\na = b + // TODO this is a very very very very long comment that makes it go > 80 columns\nc") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a =\n  b || // Comment\n  c;\n\na =\n  b || // TODO this is a very very very very long comment that makes it go > 80 columns\n  c;\n\na =\n  b && // Comment\n  c;\n\na =\n  b && // TODO this is a very very very very long comment that makes it go > 80 columns\n  c;\n\na =\n  b + // Comment\n  c;\n\na =\n  b + // TODO this is a very very very very long comment that makes it go > 80 columns\n  c;");
}
#[test]
fn test_blank_js_semifalse_format_1_fb75bf4f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// This file only\n// has comments. This comment\n// should still exist\n//\n// when printed.\n\n/**\n * @typedef {DataDrivenMapping|ConstantMapping} Mapping\n */\n/**\n * @typedef {Object.<String, Mapping>} ConfigurationMapping\n */\n\n/**\n * @typedef {Function} D3Scale - a D3 scale\n * @property {Function} ticks\n * @property {Function} tickFormat\n */\n// comment\n\n// comment") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// This file only\n// has comments. This comment\n// should still exist\n//\n// when printed.\n\n/**\n * @typedef {DataDrivenMapping|ConstantMapping} Mapping\n */\n/**\n * @typedef {Object.<String, Mapping>} ConfigurationMapping\n */\n\n/**\n * @typedef {Function} D3Scale - a D3 scale\n * @property {Function} ticks\n * @property {Function} tickFormat\n */\n// comment\n\n// comment");
}
#[test]
fn test_blank_js_format_1_fb75bf4f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// This file only\n// has comments. This comment\n// should still exist\n//\n// when printed.\n\n/**\n * @typedef {DataDrivenMapping|ConstantMapping} Mapping\n */\n/**\n * @typedef {Object.<String, Mapping>} ConfigurationMapping\n */\n\n/**\n * @typedef {Function} D3Scale - a D3 scale\n * @property {Function} ticks\n * @property {Function} tickFormat\n */\n// comment\n\n// comment") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// This file only\n// has comments. This comment\n// should still exist\n//\n// when printed.\n\n/**\n * @typedef {DataDrivenMapping|ConstantMapping} Mapping\n */\n/**\n * @typedef {Object.<String, Mapping>} ConfigurationMapping\n */\n\n/**\n * @typedef {Function} D3Scale - a D3 scale\n * @property {Function} ticks\n * @property {Function} tickFormat\n */\n// comment\n\n// comment");
}
#[test]
fn test_break_continue_statements_js_semifalse_format_1_b39831cd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("for (;;) {\n  break /* comment */;\n  continue /* comment */;\n}\n\nloop: for (;;) {\n  break /* comment */ loop;\n  break loop /* comment */;\n  continue /* comment */ loop;\n  continue loop /* comment */;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "for (;;) {\n  break /* comment */\n  continue /* comment */\n}\n\nloop: for (;;) {\n  break /* comment */ loop\n  break loop /* comment */\n  continue /* comment */ loop\n  continue loop /* comment */\n}");
}
#[test]
fn test_break_continue_statements_js_format_1_b39831cd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("for (;;) {\n  break /* comment */;\n  continue /* comment */;\n}\n\nloop: for (;;) {\n  break /* comment */ loop;\n  break loop /* comment */;\n  continue /* comment */ loop;\n  continue loop /* comment */;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "for (;;) {\n  break; /* comment */\n  continue; /* comment */\n}\n\nloop: for (;;) {\n  break /* comment */ loop;\n  break loop /* comment */;\n  continue /* comment */ loop;\n  continue loop /* comment */;\n}");
}
#[test]
fn test_call_comment_js_semifalse_format_1_f52084a7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("render( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);\n\nReact.render( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);\n\nrender?.( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "render(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n)\n\nReact.render(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n)\n\nrender?.(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n)");
}
#[test]
fn test_call_comment_js_format_1_f52084a7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("render( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);\n\nReact.render( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);\n\nrender?.( // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "render(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n);\n\nReact.render(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n);\n\nrender?.(\n  // Warm any cache\n  <ChildUpdates renderAnchor={true} anchorClassOn={true} />,\n  container,\n);");
}
#[test]
fn test_class_js_semifalse_format_1_efbe208f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #8718\nclass C {\n    ma() {} /* D */ /* E */\n    mb() {}\n}\n\nclass D {\n    ma() {} /* D */ /* E */ /* F */\n    mb() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #8718\nclass C {\n  ma() {} /* D */ /* E */\n  mb() {}\n}\n\nclass D {\n  ma() {} /* D */ /* E */ /* F */\n  mb() {}\n}");
}
#[test]
fn test_class_js_format_1_efbe208f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #8718\nclass C {\n    ma() {} /* D */ /* E */\n    mb() {}\n}\n\nclass D {\n    ma() {} /* D */ /* E */ /* F */\n    mb() {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #8718\nclass C {\n  ma() {} /* D */ /* E */\n  mb() {}\n}\n\nclass D {\n  ma() {} /* D */ /* E */ /* F */\n  mb() {}\n}");
}
#[test]
fn test_dangling_js_semifalse_format_1_76b8c2a8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var a = {/* dangling */};\nvar b = {\n  // dangling\n};\nvar b = [/* dangling */];\nfunction d() {\n  /* dangling */\n}\nnew Thing(/* dangling */);\nThing(/* dangling */);\nexport /* dangling */{};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a = {\n  /* dangling */\n}\nvar b = {\n  // dangling\n}\nvar b = [\n  /* dangling */\n]\nfunction d() {\n  /* dangling */\n}\nnew Thing(/* dangling */)\nThing(/* dangling */)\nexport /* dangling */ {}");
}
#[test]
fn test_dangling_js_format_1_76b8c2a8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("var a = {/* dangling */};\nvar b = {\n  // dangling\n};\nvar b = [/* dangling */];\nfunction d() {\n  /* dangling */\n}\nnew Thing(/* dangling */);\nThing(/* dangling */);\nexport /* dangling */{};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a = {\n  /* dangling */\n};\nvar b = {\n  // dangling\n};\nvar b = [\n  /* dangling */\n];\nfunction d() {\n  /* dangling */\n}\nnew Thing(/* dangling */);\nThing(/* dangling */);\nexport /* dangling */ {};");
}
#[test]
fn test_dangling_array_js_semifalse_format_1_c46fff14() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("expect(() => {}).toTriggerReadyStateChanges([\n  // Nothing.\n]);\n\n[1 /*\u{2003}first comment */, 2 /* second comment */, 3];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "expect(() => {}).toTriggerReadyStateChanges([\n  // Nothing.\n])\n\n;[1 /*\u{2003}first comment */, 2 /* second comment */, 3]");
}
#[test]
fn test_dangling_array_js_format_1_c46fff14() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("expect(() => {}).toTriggerReadyStateChanges([\n  // Nothing.\n]);\n\n[1 /*\u{2003}first comment */, 2 /* second comment */, 3];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "expect(() => {}).toTriggerReadyStateChanges([\n  // Nothing.\n]);\n\n[1 /*\u{2003}first comment */, 2 /* second comment */, 3];");
}
#[test]
fn test_dangling_for_js_semifalse_format_1_0a66ac32() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("for // comment\n(;;);\n\nfor /* comment */(;;);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment\nfor (;;);\n\n/* comment */\nfor (;;);"
    );
}
#[test]
fn test_dangling_for_js_format_1_0a66ac32() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("for // comment\n(;;);\n\nfor /* comment */(;;);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// comment\nfor (;;);\n\n/* comment */\nfor (;;);"
    );
}
#[test]
fn test_dynamic_imports_js_semifalse_format_1_929bc3b7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("import(/* Hello */ 'something')\n\nimport('something' /* Hello */)\n\nimport(/* Hello */ 'something' /* Hello */)\n\nimport('something' /* Hello */ + 'else')\n\nimport(\n  /* Hello */\n  'something'\n  /* Hello */\n)\n\nwrap(\n  import(/* Hello */\n    'something'\n  )\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import(/* Hello */ \"something\")\n\nimport(\"something\" /* Hello */)\n\nimport(/* Hello */ \"something\" /* Hello */)\n\nimport(\"something\" /* Hello */ + \"else\")\n\nimport(\n  /* Hello */\n  \"something\"\n  /* Hello */\n)\n\nwrap(import(/* Hello */ \"something\"))");
}
#[test]
fn test_dynamic_imports_js_format_1_929bc3b7() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("import(/* Hello */ 'something')\n\nimport('something' /* Hello */)\n\nimport(/* Hello */ 'something' /* Hello */)\n\nimport('something' /* Hello */ + 'else')\n\nimport(\n  /* Hello */\n  'something'\n  /* Hello */\n)\n\nwrap(\n  import(/* Hello */\n    'something'\n  )\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import(/* Hello */ \"something\");\n\nimport(\"something\" /* Hello */);\n\nimport(/* Hello */ \"something\" /* Hello */);\n\nimport(\"something\" /* Hello */ + \"else\");\n\nimport(\n  /* Hello */\n  \"something\"\n  /* Hello */\n);\n\nwrap(import(/* Hello */ \"something\"));");
}
#[test]
fn test_emoji_js_semifalse_format_1_374d3f48() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/* #2091 */\n\nconst test = '💖'\n// This comment\n// should not get collapsed");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* #2091 */\n\nconst test = \"💖\"\n// This comment\n// should not get collapsed"
    );
}
#[test]
fn test_emoji_js_format_1_374d3f48() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer
        .format("/* #2091 */\n\nconst test = '💖'\n// This comment\n// should not get collapsed");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* #2091 */\n\nconst test = \"💖\";\n// This comment\n// should not get collapsed"
    );
}
#[test]
fn test_empty_statements_js_semifalse_format_1_fc95c709() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a; /* a */ // b\n; /* c */\n\nfoo; // first\n;// second\n;// third\n\nfunction x() {\n} // first\n; // second\n\na = (\n  b // 1\n  + // 2\n  c // 3\n  + // 4\n  d // 5\n  + /* 6 */\n  e // 7\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a /* a */ // b\n/* c */\nfoo // first\n// second\n// third\nfunction x() {} // first\n// second\na =\n  b + // 1\n  // 2\n  c + // 3\n  // 4\n  d + // 5\n  /* 6 */\n  e // 7");
}
#[test]
fn test_empty_statements_js_format_1_fc95c709() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("a; /* a */ // b\n; /* c */\n\nfoo; // first\n;// second\n;// third\n\nfunction x() {\n} // first\n; // second\n\na = (\n  b // 1\n  + // 2\n  c // 3\n  + // 4\n  d // 5\n  + /* 6 */\n  e // 7\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a; /* a */ // b\n/* c */\nfoo; // first\n// second\n// third\nfunction x() {} // first\n// second\na =\n  b + // 1\n  // 2\n  c + // 3\n  // 4\n  d + // 5\n  /* 6 */\n  e; // 7");
}
#[test]
fn test_export_js_semifalse_format_1_fe3d3e31() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export //comment\n{}\n\nexport /* comment */ {};\n\nconst foo = ''\nexport {\n  foo // comment\n}\n\nconst bar = ''\nexport {\n  // comment\n  bar\n}\n\nconst fooo = ''\nconst barr = ''\nexport {\n  fooo, // comment\n  barr, // comment\n}\n\nconst foooo = ''\nconst barrr = ''\nexport {\n  foooo,\n\n  barrr as  // comment\n\t\t baz,\n} from 'foo'\n\nconst fooooo = ''\nconst barrrr = ''\nexport {\n  fooooo,\n\n  barrrr as  // comment\n\t\t bazz,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export //comment\n {}\n\nexport /* comment */ {}\n\nconst foo = \"\"\nexport {\n  foo, // comment\n}\n\nconst bar = \"\"\nexport {\n  // comment\n  bar,\n}\n\nconst fooo = \"\"\nconst barr = \"\"\nexport {\n  fooo, // comment\n  barr, // comment\n}\n\nconst foooo = \"\"\nconst barrr = \"\"\nexport {\n  foooo,\n  // comment\n  barrr as baz,\n} from \"foo\"\n\nconst fooooo = \"\"\nconst barrrr = \"\"\nexport {\n  fooooo,\n  // comment\n  barrrr as bazz,\n}");
}
#[test]
fn test_export_js_format_1_fe3d3e31() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export //comment\n{}\n\nexport /* comment */ {};\n\nconst foo = ''\nexport {\n  foo // comment\n}\n\nconst bar = ''\nexport {\n  // comment\n  bar\n}\n\nconst fooo = ''\nconst barr = ''\nexport {\n  fooo, // comment\n  barr, // comment\n}\n\nconst foooo = ''\nconst barrr = ''\nexport {\n  foooo,\n\n  barrr as  // comment\n\t\t baz,\n} from 'foo'\n\nconst fooooo = ''\nconst barrrr = ''\nexport {\n  fooooo,\n\n  barrrr as  // comment\n\t\t bazz,\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export //comment\n {};\n\nexport /* comment */ {};\n\nconst foo = \"\";\nexport {\n  foo, // comment\n};\n\nconst bar = \"\";\nexport {\n  // comment\n  bar,\n};\n\nconst fooo = \"\";\nconst barr = \"\";\nexport {\n  fooo, // comment\n  barr, // comment\n};\n\nconst foooo = \"\";\nconst barrr = \"\";\nexport {\n  foooo,\n  // comment\n  barrr as baz,\n} from \"foo\";\n\nconst fooooo = \"\";\nconst barrrr = \"\";\nexport {\n  fooooo,\n  // comment\n  barrrr as bazz,\n};");
}
#[test]
fn test_export_and_import_js_semifalse_format_1_8b709f15() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// These are tests to compare comment formats in \\`export\\` and \\`import\\`.\n\nexport {\n  foo,\n\n  bar as  // comment\n\t\t baz,\n} from 'foo'\n\nconst fooo = \"\"\nconst barr = \"\"\n\nexport {\n  fooo,\n\n  barr as  // comment\n\t\t bazz,\n}\n\nimport {\n  foo,\n\n  bar as  // comment\n\t\t baz,\n} from 'foo'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// These are tests to compare comment formats in \\`export\\` and \\`import\\`.\n\nexport {\n  foo,\n  // comment\n  bar as baz,\n} from \"foo\"\n\nconst fooo = \"\"\nconst barr = \"\"\n\nexport {\n  fooo,\n  // comment\n  barr as bazz,\n}\n\nimport {\n  foo,\n  // comment\n  bar as baz,\n} from \"foo\"");
}
#[test]
fn test_export_and_import_js_format_1_8b709f15() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// These are tests to compare comment formats in \\`export\\` and \\`import\\`.\n\nexport {\n  foo,\n\n  bar as  // comment\n\t\t baz,\n} from 'foo'\n\nconst fooo = \"\"\nconst barr = \"\"\n\nexport {\n  fooo,\n\n  barr as  // comment\n\t\t bazz,\n}\n\nimport {\n  foo,\n\n  bar as  // comment\n\t\t baz,\n} from 'foo'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// These are tests to compare comment formats in \\`export\\` and \\`import\\`.\n\nexport {\n  foo,\n  // comment\n  bar as baz,\n} from \"foo\";\n\nconst fooo = \"\";\nconst barr = \"\";\n\nexport {\n  fooo,\n  // comment\n  barr as bazz,\n};\n\nimport {\n  foo,\n  // comment\n  bar as baz,\n} from \"foo\";");
}
#[test]
fn test_first_line_js_semifalse_format_1_e73569c1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a // comment\nb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a // comment\nb");
}
#[test]
fn test_first_line_js_format_1_e73569c1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("a // comment\nb");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a; // comment\nb;");
}
#[test]
fn test_function_declaration_js_semifalse_format_1_670d17f4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function a(/* comment */) {} // comment\nfunction b() {} // comment\nfunction c(/* comment */ argA, argB, argC) {} // comment\ncall((/*object*/ row) => {});\nKEYPAD_NUMBERS.map(num => ( // Buttons 0-9\n  <div />\n));\n\nfunction f1 /* f */() {}\nfunction f2 (/* args */) {}\nfunction f3 () /* returns */ {}\nfunction f4 /* f */(/* args */) /* returns */ {}\n\nfunction f5 /* f */(/* a */ a) {}\nfunction f6 /* f */(a /* a */) {}\nfunction f7 /* f */(/* a */ a) /* returns */ {}\n\nconst obj = {\n  f1 /* f */() {},\n  f2 (/* args */) {},\n  f3 () /* returns */ {},\n  f4 /* f */(/* args */) /* returns */ {},\n};\n\n(function f /* f */() {})();\n(function f (/* args */) {})();\n(function f () /* returns */ {})();\n(function f /* f */(/* args */) /* returns */ {})();\n\nclass C1 {\n  f/* f */() {}\n}\nclass C2 {\n  f(/* args */) {}\n}\nclass C3 {\n  f() /* returns */ {}\n}\nclass C4 {\n  f/* f */(/* args */) /* returns */ {}\n}\n\nfunction foo1() \n// this is a function\n{\n  return 42\n}\n\nfunction foo2() // this is a function\n{\n  return 42\n}\n\nfunction foo3() { // this is a function\n  return 42\n}\n\nfunction foo4() {\n  // this is a function\n  return 42;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function a(/* comment */) {} // comment\nfunction b() {} // comment\nfunction c(/* comment */ argA, argB, argC) {} // comment\ncall((/*object*/ row) => {})\nKEYPAD_NUMBERS.map(\n  (\n    num, // Buttons 0-9\n  ) => <div />,\n)\n\nfunction f1 /* f */() {}\nfunction f2(/* args */) {}\nfunction f3() /* returns */ {}\nfunction f4 /* f */(/* args */) /* returns */ {}\n\nfunction f5 /* f */(/* a */ a) {}\nfunction f6 /* f */(a /* a */) {}\nfunction f7 /* f */(/* a */ a) /* returns */ {}\n\nconst obj = {\n  f1 /* f */() {},\n  f2(/* args */) {},\n  f3() /* returns */ {},\n  f4 /* f */(/* args */) /* returns */ {},\n}\n\n;(function f /* f */() {})()\n;(function f(/* args */) {})()\n;(function f() /* returns */ {})()\n;(function f /* f */(/* args */) /* returns */ {})()\n\nclass C1 {\n  f /* f */() {}\n}\nclass C2 {\n  f(/* args */) {}\n}\nclass C3 {\n  f() /* returns */ {}\n}\nclass C4 {\n  f /* f */(/* args */) /* returns */ {}\n}\n\nfunction foo1() {\n  // this is a function\n  return 42\n}\n\nfunction foo2() {\n  // this is a function\n  return 42\n}\n\nfunction foo3() {\n  // this is a function\n  return 42\n}\n\nfunction foo4() {\n  // this is a function\n  return 42\n}");
}
#[test]
fn test_function_declaration_js_format_1_670d17f4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function a(/* comment */) {} // comment\nfunction b() {} // comment\nfunction c(/* comment */ argA, argB, argC) {} // comment\ncall((/*object*/ row) => {});\nKEYPAD_NUMBERS.map(num => ( // Buttons 0-9\n  <div />\n));\n\nfunction f1 /* f */() {}\nfunction f2 (/* args */) {}\nfunction f3 () /* returns */ {}\nfunction f4 /* f */(/* args */) /* returns */ {}\n\nfunction f5 /* f */(/* a */ a) {}\nfunction f6 /* f */(a /* a */) {}\nfunction f7 /* f */(/* a */ a) /* returns */ {}\n\nconst obj = {\n  f1 /* f */() {},\n  f2 (/* args */) {},\n  f3 () /* returns */ {},\n  f4 /* f */(/* args */) /* returns */ {},\n};\n\n(function f /* f */() {})();\n(function f (/* args */) {})();\n(function f () /* returns */ {})();\n(function f /* f */(/* args */) /* returns */ {})();\n\nclass C1 {\n  f/* f */() {}\n}\nclass C2 {\n  f(/* args */) {}\n}\nclass C3 {\n  f() /* returns */ {}\n}\nclass C4 {\n  f/* f */(/* args */) /* returns */ {}\n}\n\nfunction foo1() \n// this is a function\n{\n  return 42\n}\n\nfunction foo2() // this is a function\n{\n  return 42\n}\n\nfunction foo3() { // this is a function\n  return 42\n}\n\nfunction foo4() {\n  // this is a function\n  return 42;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function a(/* comment */) {} // comment\nfunction b() {} // comment\nfunction c(/* comment */ argA, argB, argC) {} // comment\ncall((/*object*/ row) => {});\nKEYPAD_NUMBERS.map(\n  (\n    num, // Buttons 0-9\n  ) => <div />,\n);\n\nfunction f1 /* f */() {}\nfunction f2(/* args */) {}\nfunction f3() /* returns */ {}\nfunction f4 /* f */(/* args */) /* returns */ {}\n\nfunction f5 /* f */(/* a */ a) {}\nfunction f6 /* f */(a /* a */) {}\nfunction f7 /* f */(/* a */ a) /* returns */ {}\n\nconst obj = {\n  f1 /* f */() {},\n  f2(/* args */) {},\n  f3() /* returns */ {},\n  f4 /* f */(/* args */) /* returns */ {},\n};\n\n(function f /* f */() {})();\n(function f(/* args */) {})();\n(function f() /* returns */ {})();\n(function f /* f */(/* args */) /* returns */ {})();\n\nclass C1 {\n  f /* f */() {}\n}\nclass C2 {\n  f(/* args */) {}\n}\nclass C3 {\n  f() /* returns */ {}\n}\nclass C4 {\n  f /* f */(/* args */) /* returns */ {}\n}\n\nfunction foo1() {\n  // this is a function\n  return 42;\n}\n\nfunction foo2() {\n  // this is a function\n  return 42;\n}\n\nfunction foo3() {\n  // this is a function\n  return 42;\n}\n\nfunction foo4() {\n  // this is a function\n  return 42;\n}");
}
#[test]
fn test_if_js_semifalse_format_1_5e51aee2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("if (1)\n// comment\n{\n  false\n}\n// comment\nelse if (2)\n  true\n// multi\n// ple\n// lines\nelse if (3)\n  // existing comment\n  true\n// okay?\nelse if (4) {\n  // empty with existing comment\n}\n// comment\nelse {\n}\n\nif (5) // comment\ntrue\n\nif (6) // comment\n{true}\nelse if (7) // comment\ntrue\nelse // comment\n{true}\n\nif (8) // comment\n// comment\n{true}\nelse if (9) // comment\n// comment\ntrue\nelse // comment\n// comment\n{true}\n\nif (10) /* comment */ // comment\n{true}\nelse if (11) /* comment */\ntrue\nelse if (12) // comment /* comment */ // comment\ntrue\nelse if (13) /* comment */ /* comment */ // comment\ntrue\nelse /* comment */\n{true}\n\nif (14) // comment\n/* comment */\n// comment\n{true}\nelse if (15) // comment\n/* comment */\n/* comment */ // comment\ntrue") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (1) {\n  // comment\n  false\n}\n// comment\nelse if (2) true\n// multi\n// ple\n// lines\nelse if (3)\n  // existing comment\n  true\n// okay?\nelse if (4) {\n  // empty with existing comment\n}\n// comment\nelse {\n}\n\nif (5)\n  // comment\n  true\n\nif (6) {\n  // comment\n  true\n} else if (7)\n  // comment\n  true\n// comment\nelse {\n  true\n}\n\nif (8) {\n  // comment\n  // comment\n  true\n} else if (9)\n  // comment\n  // comment\n  true\n// comment\n// comment\nelse {\n  true\n}\n\nif (10) {\n  /* comment */ // comment\n  true\n} else if (11) /* comment */ true\nelse if (12)\n  // comment /* comment */ // comment\n  true\nelse if (13)\n  /* comment */ /* comment */ // comment\n  true\n/* comment */ else {\n  true\n}\n\nif (14) {\n  // comment\n  /* comment */\n  // comment\n  true\n} else if (15)\n  // comment\n  /* comment */\n  /* comment */ // comment\n  true");
}
#[test]
fn test_if_js_format_1_5e51aee2() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("if (1)\n// comment\n{\n  false\n}\n// comment\nelse if (2)\n  true\n// multi\n// ple\n// lines\nelse if (3)\n  // existing comment\n  true\n// okay?\nelse if (4) {\n  // empty with existing comment\n}\n// comment\nelse {\n}\n\nif (5) // comment\ntrue\n\nif (6) // comment\n{true}\nelse if (7) // comment\ntrue\nelse // comment\n{true}\n\nif (8) // comment\n// comment\n{true}\nelse if (9) // comment\n// comment\ntrue\nelse // comment\n// comment\n{true}\n\nif (10) /* comment */ // comment\n{true}\nelse if (11) /* comment */\ntrue\nelse if (12) // comment /* comment */ // comment\ntrue\nelse if (13) /* comment */ /* comment */ // comment\ntrue\nelse /* comment */\n{true}\n\nif (14) // comment\n/* comment */\n// comment\n{true}\nelse if (15) // comment\n/* comment */\n/* comment */ // comment\ntrue") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (1) {\n  // comment\n  false;\n}\n// comment\nelse if (2) true;\n// multi\n// ple\n// lines\nelse if (3)\n  // existing comment\n  true;\n// okay?\nelse if (4) {\n  // empty with existing comment\n}\n// comment\nelse {\n}\n\nif (5)\n  // comment\n  true;\n\nif (6) {\n  // comment\n  true;\n} else if (7)\n  // comment\n  true;\n// comment\nelse {\n  true;\n}\n\nif (8) {\n  // comment\n  // comment\n  true;\n} else if (9)\n  // comment\n  // comment\n  true;\n// comment\n// comment\nelse {\n  true;\n}\n\nif (10) {\n  /* comment */ // comment\n  true;\n} else if (11) /* comment */ true;\nelse if (12)\n  // comment /* comment */ // comment\n  true;\nelse if (13)\n  /* comment */ /* comment */ // comment\n  true;\n/* comment */ else {\n  true;\n}\n\nif (14) {\n  // comment\n  /* comment */\n  // comment\n  true;\n} else if (15)\n  // comment\n  /* comment */\n  /* comment */ // comment\n  true;");
}
#[test]
fn test_issue_3532_js_semifalse_format_1_16f90a02() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("import React from 'react';\n\n/*\nimport styled from 'react-emotion';\n\nconst AspectRatioBox = styled.div\\`\n  &::before {\n    content: '';\n    width: 1px;\n    margin-left: -1px;\n    float: left;\n    height: 0;\n    padding-top: \\${props => 100 / props.aspectRatio}%;\n  }\n\n  &::after {\n    /* To clear float *//*\n    content: '';\n    display: table;\n    clear: both;\n  }\n\\`;\n*/\n\nconst AspectRatioBox = ({\n  aspectRatio,\n  children,\n  ...props\n}) => (\n  <div\n    className={\\`height: 0;\n  overflow: hidden;\n  padding-top: \\${props => 100 / props.aspectRatio}%;\n  background: white;\n  position: relative;\\`}\n  >\n    <div>{children}</div>\n  </div>\n);\n\nexport default AspectRatioBox;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\"\n\n/*\nimport styled from 'react-emotion';\n\nconst AspectRatioBox = styled.div\\`\n  &::before {\n    content: '';\n    width: 1px;\n    margin-left: -1px;\n    float: left;\n    height: 0;\n    padding-top: \\${props => 100 / props.aspectRatio}%;\n  }\n\n  &::after {\n    /* To clear float */ /*\n    content: '';\n    display: table;\n    clear: both;\n  }\n\\`;\n*/\n\nconst AspectRatioBox = ({ aspectRatio, children, ...props }) => (\n  <div\n    className={\\`height: 0;\n  overflow: hidden;\n  padding-top: \\${(props) => 100 / props.aspectRatio}%;\n  background: white;\n  position: relative;\\`}\n  >\n    <div>{children}</div>\n  </div>\n)\n\nexport default AspectRatioBox");
}
#[test]
fn test_issue_3532_js_format_1_16f90a02() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("import React from 'react';\n\n/*\nimport styled from 'react-emotion';\n\nconst AspectRatioBox = styled.div\\`\n  &::before {\n    content: '';\n    width: 1px;\n    margin-left: -1px;\n    float: left;\n    height: 0;\n    padding-top: \\${props => 100 / props.aspectRatio}%;\n  }\n\n  &::after {\n    /* To clear float *//*\n    content: '';\n    display: table;\n    clear: both;\n  }\n\\`;\n*/\n\nconst AspectRatioBox = ({\n  aspectRatio,\n  children,\n  ...props\n}) => (\n  <div\n    className={\\`height: 0;\n  overflow: hidden;\n  padding-top: \\${props => 100 / props.aspectRatio}%;\n  background: white;\n  position: relative;\\`}\n  >\n    <div>{children}</div>\n  </div>\n);\n\nexport default AspectRatioBox;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\n/*\nimport styled from 'react-emotion';\n\nconst AspectRatioBox = styled.div\\`\n  &::before {\n    content: '';\n    width: 1px;\n    margin-left: -1px;\n    float: left;\n    height: 0;\n    padding-top: \\${props => 100 / props.aspectRatio}%;\n  }\n\n  &::after {\n    /* To clear float */ /*\n    content: '';\n    display: table;\n    clear: both;\n  }\n\\`;\n*/\n\nconst AspectRatioBox = ({ aspectRatio, children, ...props }) => (\n  <div\n    className={\\`height: 0;\n  overflow: hidden;\n  padding-top: \\${(props) => 100 / props.aspectRatio}%;\n  background: white;\n  position: relative;\\`}\n  >\n    <div>{children}</div>\n  </div>\n);\n\nexport default AspectRatioBox;");
}
#[test]
fn test_issues_js_semifalse_format_1_cc374b47() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Does not need to break as it fits in 80 columns\nthis.call(a, /* comment */ b);\n\n// Comments should either stay at the end of the line or always before, but\n// not one before and one after.\nthrow new ProcessSystemError({\n  code: acc.error.code, // Alias of errno\n  originalError: acc.error, // Just in case.\n});\n\n// Missing one level of indentation because of the comment\nconst rootEpic = (actions, store) => (\n  combineEpics(...epics)(actions, store)\n    // Log errors and continue.\n    .catch((err, stream) => {\n      getLogger().error(err);\n      return stream;\n    })\n);\n\n// optional trailing comma gets moved all the way to the beginning\nconst regex = new RegExp(\n  '^\\\\\\\\s*' + // beginning of the line\n  'name\\\\\\\\s*=\\\\\\\\s*' + // name =\n  '[\\\\'\"]' + // opening quotation mark\n  escapeStringRegExp(target.name) + // target name\n  '[\\\\'\"]' + // closing quotation mark\n  ',?$', // optional trailing comma\n);\n\n// The comment is moved and doesn't trigger the eslint rule anymore\nimport path from 'path'; // eslint-disable-line nuclide-internal/prefer-nuclide-uri\n\n// Comments disappear in-between MemberExpressions\nObservable.of(process)\n  // Don't complete until we say so!\n  .merge(Observable.never())\n  // Get the errors.\n  .takeUntil(throwOnError ? errors.flatMap(Observable.throw) : errors)\n  .takeUntil(exit);\n\n// Comments disappear inside of JSX\n<div>\n  {/* Some comment */}\n</div>;\n\n// Comments in JSX tag are placed in a non optimal way\n<div\n  // comment\n/>;\n\n// Comments disappear in empty blocks\nif (1) {\n  // Comment\n}\n\n// Comments trigger invalid JavaScript in-between else if\nif (1) {\n}\n// Comment\nelse {\n\n}\n\n// The comment makes the line break in a weird way\nconst result = asyncExecute('non_existing_command', /* args */ []);\n\n// The closing paren is printed on the same line as the comment\nfoo({}\n  // Hi\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Does not need to break as it fits in 80 columns\nthis.call(a, /* comment */ b)\n\n// Comments should either stay at the end of the line or always before, but\n// not one before and one after.\nthrow new ProcessSystemError({\n  code: acc.error.code, // Alias of errno\n  originalError: acc.error, // Just in case.\n})\n\n// Missing one level of indentation because of the comment\nconst rootEpic = (actions, store) =>\n  combineEpics(...epics)(actions, store)\n    // Log errors and continue.\n    .catch((err, stream) => {\n      getLogger().error(err)\n      return stream\n    })\n\n// optional trailing comma gets moved all the way to the beginning\nconst regex = new RegExp(\n  \"^\\\\\\\\s*\" + // beginning of the line\n    \"name\\\\\\\\s*=\\\\\\\\s*\" + // name =\n    \"['\\\\\"]\" + // opening quotation mark\n    escapeStringRegExp(target.name) + // target name\n    \"['\\\\\"]\" + // closing quotation mark\n    \",?$\", // optional trailing comma\n)\n\n// The comment is moved and doesn't trigger the eslint rule anymore\nimport path from \"path\" // eslint-disable-line nuclide-internal/prefer-nuclide-uri\n\n// Comments disappear in-between MemberExpressions\nObservable.of(process)\n  // Don't complete until we say so!\n  .merge(Observable.never())\n  // Get the errors.\n  .takeUntil(throwOnError ? errors.flatMap(Observable.throw) : errors)\n  .takeUntil(exit)\n\n// Comments disappear inside of JSX\n;<div>{/* Some comment */}</div>\n\n// Comments in JSX tag are placed in a non optimal way\n;<div\n// comment\n/>\n\n// Comments disappear in empty blocks\nif (1) {\n  // Comment\n}\n\n// Comments trigger invalid JavaScript in-between else if\nif (1) {\n}\n// Comment\nelse {\n}\n\n// The comment makes the line break in a weird way\nconst result = asyncExecute(\"non_existing_command\", /* args */ [])\n\n// The closing paren is printed on the same line as the comment\nfoo(\n  {},\n  // Hi\n)");
}
#[test]
fn test_issues_js_format_1_cc374b47() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Does not need to break as it fits in 80 columns\nthis.call(a, /* comment */ b);\n\n// Comments should either stay at the end of the line or always before, but\n// not one before and one after.\nthrow new ProcessSystemError({\n  code: acc.error.code, // Alias of errno\n  originalError: acc.error, // Just in case.\n});\n\n// Missing one level of indentation because of the comment\nconst rootEpic = (actions, store) => (\n  combineEpics(...epics)(actions, store)\n    // Log errors and continue.\n    .catch((err, stream) => {\n      getLogger().error(err);\n      return stream;\n    })\n);\n\n// optional trailing comma gets moved all the way to the beginning\nconst regex = new RegExp(\n  '^\\\\\\\\s*' + // beginning of the line\n  'name\\\\\\\\s*=\\\\\\\\s*' + // name =\n  '[\\\\'\"]' + // opening quotation mark\n  escapeStringRegExp(target.name) + // target name\n  '[\\\\'\"]' + // closing quotation mark\n  ',?$', // optional trailing comma\n);\n\n// The comment is moved and doesn't trigger the eslint rule anymore\nimport path from 'path'; // eslint-disable-line nuclide-internal/prefer-nuclide-uri\n\n// Comments disappear in-between MemberExpressions\nObservable.of(process)\n  // Don't complete until we say so!\n  .merge(Observable.never())\n  // Get the errors.\n  .takeUntil(throwOnError ? errors.flatMap(Observable.throw) : errors)\n  .takeUntil(exit);\n\n// Comments disappear inside of JSX\n<div>\n  {/* Some comment */}\n</div>;\n\n// Comments in JSX tag are placed in a non optimal way\n<div\n  // comment\n/>;\n\n// Comments disappear in empty blocks\nif (1) {\n  // Comment\n}\n\n// Comments trigger invalid JavaScript in-between else if\nif (1) {\n}\n// Comment\nelse {\n\n}\n\n// The comment makes the line break in a weird way\nconst result = asyncExecute('non_existing_command', /* args */ []);\n\n// The closing paren is printed on the same line as the comment\nfoo({}\n  // Hi\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Does not need to break as it fits in 80 columns\nthis.call(a, /* comment */ b);\n\n// Comments should either stay at the end of the line or always before, but\n// not one before and one after.\nthrow new ProcessSystemError({\n  code: acc.error.code, // Alias of errno\n  originalError: acc.error, // Just in case.\n});\n\n// Missing one level of indentation because of the comment\nconst rootEpic = (actions, store) =>\n  combineEpics(...epics)(actions, store)\n    // Log errors and continue.\n    .catch((err, stream) => {\n      getLogger().error(err);\n      return stream;\n    });\n\n// optional trailing comma gets moved all the way to the beginning\nconst regex = new RegExp(\n  \"^\\\\\\\\s*\" + // beginning of the line\n    \"name\\\\\\\\s*=\\\\\\\\s*\" + // name =\n    \"['\\\\\"]\" + // opening quotation mark\n    escapeStringRegExp(target.name) + // target name\n    \"['\\\\\"]\" + // closing quotation mark\n    \",?$\", // optional trailing comma\n);\n\n// The comment is moved and doesn't trigger the eslint rule anymore\nimport path from \"path\"; // eslint-disable-line nuclide-internal/prefer-nuclide-uri\n\n// Comments disappear in-between MemberExpressions\nObservable.of(process)\n  // Don't complete until we say so!\n  .merge(Observable.never())\n  // Get the errors.\n  .takeUntil(throwOnError ? errors.flatMap(Observable.throw) : errors)\n  .takeUntil(exit);\n\n// Comments disappear inside of JSX\n<div>{/* Some comment */}</div>;\n\n// Comments in JSX tag are placed in a non optimal way\n<div\n// comment\n/>;\n\n// Comments disappear in empty blocks\nif (1) {\n  // Comment\n}\n\n// Comments trigger invalid JavaScript in-between else if\nif (1) {\n}\n// Comment\nelse {\n}\n\n// The comment makes the line break in a weird way\nconst result = asyncExecute(\"non_existing_command\", /* args */ []);\n\n// The closing paren is printed on the same line as the comment\nfoo(\n  {},\n  // Hi\n);");
}
#[test]
fn test_jsdoc_js_semifalse_format_1_cfee94fb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/** @type {any} */\nconst x = (\n    <div>\n        <div />\n    </div>\n);\n\n/**\n * @type {object}\n */\n() => (\n    <div>\n        sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp\n    </div>\n);\n\n/**\n * @type {object}\n */\nfunction HelloWorld() {\n    return (\n        <div>\n           <span>Test</span>\n        </div>\n    );\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @type {any} */\nconst x = (\n  <div>\n    <div />\n  </div>\n)\n\n/**\n * @type {object}\n */\n;() => (\n  <div>\n    sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp\n  </div>\n)\n\n/**\n * @type {object}\n */\nfunction HelloWorld() {\n  return (\n    <div>\n      <span>Test</span>\n    </div>\n  )\n}");
}
#[test]
fn test_jsdoc_js_format_1_cfee94fb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/** @type {any} */\nconst x = (\n    <div>\n        <div />\n    </div>\n);\n\n/**\n * @type {object}\n */\n() => (\n    <div>\n        sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp\n    </div>\n);\n\n/**\n * @type {object}\n */\nfunction HelloWorld() {\n    return (\n        <div>\n           <span>Test</span>\n        </div>\n    );\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @type {any} */\nconst x = (\n  <div>\n    <div />\n  </div>\n);\n\n/**\n * @type {object}\n */\n() => (\n  <div>\n    sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp\n  </div>\n);\n\n/**\n * @type {object}\n */\nfunction HelloWorld() {\n  return (\n    <div>\n      <span>Test</span>\n    </div>\n  );\n}");
}
#[test]
fn test_jsdoc_nestled_js_semifalse_format_1_5874d2ca() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const issues = {\n  see: \"#7724 and #12653\"\n  /** Trailing comment 1 (not nestled as both comments should be multiline for that) *//**\n  * Trailing comment 2\n  */\n};\n\n/**\n * @template T\n * @param {Type} type\n * @param {T} value\n * @return {Value}\n *//**\n * @param {Type} type\n * @return {Value}\n */\nfunction value(type, value) {\n  if (arguments.length === 2) {\n    return new ConcreteValue(type, value);\n  } else {\n    return new Value(type);\n  }\n}\n\n/** Trailing nestled comment 1\n *//** Trailing nestled comment 2\n *//** Trailing nestled comment 3\n */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const issues = {\n  see: \"#7724 and #12653\",\n  /** Trailing comment 1 (not nestled as both comments should be multiline for that) */ /**\n   * Trailing comment 2\n   */\n}\n\n/**\n * @template T\n * @param {Type} type\n * @param {T} value\n * @return {Value}\n *//**\n * @param {Type} type\n * @return {Value}\n */\nfunction value(type, value) {\n  if (arguments.length === 2) {\n    return new ConcreteValue(type, value)\n  } else {\n    return new Value(type)\n  }\n}\n\n/** Trailing nestled comment 1\n *//** Trailing nestled comment 2\n *//** Trailing nestled comment 3\n */");
}
#[test]
fn test_jsdoc_nestled_js_format_1_5874d2ca() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const issues = {\n  see: \"#7724 and #12653\"\n  /** Trailing comment 1 (not nestled as both comments should be multiline for that) *//**\n  * Trailing comment 2\n  */\n};\n\n/**\n * @template T\n * @param {Type} type\n * @param {T} value\n * @return {Value}\n *//**\n * @param {Type} type\n * @return {Value}\n */\nfunction value(type, value) {\n  if (arguments.length === 2) {\n    return new ConcreteValue(type, value);\n  } else {\n    return new Value(type);\n  }\n}\n\n/** Trailing nestled comment 1\n *//** Trailing nestled comment 2\n *//** Trailing nestled comment 3\n */") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const issues = {\n  see: \"#7724 and #12653\",\n  /** Trailing comment 1 (not nestled as both comments should be multiline for that) */ /**\n   * Trailing comment 2\n   */\n};\n\n/**\n * @template T\n * @param {Type} type\n * @param {T} value\n * @return {Value}\n *//**\n * @param {Type} type\n * @return {Value}\n */\nfunction value(type, value) {\n  if (arguments.length === 2) {\n    return new ConcreteValue(type, value);\n  } else {\n    return new Value(type);\n  }\n}\n\n/** Trailing nestled comment 1\n *//** Trailing nestled comment 2\n *//** Trailing nestled comment 3\n */");
}
#[test]
fn test_jsdoc_nestled_dangling_js_semifalse_format_1_e3d6ffb9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{{{{{{{\no={\n  /**\n   * A\n   *//**\n   * B\n   */\n\n}\n}}}}}}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  {\n    {\n      {\n        {\n          {\n            {\n              o = {\n                /**\n                 * A\n                 *//**\n                 * B\n                 */\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}");
}
#[test]
fn test_jsdoc_nestled_dangling_js_format_1_e3d6ffb9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("{{{{{{{\no={\n  /**\n   * A\n   *//**\n   * B\n   */\n\n}\n}}}}}}}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "{\n  {\n    {\n      {\n        {\n          {\n            {\n              o = {\n                /**\n                 * A\n                 *//**\n                 * B\n                 */\n              };\n            }\n          }\n        }\n      }\n    }\n  }\n}");
}
#[test]
fn test_jsx_js_semifalse_format_1_32532348() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div>\n  {\n    /* comment */\n  }\n</div>;\n\n<div>\n  {/* comment */\n  }\n</div>;\n\n<div>\n  {/* comment\n*/\n  }\n</div>;\n\n<div>\n  {a/* comment\n*/\n  }\n</div>;\n\n<div>\n  {/* comment\n*/\n  a\n  }\n</div>;\n\n<div>\n  {/* comment */\n  }\n</div>;\n\n<div>\n  {/* comment */}\n</div>;\n\n<div>\n  {\n    // single line comment\n  }\n</div>;\n\n<div>\n  {\n    // multiple line comments 1\n    // multiple line comments 2\n  }\n</div>;\n\n<div>\n  {\n    // multiple mixed comments 1\n    /* multiple mixed comments 2 */\n    /* multiple mixed comments 3 */\n    // multiple mixed comments 4\n  }\n</div>;\n\n<div>\n  {\n    // Some very v  ery very very merry (xmas) very very long line to break line width limit\n  }\n</div>;\n\n<div>{/*<div>  Some very v  ery very very long line to break line width limit </div>*/}</div>;\n\n<div>\n  {/**\n   * JSDoc-y comment in JSX. I wonder what will happen to it?\n  */}\n</div>;\n\n<div>\n  {\n    /**\n   * Another JSDoc comment in JSX.\n  */\n  }\n</div>;\n\n<div\n  /**\n * Handles clicks.\n*/\nonClick={() => {}}>\n\n</div>;\n\n<div\n  // comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\" // comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\"\n  // comment\n>\n  {foo}\n</div>;\n\n<div // comment\n  id=\"foo\"\n>\n  {children}\n</div>;\n\n<Wrapper>\n  {}\n  <Component />\n</Wrapper>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ";<div>{/* comment */}</div>\n\n;<div>{/* comment */}</div>\n\n;<div>\n  {/* comment\n   */}\n</div>\n\n;<div>\n  {\n    a /* comment\n     */\n  }\n</div>\n\n;<div>\n  {\n    /* comment\n     */\n    a\n  }\n</div>\n\n;<div>{/* comment */}</div>\n\n;<div>{/* comment */}</div>\n\n;<div>\n  {\n    // single line comment\n  }\n</div>\n\n;<div>\n  {\n    // multiple line comments 1\n    // multiple line comments 2\n  }\n</div>\n\n;<div>\n  {\n    // multiple mixed comments 1\n    /* multiple mixed comments 2 */\n    /* multiple mixed comments 3 */\n    // multiple mixed comments 4\n  }\n</div>\n\n;<div>\n  {\n    // Some very v  ery very very merry (xmas) very very long line to break line width limit\n  }\n</div>\n\n;<div>\n  {/*<div>  Some very v  ery very very long line to break line width limit </div>*/}\n</div>\n\n;<div>\n  {/**\n   * JSDoc-y comment in JSX. I wonder what will happen to it?\n   */}\n</div>\n\n;<div>\n  {/**\n   * Another JSDoc comment in JSX.\n   */}\n</div>\n\n;<div\n  /**\n   * Handles clicks.\n   */\n  onClick={() => {}}\n></div>\n\n;<div\n// comment\n>\n  {foo}\n</div>\n\n;<div\n  className=\"foo\" // comment\n>\n  {foo}\n</div>\n\n;<div\n  className=\"foo\"\n  // comment\n>\n  {foo}\n</div>\n\n;<div // comment\n  id=\"foo\"\n>\n  {children}\n</div>\n\n;<Wrapper>\n  {}\n  <Component />\n</Wrapper>");
}
#[test]
fn test_jsx_js_format_1_32532348() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div>\n  {\n    /* comment */\n  }\n</div>;\n\n<div>\n  {/* comment */\n  }\n</div>;\n\n<div>\n  {/* comment\n*/\n  }\n</div>;\n\n<div>\n  {a/* comment\n*/\n  }\n</div>;\n\n<div>\n  {/* comment\n*/\n  a\n  }\n</div>;\n\n<div>\n  {/* comment */\n  }\n</div>;\n\n<div>\n  {/* comment */}\n</div>;\n\n<div>\n  {\n    // single line comment\n  }\n</div>;\n\n<div>\n  {\n    // multiple line comments 1\n    // multiple line comments 2\n  }\n</div>;\n\n<div>\n  {\n    // multiple mixed comments 1\n    /* multiple mixed comments 2 */\n    /* multiple mixed comments 3 */\n    // multiple mixed comments 4\n  }\n</div>;\n\n<div>\n  {\n    // Some very v  ery very very merry (xmas) very very long line to break line width limit\n  }\n</div>;\n\n<div>{/*<div>  Some very v  ery very very long line to break line width limit </div>*/}</div>;\n\n<div>\n  {/**\n   * JSDoc-y comment in JSX. I wonder what will happen to it?\n  */}\n</div>;\n\n<div>\n  {\n    /**\n   * Another JSDoc comment in JSX.\n  */\n  }\n</div>;\n\n<div\n  /**\n * Handles clicks.\n*/\nonClick={() => {}}>\n\n</div>;\n\n<div\n  // comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\" // comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\"\n  // comment\n>\n  {foo}\n</div>;\n\n<div // comment\n  id=\"foo\"\n>\n  {children}\n</div>;\n\n<Wrapper>\n  {}\n  <Component />\n</Wrapper>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div>{/* comment */}</div>;\n\n<div>{/* comment */}</div>;\n\n<div>\n  {/* comment\n   */}\n</div>;\n\n<div>\n  {\n    a /* comment\n     */\n  }\n</div>;\n\n<div>\n  {\n    /* comment\n     */\n    a\n  }\n</div>;\n\n<div>{/* comment */}</div>;\n\n<div>{/* comment */}</div>;\n\n<div>\n  {\n    // single line comment\n  }\n</div>;\n\n<div>\n  {\n    // multiple line comments 1\n    // multiple line comments 2\n  }\n</div>;\n\n<div>\n  {\n    // multiple mixed comments 1\n    /* multiple mixed comments 2 */\n    /* multiple mixed comments 3 */\n    // multiple mixed comments 4\n  }\n</div>;\n\n<div>\n  {\n    // Some very v  ery very very merry (xmas) very very long line to break line width limit\n  }\n</div>;\n\n<div>\n  {/*<div>  Some very v  ery very very long line to break line width limit </div>*/}\n</div>;\n\n<div>\n  {/**\n   * JSDoc-y comment in JSX. I wonder what will happen to it?\n   */}\n</div>;\n\n<div>\n  {/**\n   * Another JSDoc comment in JSX.\n   */}\n</div>;\n\n<div\n  /**\n   * Handles clicks.\n   */\n  onClick={() => {}}\n></div>;\n\n<div\n// comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\" // comment\n>\n  {foo}\n</div>;\n\n<div\n  className=\"foo\"\n  // comment\n>\n  {foo}\n</div>;\n\n<div // comment\n  id=\"foo\"\n>\n  {children}\n</div>;\n\n<Wrapper>\n  {}\n  <Component />\n</Wrapper>;");
}
#[test]
fn test_last_arg_js_semifalse_format_1_c04b3b51() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\nclass Foo {\n  a(lol /*string*/) {}\n\n  b(lol /*string*/\n  ) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) /*string*/ {}\n\n  // prettier-ignore\n  c(lol /*string*/\n  ) {}\n\n  // prettier-ignore\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  // prettier-ignore\n  e(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {} /* string*/\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  a(lol /*string*/) {}\n\n  b(lol /*string*/) {}\n\n  d(lol /*string*/, lol2 /*string*/, lol3 /*string*/, lol4 /*string*/) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/,\n  ) /*string*/ {}\n\n  // prettier-ignore\n  c(lol /*string*/\n  ) {}\n\n  // prettier-ignore\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  // prettier-ignore\n  e(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {} /* string*/\n}");
}
#[test]
fn test_last_arg_js_format_1_c04b3b51() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\nclass Foo {\n  a(lol /*string*/) {}\n\n  b(lol /*string*/\n  ) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) /*string*/ {}\n\n  // prettier-ignore\n  c(lol /*string*/\n  ) {}\n\n  // prettier-ignore\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  // prettier-ignore\n  e(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {} /* string*/\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class Foo {\n  a(lol /*string*/) {}\n\n  b(lol /*string*/) {}\n\n  d(lol /*string*/, lol2 /*string*/, lol3 /*string*/, lol4 /*string*/) {}\n\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/,\n  ) /*string*/ {}\n\n  // prettier-ignore\n  c(lol /*string*/\n  ) {}\n\n  // prettier-ignore\n  d(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {}\n\n  // prettier-ignore\n  e(\n    lol /*string*/,\n    lol2 /*string*/,\n    lol3 /*string*/,\n    lol4 /*string*/\n  ) {} /* string*/\n}");
}
#[test]
fn test_multi_comments_js_semifalse_format_1_d35ffa10() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #8323\n\nimport { MapViewProps } from 'react-native-maps'; /*\ncomment 14\n*/ /* comment1\n10\n*/ /*/ comment 13 */\n/*\n comment 9\n ****/\nimport * as ts from 'typescript';\n\nx; /*\n1 */ /* 2 */\n\ny\n\nx; /*1*//*2*/\ny;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #8323\n\nimport { MapViewProps } from \"react-native-maps\" /*\ncomment 14\n*/ /* comment1\n10\n*/ /*/ comment 13 */\n/*\n comment 9\n ****/\nimport * as ts from \"typescript\"\n\nx /*\n1 */ /* 2 */\n\ny\n\nx /*1*/ /*2*/\ny");
}
#[test]
fn test_multi_comments_js_format_1_d35ffa10() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #8323\n\nimport { MapViewProps } from 'react-native-maps'; /*\ncomment 14\n*/ /* comment1\n10\n*/ /*/ comment 13 */\n/*\n comment 9\n ****/\nimport * as ts from 'typescript';\n\nx; /*\n1 */ /* 2 */\n\ny\n\nx; /*1*//*2*/\ny;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #8323\n\nimport { MapViewProps } from \"react-native-maps\"; /*\ncomment 14\n*/ /* comment1\n10\n*/ /*/ comment 13 */\n/*\n comment 9\n ****/\nimport * as ts from \"typescript\";\n\nx; /*\n1 */ /* 2 */\n\ny;\n\nx; /*1*/ /*2*/\ny;");
}
#[test]
fn test_multi_comments_2_js_semifalse_format_1_0ecea16e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  const x = 0;\n  \n  /* istanbul ignore if */ // debug case currently not triggered\n  if (true) {\n    x;\n  }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = 0\n\n/* istanbul ignore if */ // debug case currently not triggered\nif (true) {\n  x\n}");
}
#[test]
fn test_multi_comments_2_js_format_1_0ecea16e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("  const x = 0;\n  \n  /* istanbul ignore if */ // debug case currently not triggered\n  if (true) {\n    x;\n  }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const x = 0;\n\n/* istanbul ignore if */ // debug case currently not triggered\nif (true) {\n  x;\n}");
}
#[test]
fn test_multi_comments_on_same_line_js_semifalse_format_1_7d1ce939() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/*========= All on same line =========*/\na;\n/*1*//*2*//*3*/\nb;\n\na;/*1*//*2*//*3*/\nb;\n\na;\n/*1*//*2*//*3*/b;\n\na;\n/*\n1*//*2*//*3\n*/\nb;\n\na;/*\n1*//*2*//*3\n*/\nb;\n\na;/*\n1*//*2*//*3\n*/b;\n\n/*========= First two on same line =========*/\na;\n/*1*//*2*/\n/*3*/\nb;\n\na;/*1*//*2*/\n/*3*/\nb;\n\na;\n/*1*//*2*/\n/*3*/b;\n\na;\n/*\n1*//*2*/\n/*3\n*/\nb;\n\na;/*\n1*//*2*/\n/*3\n*/\nb;\n\na;/*\n1*//*2*/\n/*3\n*/b;\n\n/*========= Last two on same line =========*/\na;\n/*1*/\n/*2*//*3*/\nb;\n\na;/*1*/\n/*2*//*3*/\nb;\n\na;\n/*1*/\n/*2*//*3*/b;\n\na;\n/*\n1*/\n/*2*//*3\n*/\nb;\n\na;/*\n1*/\n/*2*//*3\n*/\nb;\n\na;/*\n1*/\n/*2*//*3\n*/b;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*========= All on same line =========*/\na\n/*1*/ /*2*/ /*3*/\nb\n\na /*1*/ /*2*/ /*3*/\nb\n\na\n/*1*/ /*2*/ /*3*/ b\n\na\n/*\n1*/ /*2*/ /*3\n */\nb\n\na /*\n1*/ /*2*/ /*3\n */\nb\n\na\n/*\n1*/ /*2*/ /*3\n */ b\n\n/*========= First two on same line =========*/\na\n/*1*/ /*2*/\n/*3*/\nb\n\na /*1*/ /*2*/\n/*3*/\nb\n\na\n/*1*/ /*2*/\n/*3*/ b\n\na\n/*\n1*/ /*2*/\n/*3\n */\nb\n\na /*\n1*/ /*2*/\n/*3\n */\nb\n\na /*\n1*/ /*2*/\n/*3\n */ b\n\n/*========= Last two on same line =========*/\na\n/*1*/\n/*2*/ /*3*/\nb\n\na /*1*/\n/*2*/ /*3*/\nb\n\na\n/*1*/\n/*2*/ /*3*/ b\n\na\n/*\n1*/\n/*2*/ /*3\n */\nb\n\na /*\n1*/\n/*2*/ /*3\n */\nb\n\na /*\n1*/\n/*2*/ /*3\n */ b");
}
#[test]
fn test_multi_comments_on_same_line_js_format_1_7d1ce939() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/*========= All on same line =========*/\na;\n/*1*//*2*//*3*/\nb;\n\na;/*1*//*2*//*3*/\nb;\n\na;\n/*1*//*2*//*3*/b;\n\na;\n/*\n1*//*2*//*3\n*/\nb;\n\na;/*\n1*//*2*//*3\n*/\nb;\n\na;/*\n1*//*2*//*3\n*/b;\n\n/*========= First two on same line =========*/\na;\n/*1*//*2*/\n/*3*/\nb;\n\na;/*1*//*2*/\n/*3*/\nb;\n\na;\n/*1*//*2*/\n/*3*/b;\n\na;\n/*\n1*//*2*/\n/*3\n*/\nb;\n\na;/*\n1*//*2*/\n/*3\n*/\nb;\n\na;/*\n1*//*2*/\n/*3\n*/b;\n\n/*========= Last two on same line =========*/\na;\n/*1*/\n/*2*//*3*/\nb;\n\na;/*1*/\n/*2*//*3*/\nb;\n\na;\n/*1*/\n/*2*//*3*/b;\n\na;\n/*\n1*/\n/*2*//*3\n*/\nb;\n\na;/*\n1*/\n/*2*//*3\n*/\nb;\n\na;/*\n1*/\n/*2*//*3\n*/b;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*========= All on same line =========*/\na;\n/*1*/ /*2*/ /*3*/\nb;\n\na; /*1*/ /*2*/ /*3*/\nb;\n\na;\n/*1*/ /*2*/ /*3*/ b;\n\na;\n/*\n1*/ /*2*/ /*3\n */\nb;\n\na; /*\n1*/ /*2*/ /*3\n */\nb;\n\na;\n/*\n1*/ /*2*/ /*3\n */ b;\n\n/*========= First two on same line =========*/\na;\n/*1*/ /*2*/\n/*3*/\nb;\n\na; /*1*/ /*2*/\n/*3*/\nb;\n\na;\n/*1*/ /*2*/\n/*3*/ b;\n\na;\n/*\n1*/ /*2*/\n/*3\n */\nb;\n\na; /*\n1*/ /*2*/\n/*3\n */\nb;\n\na; /*\n1*/ /*2*/\n/*3\n */ b;\n\n/*========= Last two on same line =========*/\na;\n/*1*/\n/*2*/ /*3*/\nb;\n\na; /*1*/\n/*2*/ /*3*/\nb;\n\na;\n/*1*/\n/*2*/ /*3*/ b;\n\na;\n/*\n1*/\n/*2*/ /*3\n */\nb;\n\na; /*\n1*/\n/*2*/ /*3\n */\nb;\n\na; /*\n1*/\n/*2*/ /*3\n */ b;");
}
#[test]
fn test_multi_comments_on_same_line_2_js_semifalse_format_1_4319e6af() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* 1 */ /* 2 */ /* 3 */ a;\na; /* 4 */ /* 5 */ /* 6 */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* 1 */ /* 2 */ /* 3 */ a\na /* 4 */ /* 5 */ /* 6 */"
    );
}
#[test]
fn test_multi_comments_on_same_line_2_js_format_1_4319e6af() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("/* 1 */ /* 2 */ /* 3 */ a;\na; /* 4 */ /* 5 */ /* 6 */");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* 1 */ /* 2 */ /* 3 */ a;\na; /* 4 */ /* 5 */ /* 6 */"
    );
}
#[test]
fn test_preserve_new_line_last_js_semifalse_format_1_c5fcc80d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function f() {\n  a\n  /* eslint-disable */\n}\n\nfunction f() {\n  a\n\n  /* eslint-disable */\n}\n\nfunction name() {\n  // comment1\n  func1()\n\n  // comment2\n  func2()\n\n  // comment3 why func3 commented\n  // func3()\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  a\n  /* eslint-disable */\n}\n\nfunction f() {\n  a\n\n  /* eslint-disable */\n}\n\nfunction name() {\n  // comment1\n  func1()\n\n  // comment2\n  func2()\n\n  // comment3 why func3 commented\n  // func3()\n}");
}
#[test]
fn test_preserve_new_line_last_js_format_1_c5fcc80d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function f() {\n  a\n  /* eslint-disable */\n}\n\nfunction f() {\n  a\n\n  /* eslint-disable */\n}\n\nfunction name() {\n  // comment1\n  func1()\n\n  // comment2\n  func2()\n\n  // comment3 why func3 commented\n  // func3()\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  a;\n  /* eslint-disable */\n}\n\nfunction f() {\n  a;\n\n  /* eslint-disable */\n}\n\nfunction name() {\n  // comment1\n  func1();\n\n  // comment2\n  func2();\n\n  // comment3 why func3 commented\n  // func3()\n}");
}
#[test]
fn test_return_statement_js_semifalse_format_1_824cf778() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function jsx() {\n  return (\n    // Comment\n    <div />\n  );\n}\n\nfunction unary() {\n  return (\n    // Comment\n    !!x\n  );\n}\n\nfunction numericLiteralNoParen() {\n  return 1337; // Comment\n}\n\nfunction logical() {\n  return (\n    // Reason for 42\n    42\n  ) && 84\n}\n\nfunction binary() {\n  return (\n    // Reason for 42\n    42\n  ) * 84\n}\n\nfunction binaryInBinaryLeft() {\n  return (\n    // Reason for 42\n    42\n  ) * 84 + 2\n}\n\nfunction binaryInBinaryRight() {\n  return (\n    // Reason for 42\n    42\n  ) + 84 * 2\n}\n\nfunction conditional() {\n  return (\n    // Reason for 42\n    42\n  ) ? 1 : 2\n}\n\nfunction binaryInConditional() {\n  return (\n    // Reason for 42\n    42\n  ) * 3 ? 1 : 2\n}\n\nfunction call() {\n  return (\n    // Reason for a\n    a\n  )()\n}\n\nfunction memberInside() {\n  return (\n    // Reason for a.b\n    a.b\n  ).c\n}\n\nfunction memberOutside() {\n  return (\n    // Reason for a\n    a\n  ).b.c\n}\n\nfunction memberInAndOutWithCalls() {\n  return (\n    // Reason for a\n    aFunction.b()\n  ).c.d()\n}\n\nfunction excessiveEverything() {\n  return (\n    // Reason for stuff\n    a.b() * 3 + 4 ? (a\\`hi\\`, 1) ? 1 : 1 : 1\n  )\n}\n\n// See https://github.com/prettier/prettier/issues/2392\n// function sequenceExpression() {\n//   return (\n//     // Reason for a\n//     a\n//   ), b\n// }\n\nfunction sequenceExpressionInside() {\n  return ( // Reason for a\n    a, b\n  );\n}\n\nfunction taggedTemplate() {\n  return (\n    // Reason for a\n    a\n  )\\`b\\`\n}\n\nfunction inlineComment() {\n  return (\n    /* hi */ 42\n  ) || 42\n}\n\nfunction multilineBlockSameLine() {\n  return (\n    /**\n    * @type {string}\n    */ 'result'\n  )\n}\n\nfunction multilineBlockNextLine() {\n  return (\n    /**\n    * @type {string}\n    */ \n    'result'\n  )\n}\n\nfunction multilineBlockSameLineJsx() {\n  return (\n    /**\n    * JSX Same line\n    */ <div></div>\n  )\n}\n\nfunction multilineBlockNextLineJsx() {\n  return (\n    /**\n    * JSX Next line\n    */\n    <div></div>\n  )\n}\n\nfunction singleLineBlockSameLine() {\n  return (\n    /** Result -> */ 'result'\n  )\n}\n\nfunction singleLineBlockNextLine() {\n  return (\n    /** Result below */ \n    'result'\n  )\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function jsx() {\n  return (\n    // Comment\n    <div />\n  )\n}\n\nfunction unary() {\n  return (\n    // Comment\n    !!x\n  )\n}\n\nfunction numericLiteralNoParen() {\n  return 1337 // Comment\n}\n\nfunction logical() {\n  return (\n    // Reason for 42\n    42 && 84\n  )\n}\n\nfunction binary() {\n  return (\n    // Reason for 42\n    42 * 84\n  )\n}\n\nfunction binaryInBinaryLeft() {\n  return (\n    // Reason for 42\n    42 *\n      84 +\n    2\n  )\n}\n\nfunction binaryInBinaryRight() {\n  return (\n    // Reason for 42\n    42 +\n    84 * 2\n  )\n}\n\nfunction conditional() {\n  return (\n    // Reason for 42\n    42\n      ? 1\n      : 2\n  )\n}\n\nfunction binaryInConditional() {\n  return (\n    // Reason for 42\n    42 * 3\n      ? 1\n      : 2\n  )\n}\n\nfunction call() {\n  return (\n    // Reason for a\n    a()\n  )\n}\n\nfunction memberInside() {\n  return (\n    // Reason for a.b\n    a.b.c\n  )\n}\n\nfunction memberOutside() {\n  return (\n    // Reason for a\n    a.b.c\n  )\n}\n\nfunction memberInAndOutWithCalls() {\n  return (\n    aFunction\n      .b// Reason for a\n      ()\n      .c.d()\n  )\n}\n\nfunction excessiveEverything() {\n  return (\n    // Reason for stuff\n    a.b() * 3 + 4 ? ((a\\`hi\\`, 1) ? 1 : 1) : 1\n  )\n}\n\n// See https://github.com/prettier/prettier/issues/2392\n// function sequenceExpression() {\n//   return (\n//     // Reason for a\n//     a\n//   ), b\n// }\n\nfunction sequenceExpressionInside() {\n  return (\n    // Reason for a\n    a, b\n  )\n}\n\nfunction taggedTemplate() {\n  return (\n    // Reason for a\n    a\\`b\\`\n  )\n}\n\nfunction inlineComment() {\n  return /* hi */ 42 || 42\n}\n\nfunction multilineBlockSameLine() {\n  return (\n    /**\n     * @type {string}\n     */ \"result\"\n  )\n}\n\nfunction multilineBlockNextLine() {\n  return (\n    /**\n     * @type {string}\n     */\n    \"result\"\n  )\n}\n\nfunction multilineBlockSameLineJsx() {\n  return (\n    /**\n     * JSX Same line\n     */ <div></div>\n  )\n}\n\nfunction multilineBlockNextLineJsx() {\n  return (\n    /**\n     * JSX Next line\n     */\n    <div></div>\n  )\n}\n\nfunction singleLineBlockSameLine() {\n  return /** Result -> */ \"result\"\n}\n\nfunction singleLineBlockNextLine() {\n  return (\n    /** Result below */\n    \"result\"\n  )\n}");
}
#[test]
fn test_return_statement_js_format_1_824cf778() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function jsx() {\n  return (\n    // Comment\n    <div />\n  );\n}\n\nfunction unary() {\n  return (\n    // Comment\n    !!x\n  );\n}\n\nfunction numericLiteralNoParen() {\n  return 1337; // Comment\n}\n\nfunction logical() {\n  return (\n    // Reason for 42\n    42\n  ) && 84\n}\n\nfunction binary() {\n  return (\n    // Reason for 42\n    42\n  ) * 84\n}\n\nfunction binaryInBinaryLeft() {\n  return (\n    // Reason for 42\n    42\n  ) * 84 + 2\n}\n\nfunction binaryInBinaryRight() {\n  return (\n    // Reason for 42\n    42\n  ) + 84 * 2\n}\n\nfunction conditional() {\n  return (\n    // Reason for 42\n    42\n  ) ? 1 : 2\n}\n\nfunction binaryInConditional() {\n  return (\n    // Reason for 42\n    42\n  ) * 3 ? 1 : 2\n}\n\nfunction call() {\n  return (\n    // Reason for a\n    a\n  )()\n}\n\nfunction memberInside() {\n  return (\n    // Reason for a.b\n    a.b\n  ).c\n}\n\nfunction memberOutside() {\n  return (\n    // Reason for a\n    a\n  ).b.c\n}\n\nfunction memberInAndOutWithCalls() {\n  return (\n    // Reason for a\n    aFunction.b()\n  ).c.d()\n}\n\nfunction excessiveEverything() {\n  return (\n    // Reason for stuff\n    a.b() * 3 + 4 ? (a\\`hi\\`, 1) ? 1 : 1 : 1\n  )\n}\n\n// See https://github.com/prettier/prettier/issues/2392\n// function sequenceExpression() {\n//   return (\n//     // Reason for a\n//     a\n//   ), b\n// }\n\nfunction sequenceExpressionInside() {\n  return ( // Reason for a\n    a, b\n  );\n}\n\nfunction taggedTemplate() {\n  return (\n    // Reason for a\n    a\n  )\\`b\\`\n}\n\nfunction inlineComment() {\n  return (\n    /* hi */ 42\n  ) || 42\n}\n\nfunction multilineBlockSameLine() {\n  return (\n    /**\n    * @type {string}\n    */ 'result'\n  )\n}\n\nfunction multilineBlockNextLine() {\n  return (\n    /**\n    * @type {string}\n    */ \n    'result'\n  )\n}\n\nfunction multilineBlockSameLineJsx() {\n  return (\n    /**\n    * JSX Same line\n    */ <div></div>\n  )\n}\n\nfunction multilineBlockNextLineJsx() {\n  return (\n    /**\n    * JSX Next line\n    */\n    <div></div>\n  )\n}\n\nfunction singleLineBlockSameLine() {\n  return (\n    /** Result -> */ 'result'\n  )\n}\n\nfunction singleLineBlockNextLine() {\n  return (\n    /** Result below */ \n    'result'\n  )\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function jsx() {\n  return (\n    // Comment\n    <div />\n  );\n}\n\nfunction unary() {\n  return (\n    // Comment\n    !!x\n  );\n}\n\nfunction numericLiteralNoParen() {\n  return 1337; // Comment\n}\n\nfunction logical() {\n  return (\n    // Reason for 42\n    42 && 84\n  );\n}\n\nfunction binary() {\n  return (\n    // Reason for 42\n    42 * 84\n  );\n}\n\nfunction binaryInBinaryLeft() {\n  return (\n    // Reason for 42\n    42 *\n      84 +\n    2\n  );\n}\n\nfunction binaryInBinaryRight() {\n  return (\n    // Reason for 42\n    42 +\n    84 * 2\n  );\n}\n\nfunction conditional() {\n  return (\n    // Reason for 42\n    42\n      ? 1\n      : 2\n  );\n}\n\nfunction binaryInConditional() {\n  return (\n    // Reason for 42\n    42 * 3\n      ? 1\n      : 2\n  );\n}\n\nfunction call() {\n  return (\n    // Reason for a\n    a()\n  );\n}\n\nfunction memberInside() {\n  return (\n    // Reason for a.b\n    a.b.c\n  );\n}\n\nfunction memberOutside() {\n  return (\n    // Reason for a\n    a.b.c\n  );\n}\n\nfunction memberInAndOutWithCalls() {\n  return (\n    aFunction\n      .b// Reason for a\n      ()\n      .c.d()\n  );\n}\n\nfunction excessiveEverything() {\n  return (\n    // Reason for stuff\n    a.b() * 3 + 4 ? ((a\\`hi\\`, 1) ? 1 : 1) : 1\n  );\n}\n\n// See https://github.com/prettier/prettier/issues/2392\n// function sequenceExpression() {\n//   return (\n//     // Reason for a\n//     a\n//   ), b\n// }\n\nfunction sequenceExpressionInside() {\n  return (\n    // Reason for a\n    a, b\n  );\n}\n\nfunction taggedTemplate() {\n  return (\n    // Reason for a\n    a\\`b\\`\n  );\n}\n\nfunction inlineComment() {\n  return /* hi */ 42 || 42;\n}\n\nfunction multilineBlockSameLine() {\n  return (\n    /**\n     * @type {string}\n     */ \"result\"\n  );\n}\n\nfunction multilineBlockNextLine() {\n  return (\n    /**\n     * @type {string}\n     */\n    \"result\"\n  );\n}\n\nfunction multilineBlockSameLineJsx() {\n  return (\n    /**\n     * JSX Same line\n     */ <div></div>\n  );\n}\n\nfunction multilineBlockNextLineJsx() {\n  return (\n    /**\n     * JSX Next line\n     */\n    <div></div>\n  );\n}\n\nfunction singleLineBlockSameLine() {\n  return /** Result -> */ \"result\";\n}\n\nfunction singleLineBlockNextLine() {\n  return (\n    /** Result below */\n    \"result\"\n  );\n}");
}
#[test]
fn test_single_star_jsdoc_js_semifalse_format_1_eaf1095e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/*\n * Looking good!\n */\n\nif(true) {\n    /*\n     * Oh no\n     */\n}\n\n  /** first line\n* second line\n   * third line */\n\n  /* first line\n* second line\n   * third line */\n\n  /*! first line\n*second line\n   *  third line */\n\n/*!\n* Extracted from vue codebase\n* https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js\n* HTML Parser By John Resig (ejohn.org)\n* Modified by Juriy \"kangax\" Zaytsev\n* Original code by Erik Arvidsson, Mozilla Public License\n* http://erik.eae.net/simplehtmlparser/simplehtmlparser.js\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * Looking good!\n */\n\nif (true) {\n  /*\n   * Oh no\n   */\n}\n\n/** first line\n * second line\n * third line */\n\n/* first line\n * second line\n * third line */\n\n/*! first line\n *second line\n *  third line */\n\n/*!\n * Extracted from vue codebase\n * https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js\n * HTML Parser By John Resig (ejohn.org)\n * Modified by Juriy \"kangax\" Zaytsev\n * Original code by Erik Arvidsson, Mozilla Public License\n * http://erik.eae.net/simplehtmlparser/simplehtmlparser.js\n */");
}
#[test]
fn test_single_star_jsdoc_js_format_1_eaf1095e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("/*\n * Looking good!\n */\n\nif(true) {\n    /*\n     * Oh no\n     */\n}\n\n  /** first line\n* second line\n   * third line */\n\n  /* first line\n* second line\n   * third line */\n\n  /*! first line\n*second line\n   *  third line */\n\n/*!\n* Extracted from vue codebase\n* https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js\n* HTML Parser By John Resig (ejohn.org)\n* Modified by Juriy \"kangax\" Zaytsev\n* Original code by Erik Arvidsson, Mozilla Public License\n* http://erik.eae.net/simplehtmlparser/simplehtmlparser.js\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/*\n * Looking good!\n */\n\nif (true) {\n  /*\n   * Oh no\n   */\n}\n\n/** first line\n * second line\n * third line */\n\n/* first line\n * second line\n * third line */\n\n/*! first line\n *second line\n *  third line */\n\n/*!\n * Extracted from vue codebase\n * https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js\n * HTML Parser By John Resig (ejohn.org)\n * Modified by Juriy \"kangax\" Zaytsev\n * Original code by Erik Arvidsson, Mozilla Public License\n * http://erik.eae.net/simplehtmlparser/simplehtmlparser.js\n */");
}
#[test]
fn test_switch_js_semifalse_format_1_e9973c13() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("switch (node && node.type) {\n  case \"Property\":\n  case \"MethodDefinition\":\n    prop = node.key;\n    break;\n\n  case \"MemberExpression\":\n    prop = node.property;\n    break;\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\":\n    doThing()\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\": //comment\n    doThing(); //comment\n\n  case \"baz\":\n    doOtherThing(); //comment\n\n}\n\nswitch (foo) {\n  case \"bar\": {\n    doThing();\n  } //comment\n\n  case \"baz\": {\n    doThing();\n  } //comment\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (node && node.type) {\n  case \"Property\":\n  case \"MethodDefinition\":\n    prop = node.key\n    break\n\n  case \"MemberExpression\":\n    prop = node.property\n    break\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\":\n    doThing()\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\": //comment\n    doThing() //comment\n\n  case \"baz\":\n    doOtherThing() //comment\n}\n\nswitch (foo) {\n  case \"bar\": {\n    doThing()\n  } //comment\n\n  case \"baz\": {\n    doThing()\n  } //comment\n}");
}
#[test]
fn test_switch_js_format_1_e9973c13() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("switch (node && node.type) {\n  case \"Property\":\n  case \"MethodDefinition\":\n    prop = node.key;\n    break;\n\n  case \"MemberExpression\":\n    prop = node.property;\n    break;\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\":\n    doThing()\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\": //comment\n    doThing(); //comment\n\n  case \"baz\":\n    doOtherThing(); //comment\n\n}\n\nswitch (foo) {\n  case \"bar\": {\n    doThing();\n  } //comment\n\n  case \"baz\": {\n    doThing();\n  } //comment\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "switch (node && node.type) {\n  case \"Property\":\n  case \"MethodDefinition\":\n    prop = node.key;\n    break;\n\n  case \"MemberExpression\":\n    prop = node.property;\n    break;\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\":\n    doThing();\n\n  // no default\n}\n\nswitch (foo) {\n  case \"bar\": //comment\n    doThing(); //comment\n\n  case \"baz\":\n    doOtherThing(); //comment\n}\n\nswitch (foo) {\n  case \"bar\": {\n    doThing();\n  } //comment\n\n  case \"baz\": {\n    doThing();\n  } //comment\n}");
}
#[test]
fn test_tagged_template_literal_js_semifalse_format_1_e740565b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("foo\\`\\`; // comment\n\nfoo // comment\n\\`\\`;\n\nfoo // comment\n\\`\n\\`;\n\nfoo /* comment */\\`\n\\`;\n\nfoo /* comment */\n\\`\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo\\`\\` // comment\n\nfoo // comment\n\\`\\`\n\nfoo // comment\n\\`\n\\`\n\nfoo/* comment */ \\`\n\\`\n\nfoo /* comment */\\`\n\\`");
}
#[test]
fn test_tagged_template_literal_js_format_1_e740565b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("foo\\`\\`; // comment\n\nfoo // comment\n\\`\\`;\n\nfoo // comment\n\\`\n\\`;\n\nfoo /* comment */\\`\n\\`;\n\nfoo /* comment */\n\\`\n\\`;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo\\`\\`; // comment\n\nfoo // comment\n\\`\\`;\n\nfoo // comment\n\\`\n\\`;\n\nfoo/* comment */ \\`\n\\`;\n\nfoo /* comment */\\`\n\\`;");
}
#[test]
fn test_template_literal_js_semifalse_format_1_b1d2dafc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\\`\n\\${a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${// comment\nd //comment\n};\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , ";\\`\n\\${\n  a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${\n  // comment\n  d //comment\n};\n\\`");
}
#[test]
fn test_template_literal_js_format_1_b1d2dafc() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("\\`\n\\${a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${// comment\nd //comment\n};\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "\\`\n\\${\n  a // comment\n}\n\n\\${b /* comment */}\n\n\\${/* comment */ c /* comment */}\n\n\\${\n  // comment\n  d //comment\n};\n\\`;");
}
#[test]
fn test_trailing_space_js_semifalse_format_1_1c9e948d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("#!/there/is-space-here->         \n\n// Do not trim trailing whitespace from this source file!\n\n// There is some space here ->                        ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "#!/there/is-space-here->\n\n// Do not trim trailing whitespace from this source file!\n\n// There is some space here ->");
}
#[test]
fn test_trailing_space_js_format_1_1c9e948d() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("#!/there/is-space-here->         \n\n// Do not trim trailing whitespace from this source file!\n\n// There is some space here ->                        ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "#!/there/is-space-here->\n\n// Do not trim trailing whitespace from this source file!\n\n// There is some space here ->");
}
#[test]
fn test_trailing_jsdocs_js_semifalse_format_1_a916a962() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const CONNECTION_STATUS = exports.CONNECTION_STATUS = {\n  CLOSED: Object.freeze({ kind: 'CLOSED' }),\n  CONNECTED: Object.freeze({ kind: 'CONNECTED' }),\n  CONNECTING: Object.freeze({ kind: 'CONNECTING' }),\n  NOT_CONNECTED: Object.freeze({ kind: 'NOT_CONNECTED' }) };\n\n/* A comment */ /**\n* A type that can be written to a buffer.\n*/ /**\n* Describes the connection status of a ReactiveSocket/DuplexConnection.\n* - NOT_CONNECTED: no connection established or pending.\n* - CONNECTING: when \\`connect()\\` has been called but a connection is not yet\n*   established.\n* - CONNECTED: when a connection is established.\n* - CLOSED: when the connection has been explicitly closed via \\`close()\\`.\n* - ERROR: when the connection has been closed for any other reason.\n*/ /**\n* A contract providing different interaction models per the [ReactiveSocket protocol]\n* (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).\n*/ /**\n* A single unit of data exchanged between the peers of a \\`ReactiveSocket\\`.\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const CONNECTION_STATUS = (exports.CONNECTION_STATUS = {\n  CLOSED: Object.freeze({ kind: \"CLOSED\" }),\n  CONNECTED: Object.freeze({ kind: \"CONNECTED\" }),\n  CONNECTING: Object.freeze({ kind: \"CONNECTING\" }),\n  NOT_CONNECTED: Object.freeze({ kind: \"NOT_CONNECTED\" }),\n})\n\n/* A comment */ /**\n * A type that can be written to a buffer.\n */ /**\n * Describes the connection status of a ReactiveSocket/DuplexConnection.\n * - NOT_CONNECTED: no connection established or pending.\n * - CONNECTING: when \\`connect()\\` has been called but a connection is not yet\n *   established.\n * - CONNECTED: when a connection is established.\n * - CLOSED: when the connection has been explicitly closed via \\`close()\\`.\n * - ERROR: when the connection has been closed for any other reason.\n */ /**\n * A contract providing different interaction models per the [ReactiveSocket protocol]\n * (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).\n */ /**\n * A single unit of data exchanged between the peers of a \\`ReactiveSocket\\`.\n */");
}
#[test]
fn test_trailing_jsdocs_js_format_1_a916a962() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("const CONNECTION_STATUS = exports.CONNECTION_STATUS = {\n  CLOSED: Object.freeze({ kind: 'CLOSED' }),\n  CONNECTED: Object.freeze({ kind: 'CONNECTED' }),\n  CONNECTING: Object.freeze({ kind: 'CONNECTING' }),\n  NOT_CONNECTED: Object.freeze({ kind: 'NOT_CONNECTED' }) };\n\n/* A comment */ /**\n* A type that can be written to a buffer.\n*/ /**\n* Describes the connection status of a ReactiveSocket/DuplexConnection.\n* - NOT_CONNECTED: no connection established or pending.\n* - CONNECTING: when \\`connect()\\` has been called but a connection is not yet\n*   established.\n* - CONNECTED: when a connection is established.\n* - CLOSED: when the connection has been explicitly closed via \\`close()\\`.\n* - ERROR: when the connection has been closed for any other reason.\n*/ /**\n* A contract providing different interaction models per the [ReactiveSocket protocol]\n* (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).\n*/ /**\n* A single unit of data exchanged between the peers of a \\`ReactiveSocket\\`.\n*/") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const CONNECTION_STATUS = (exports.CONNECTION_STATUS = {\n  CLOSED: Object.freeze({ kind: \"CLOSED\" }),\n  CONNECTED: Object.freeze({ kind: \"CONNECTED\" }),\n  CONNECTING: Object.freeze({ kind: \"CONNECTING\" }),\n  NOT_CONNECTED: Object.freeze({ kind: \"NOT_CONNECTED\" }),\n});\n\n/* A comment */ /**\n * A type that can be written to a buffer.\n */ /**\n * Describes the connection status of a ReactiveSocket/DuplexConnection.\n * - NOT_CONNECTED: no connection established or pending.\n * - CONNECTING: when \\`connect()\\` has been called but a connection is not yet\n *   established.\n * - CONNECTED: when a connection is established.\n * - CLOSED: when the connection has been explicitly closed via \\`close()\\`.\n * - ERROR: when the connection has been closed for any other reason.\n */ /**\n * A contract providing different interaction models per the [ReactiveSocket protocol]\n * (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).\n */ /**\n * A single unit of data exchanged between the peers of a \\`ReactiveSocket\\`.\n */");
}
#[test]
fn test_try_js_semifalse_format_1_00319d90() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Comment 1\ntry { // Comment 2\n  // Comment 3\n}\n// Comment 4\ncatch(e) { // Comment 5\n  // Comment 6\n}\n// Comment 7\nfinally { // Comment 8\n  // Comment 9\n}\n// Comment 10") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Comment 1\ntry {\n  // Comment 2\n  // Comment 3\n} catch (e) {\n  // Comment 4\n  // Comment 5\n  // Comment 6\n} finally {\n  // Comment 7\n  // Comment 8\n  // Comment 9\n}\n// Comment 10");
}
#[test]
fn test_try_js_format_1_00319d90() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// Comment 1\ntry { // Comment 2\n  // Comment 3\n}\n// Comment 4\ncatch(e) { // Comment 5\n  // Comment 6\n}\n// Comment 7\nfinally { // Comment 8\n  // Comment 9\n}\n// Comment 10") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// Comment 1\ntry {\n  // Comment 2\n  // Comment 3\n} catch (e) {\n  // Comment 4\n  // Comment 5\n  // Comment 6\n} finally {\n  // Comment 7\n  // Comment 8\n  // Comment 9\n}\n// Comment 10");
}
#[test]
fn test_tuple_and_record_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_semifalse_format_1_35f893b3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let object = // Comment\n{\n  key: 'value'\n}\n\nlet record = // Comment\n#{\n  key: 'value'\n}\n\nlet array = // Comment\n[\n  'value'\n]\n\nlet tuple = // Comment\n#[\n  'value'\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let object =\n  // Comment\n  {\n    key: \"value\",\n  }\n\nlet record =\n  // Comment\n  #{\n    key: \"value\",\n  }\n\nlet array =\n  // Comment\n  [\"value\"]\n\nlet tuple =\n  // Comment\n  #[\"value\"]");
}
#[test]
fn test_tuple_and_record_js_format_1_35f893b3() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let object = // Comment\n{\n  key: 'value'\n}\n\nlet record = // Comment\n#{\n  key: 'value'\n}\n\nlet array = // Comment\n[\n  'value'\n]\n\nlet tuple = // Comment\n#[\n  'value'\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let object =\n  // Comment\n  {\n    key: \"value\",\n  };\n\nlet record =\n  // Comment\n  #{\n    key: \"value\",\n  };\n\nlet array =\n  // Comment\n  [\"value\"];\n\nlet tuple =\n  // Comment\n  #[\"value\"];");
}
#[test]
fn test_variable_declarator_js_semifalse_format_1_01a90f44() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let obj1 = // Comment\n{\n  key: 'val'\n}\n\nlet obj2 // Comment\n= {\n  key: 'val'\n}\n\nlet obj3 = { // Comment\n  key: 'val'\n}\n\nlet obj4 = {\n  // Comment\n  key: 'val'\n}\n\nlet obj5 = // Comment\n[\n  'val'\n]\n\nlet obj6 // Comment\n= [\n  'val'\n]\n\nlet obj7 = [ // Comment\n  'val'\n]\n\nlet obj8 = [\n  // Comment\n  'val'\n]\n\nlet obj9 = // Comment\n\\`val\\`;\n\nlet obj10 = // Comment\n\\`\nval\nval\n\\`;\n\nlet obj11 = // Comment\ntag\\`val\\`;\n\nlet obj12 = // Comment\ntag\\`\nval\nval\n\\`;\n\nlet // Comment\n  foo1 = 'val';\n\nlet // Comment\n  foo2 = 'val',\n  bar = 'val';\n\nconst foo3 = 123\n// Nothing to see here.\n;[\"2\", \"3\"].forEach(x => console.log(x))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let obj1 =\n  // Comment\n  {\n    key: \"val\",\n  }\n\nlet obj2 =\n  // Comment\n  {\n    key: \"val\",\n  }\n\nlet obj3 = {\n  // Comment\n  key: \"val\",\n}\n\nlet obj4 = {\n  // Comment\n  key: \"val\",\n}\n\nlet obj5 =\n  // Comment\n  [\"val\"]\n\nlet obj6 =\n  // Comment\n  [\"val\"]\n\nlet obj7 = [\n  // Comment\n  \"val\",\n]\n\nlet obj8 = [\n  // Comment\n  \"val\",\n]\n\nlet obj9 =\n  // Comment\n  \\`val\\`\n\nlet obj10 =\n  // Comment\n  \\`\nval\nval\n\\`\n\nlet obj11 =\n  // Comment\n  tag\\`val\\`\n\nlet obj12 =\n  // Comment\n  tag\\`\nval\nval\n\\`\n\nlet // Comment\n  foo1 = \"val\"\n\nlet // Comment\n  foo2 = \"val\",\n  bar = \"val\"\n\nconst foo3 = 123\n// Nothing to see here.\n;[\"2\", \"3\"].forEach((x) => console.log(x))");
}
#[test]
fn test_variable_declarator_js_format_1_01a90f44() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("let obj1 = // Comment\n{\n  key: 'val'\n}\n\nlet obj2 // Comment\n= {\n  key: 'val'\n}\n\nlet obj3 = { // Comment\n  key: 'val'\n}\n\nlet obj4 = {\n  // Comment\n  key: 'val'\n}\n\nlet obj5 = // Comment\n[\n  'val'\n]\n\nlet obj6 // Comment\n= [\n  'val'\n]\n\nlet obj7 = [ // Comment\n  'val'\n]\n\nlet obj8 = [\n  // Comment\n  'val'\n]\n\nlet obj9 = // Comment\n\\`val\\`;\n\nlet obj10 = // Comment\n\\`\nval\nval\n\\`;\n\nlet obj11 = // Comment\ntag\\`val\\`;\n\nlet obj12 = // Comment\ntag\\`\nval\nval\n\\`;\n\nlet // Comment\n  foo1 = 'val';\n\nlet // Comment\n  foo2 = 'val',\n  bar = 'val';\n\nconst foo3 = 123\n// Nothing to see here.\n;[\"2\", \"3\"].forEach(x => console.log(x))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let obj1 =\n  // Comment\n  {\n    key: \"val\",\n  };\n\nlet obj2 =\n  // Comment\n  {\n    key: \"val\",\n  };\n\nlet obj3 = {\n  // Comment\n  key: \"val\",\n};\n\nlet obj4 = {\n  // Comment\n  key: \"val\",\n};\n\nlet obj5 =\n  // Comment\n  [\"val\"];\n\nlet obj6 =\n  // Comment\n  [\"val\"];\n\nlet obj7 = [\n  // Comment\n  \"val\",\n];\n\nlet obj8 = [\n  // Comment\n  \"val\",\n];\n\nlet obj9 =\n  // Comment\n  \\`val\\`;\n\nlet obj10 =\n  // Comment\n  \\`\nval\nval\n\\`;\n\nlet obj11 =\n  // Comment\n  tag\\`val\\`;\n\nlet obj12 =\n  // Comment\n  tag\\`\nval\nval\n\\`;\n\nlet // Comment\n  foo1 = \"val\";\n\nlet // Comment\n  foo2 = \"val\",\n  bar = \"val\";\n\nconst foo3 = 123;\n// Nothing to see here.\n[\"2\", \"3\"].forEach((x) => console.log(x));");
}
#[test]
fn test_while_js_semifalse_format_1_58dc3c9b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("while(\n    true\n    // Comment\n  ) {}\n\nwhile(true)// Comment\n{}\n\nwhile(true){}// Comment\n\nwhile(true)/*Comment*/{}\n\nwhile(\n  true // Comment\n  && true // Comment\n  ){}\n\nwhile(true) {} // comment\n\nwhile(true) /* comment */ ++x; \n\nwhile(1) // Comment\n  foo();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "while (\n  true\n  // Comment\n) {}\n\nwhile (true) {\n  // Comment\n}\n\nwhile (true) {} // Comment\n\nwhile (true) {\n  /*Comment*/\n}\n\nwhile (\n  true && // Comment\n  true // Comment\n) {}\n\nwhile (true) {} // comment\n\nwhile (true) /* comment */ ++x\n\nwhile (1)\n  // Comment\n  foo()");
}
#[test]
fn test_while_js_format_1_58dc3c9b() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("while(\n    true\n    // Comment\n  ) {}\n\nwhile(true)// Comment\n{}\n\nwhile(true){}// Comment\n\nwhile(true)/*Comment*/{}\n\nwhile(\n  true // Comment\n  && true // Comment\n  ){}\n\nwhile(true) {} // comment\n\nwhile(true) /* comment */ ++x; \n\nwhile(1) // Comment\n  foo();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "while (\n  true\n  // Comment\n) {}\n\nwhile (true) {\n  // Comment\n}\n\nwhile (true) {} // Comment\n\nwhile (true) {\n  /*Comment*/\n}\n\nwhile (\n  true && // Comment\n  true // Comment\n) {}\n\nwhile (true) {} // comment\n\nwhile (true) /* comment */ ++x;\n\nwhile (1)\n  // Comment\n  foo();");
}
