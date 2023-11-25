#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_binary_expr_js_format_1_065b7ebc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("var a = b || /** @type {string} */\n    (c);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "var a = b || /** @type {string} */ (c);");
}
#[test]
fn test_closure_compiler_type_cast_js_format_1_fd59fbad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// test to make sure comments are attached correctly\nlet inlineComment = /* some comment */ (\n  someReallyLongFunctionCall(withLots, ofArguments));\n\nlet object = {\n  key: /* some comment */ (someReallyLongFunctionCall(withLots, ofArguments))\n};\n\n// preserve parens only for type casts\nlet assignment = /** @type {string} */ (getValue());\nlet value = /** @type {string} */ (this.members[0]).functionCall();\n\nfunctionCall(1 + /** @type {string} */ (value), /** @type {!Foo} */ ({}));\n\nfunction returnValue() {\n  return /** @type {!Array.<string>} */ (['hello', 'you']);\n}\n\n// Only numberOrString is typecast\nvar newArray = /** @type {array} */ (numberOrString).map(x => x);\nvar newArray = /** @type {array} */ ((numberOrString)).map(x => x);\nvar newArray = test(/** @type {array} */ (numberOrString).map(x => x));\nvar newArray = test(/** @type {array} */ ((numberOrString)).map(x => x));\n\n// The numberOrString.map CallExpression is typecast\nvar newArray = /** @type {array} */ (numberOrString.map(x => x));\nvar newArray = /** @type {array} */ ((numberOrString).map(x => x));\nvar newArray = test(/** @type {array} */ (numberOrString.map(x => x)));\nvar newArray = test(/** @type {array} */ ((numberOrString).map(x => x)));\n\ntest(/** @type {number} */(num) + 1);\ntest(/** @type {!Array} */(arrOrString).length + 1);\ntest(/** @type {!Array} */((arrOrString)).length + 1);\n\nconst data = functionCall(\n  arg1,\n  arg2,\n  /** @type {{height: number, width: number}} */ (arg3));\n\nconst style = /** @type {{\n  width: number,\n  height: number,\n  marginTop: number,\n  marginLeft: number,\n  marginRight: number,\n  marginBottom: number,\n}} */ ({\n  width,\n  height,\n  ...margins,\n});\n\nconst style2 =/**\n * @type {{\n *   width: number,\n * }}\n*/({\n  width,\n});\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// test to make sure comments are attached correctly\nlet inlineComment = /* some comment */ someReallyLongFunctionCall(\n  withLots,\n  ofArguments,\n);\n\nlet object = {\n  key: /* some comment */ someReallyLongFunctionCall(withLots, ofArguments),\n};\n\n// preserve parens only for type casts\nlet assignment = /** @type {string} */ (getValue());\nlet value = /** @type {string} */ (this.members[0]).functionCall();\n\nfunctionCall(1 + /** @type {string} */ (value), /** @type {!Foo} */ ({}));\n\nfunction returnValue() {\n  return /** @type {!Array.<string>} */ ([\"hello\", \"you\"]);\n}\n\n// Only numberOrString is typecast\nvar newArray = /** @type {array} */ (numberOrString).map((x) => x);\nvar newArray = /** @type {array} */ (numberOrString).map((x) => x);\nvar newArray = test(/** @type {array} */ (numberOrString).map((x) => x));\nvar newArray = test(/** @type {array} */ (numberOrString).map((x) => x));\n\n// The numberOrString.map CallExpression is typecast\nvar newArray = /** @type {array} */ (numberOrString.map((x) => x));\nvar newArray = /** @type {array} */ (numberOrString.map((x) => x));\nvar newArray = test(/** @type {array} */ (numberOrString.map((x) => x)));\nvar newArray = test(/** @type {array} */ (numberOrString.map((x) => x)));\n\ntest(/** @type {number} */ (num) + 1);\ntest(/** @type {!Array} */ (arrOrString).length + 1);\ntest(/** @type {!Array} */ (arrOrString).length + 1);\n\nconst data = functionCall(\n  arg1,\n  arg2,\n  /** @type {{height: number, width: number}} */ (arg3),\n);\n\nconst style = /** @type {{\n  width: number,\n  height: number,\n  marginTop: number,\n  marginLeft: number,\n  marginRight: number,\n  marginBottom: number,\n}} */ ({\n  width,\n  height,\n  ...margins,\n});\n\nconst style2 = /**\n * @type {{\n *   width: number,\n * }}\n */ ({\n  width,\n});");
}
#[test]
fn test_comment_in_the_middle_js_format_1_45a7060b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var a =\n/**\n * bla bla bla\n * @type {string |\n  * number\n * }\n* bla bla bla\n */\n//2\n ((window['s'])).toString();\nconsole.log(a.foo());") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var a =\n  /**\n   * bla bla bla\n   * @type {string |\n   * number\n   * }\n   * bla bla bla\n   */\n  //2\n  (window[\"s\"]).toString();\nconsole.log(a.foo());");
}
#[test]
fn test_comment_placement_js_format_1_fcccaeef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo1 = /** @type {string} */\n  (value);\n\nconst foo2 =\n  /** @type {string} */\n  (value);\n\nconst foo3 =\n\n  /** @type {string} */\n  (value);\n\n\nconst foo4 =\n  /** @type {string} */(value);\n\nconst foo5 =\n  /** @type {string} */ (\n    value\n  );") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo1 = /** @type {string} */ (value);\n\nconst foo2 =\n  /** @type {string} */\n  (value);\n\nconst foo3 =\n  /** @type {string} */\n  (value);\n\nconst foo4 = /** @type {string} */ (value);\n\nconst foo5 = /** @type {string} */ (value);");
}
#[test]
fn test_extra_spaces_and_asterisks_js_format_1_c40480eb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const foo1 = /** @type {!Foo} */(bar);\nconst foo2 = /** @type {!Foo} **/(bar);\nconst foo3 = /** @type {!Foo} * */(bar);\nconst foo4 = /** @type {!Foo} ***/(bar);\nconst foo5 = /** @type {!Foo} * * */(bar);\nconst foo6 = /** @type {!Foo} *****/(bar);\nconst foo7 = /** @type {!Foo} *   *   *   *   */(bar);\nconst foo8 = /** @type {!Foo}    ** *   *   */(bar);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const foo1 = /** @type {!Foo} */ (bar);\nconst foo2 = /** @type {!Foo} **/ (bar);\nconst foo3 = /** @type {!Foo} * */ (bar);\nconst foo4 = /** @type {!Foo} ***/ (bar);\nconst foo5 = /** @type {!Foo} * * */ (bar);\nconst foo6 = /** @type {!Foo} *****/ (bar);\nconst foo7 = /** @type {!Foo} *   *   *   *   */ (bar);\nconst foo8 = /** @type {!Foo}    ** *   *   */ (bar);");
}
#[test]
fn test_iife_js_format_1_47ae1e98() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const helpers1 = /** @type {Helpers} */ ((\n  (helpers = {}) => helpers\n)());\n\nconst helpers2 = /** @type {Helpers} */ ((\n  function() { return something }\n)());\n\n// TODO: @param is misplaced https://github.com/prettier/prettier/issues/5850\nconst helpers = /** @type {Helpers} */ ((\n  /** @param {Partial<Helpers>} helpers */\n  (helpers = {}) => helpers\n)());") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const helpers1 = /** @type {Helpers} */ (((helpers = {}) => helpers)());\n\nconst helpers2 = /** @type {Helpers} */ (\n  (function () {\n    return something;\n  })()\n);\n\n// TODO: @param is misplaced https://github.com/prettier/prettier/issues/5850\nconst helpers = /** @type {Helpers} */ (\n  /** @param {Partial<Helpers>} helpers */\n  ((helpers = {}) => helpers)()\n);");
}
#[test]
fn test_iife_issue_5850_isolated_js_format_1_17b71495() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const a = /** @param {*} b */\n((b) => {})();");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const a = /** @param {*} b */ ((b) => {})();");
}
#[test]
fn test_issue_4124_js_format_1_0ced4ce7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/** @type {Object} */(myObject.property).someProp = true;\n(/** @type {Object} */(myObject.property)).someProp = true;\n\nconst prop = /** @type {Object} */(myObject.property).someProp;\n\nconst test = /** @type (function (*): ?|undefined) */\n      (goog.partial(NewThing.onTemplateChange, rationaleField, typeField));\n\nconst test = /** @type (function (*): ?|undefined) */ (goog.partial(NewThing.onTemplateChange, rationaleField, typeField));\n\nconst model = /** @type {?{getIndex: Function}} */ (model);\n\nconst foo = /** @type {string} */\n  (bar);\n\nconst test = /** @type (function (*): ?|undefined) */ (foo);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/** @type {Object} */ (myObject.property).someProp = true;\n/** @type {Object} */ (myObject.property).someProp = true;\n\nconst prop = /** @type {Object} */ (myObject.property).someProp;\n\nconst test =\n  /** @type (function (*): ?|undefined) */\n  (goog.partial(NewThing.onTemplateChange, rationaleField, typeField));\n\nconst test = /** @type (function (*): ?|undefined) */ (\n  goog.partial(NewThing.onTemplateChange, rationaleField, typeField)\n);\n\nconst model = /** @type {?{getIndex: Function}} */ (model);\n\nconst foo = /** @type {string} */ (bar);\n\nconst test = /** @type (function (*): ?|undefined) */ (foo);");
}
#[test]
fn test_issue_8045_js_format_1_7917e8ef() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (fooBarBaz);\n}\n\nconst myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n      (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return (/** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n    (fooBarBaz));\n}\n\nconst myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n      (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return (/** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n    (fooBarBaz));\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const myLongVariableName =\n  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (\n    fooBarBaz\n  );\n}\n\nconst myLongVariableName =\n  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n  (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return (\n    /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n    (fooBarBaz)\n  );\n}\n\nconst myLongVariableName =\n  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n  (fooBarBaz);\n\nfunction jsdocCastInReturn() {\n  return (\n    /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */\n    (fooBarBaz)\n  );\n}");
}
#[test]
fn test_issue_9358_js_format_1_1c72b763() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const fooooba1 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (fooobaarbazzItems || foo);\nconst fooooba2 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (fooobaarbazzItems + foo);\nconst fooooba3 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (fooobaarbazzItems || foo) ? foo : bar;\nconst fooooba4 = /** @type {Array.<fooo.barr.baaaaaaz>} */\n  (fooobaarbazzItems || foo) ? foo : bar;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const fooooba1 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (\n  fooobaarbazzItems || foo\n);\nconst fooooba2 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (\n  fooobaarbazzItems + foo\n);\nconst fooooba3 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (\n  fooobaarbazzItems || foo\n)\n  ? foo\n  : bar;\nconst fooooba4 =\n  /** @type {Array.<fooo.barr.baaaaaaz>} */\n  (fooobaarbazzItems || foo) ? foo : bar;");
}
#[test]
fn test_member_js_format_1_3cf6aa14() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("foo = (/** @type {!Baz} */ (baz).bar);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "foo = /** @type {!Baz} */ (baz).bar;");
}
#[test]
fn test_nested_js_format_1_73971729() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = /** @type {!Foo} */ (/** @type {!Baz} */ (baz).bar );\n\nconst BarImpl = /** @type {BarConstructor} */ (\n\t/** @type {unknown} */\n\t(function Bar() {\n\t\tthrow new Error(\"Internal error: Illegal constructor\");\n\t})\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = /** @type {!Foo} */ (/** @type {!Baz} */ (baz).bar);\n\nconst BarImpl = /** @type {BarConstructor} */ (\n  /** @type {unknown} */\n  (\n    function Bar() {\n      throw new Error(\"Internal error: Illegal constructor\");\n    }\n  )\n);");
}
#[test]
fn test_non_casts_js_format_1_71f8336e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @type { } */\nz(x => {\n  (foo)((bar)(2+(3)))\n  return (1);\n})\n\n/** @type { } */\nz(x => {\n  (foo)((bar)(2+(3)))\n  return (1);\n})\n\n/** @type {number} */\nlet q = z(x => {\n  return (1);\n})\n\nconst w1 = /** @typefoo Foo */ (value);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @type { } */\nz((x) => {\n  foo(bar(2 + 3));\n  return 1;\n});\n\n/** @type { } */\nz((x) => {\n  foo(bar(2 + 3));\n  return 1;\n});\n\n/** @type {number} */\nlet q = z((x) => {\n  return 1;\n});\n\nconst w1 = /** @typefoo Foo */ value;");
}
#[test]
fn test_object_with_comment_js_format_1_b21e8bf4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const objectWithComment = /** @type MyType */ (\n  /* comment */\n  {\n    foo: bar\n  }\n);\n\nconst objectWithComment2 = /** @type MyType */ (  /* comment */  {\n    foo: bar\n  }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const objectWithComment = /** @type MyType */ (\n  /* comment */\n  {\n    foo: bar,\n  }\n);\n\nconst objectWithComment2 = /** @type MyType */ (\n  /* comment */ {\n    foo: bar,\n  }\n);");
}
#[test]
fn test_satisfies_js_format_1_1d721841() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "module.exports = /** @satisfies {Record<string, string>} */ ({\n  hello: 1337,\n});",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "module.exports = /** @satisfies {Record<string, string>} */ ({\n  hello: 1337,\n});"
    );
}
#[test]
fn test_styled_components_js_format_1_e03d01c7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const OverlapWrapper =\n  /** @type {import('styled-components').ThemedStyledFunction<'div',null,{overlap: boolean}>} */\n  (styled.div)\\`\nposition:relative;\n    > {\n  position: absolute;\n  bottom: \\${p => p.overlap === 'previous' && 0};\ntop: \\${p => p.overlap === 'next' && 0};\n}\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const OverlapWrapper =\n  /** @type {import('styled-components').ThemedStyledFunction<'div',null,{overlap: boolean}>} */\n  (styled.div)\\`\n    position: relative;\n    > {\n      position: absolute;\n      bottom: \\${(p) => p.overlap === \"previous\" && 0};\n      top: \\${(p) => p.overlap === \"next\" && 0};\n    }\n  \\`;");
}
#[test]
fn test_superclass_js_format_1_f81032f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("class Foo extends /** @type {string} */ (Bar) {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "class Foo extends /** @type {string} */ (Bar) {}"
    );
}
#[test]
fn test_tuple_and_record_js_format_1_73c2f7e2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = /** @type A */ ({\n  // comment\n  width,\n  height,\n  ...margins,\n});\n\nfoo = /** @type A */ (#{\n  // comment\n  width,\n  height,\n  ...margins,\n});\n\nfoo = /** @type A */ ([\n  // comment\n  width,\n  height,\n  ...margins,\n]);\n\nfoo = /** @type A */ (#[\n  // comment\n  width,\n  height,\n  ...margins,\n]);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = /** @type A */ ({\n  // comment\n  width,\n  height,\n  ...margins,\n});\n\nfoo = /** @type A */ (#{\n  // comment\n  width,\n  height,\n  ...margins,\n});\n\nfoo = /** @type A */ ([\n  // comment\n  width,\n  height,\n  ...margins,\n]);\n\nfoo = /** @type A */ (#[\n  // comment\n  width,\n  height,\n  ...margins,\n]);");
}
#[test]
fn test_ways_to_specify_type_js_format_1_44477d5d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const curlyBraces = /** @type {string} */ (foo);\nconst curlyBraces2 = /**@type {string} */ (foo);\nconst noWhitespace = /** @type{string} */ (foo);\nconst noWhitespace2 = /**@type{string} */ (foo);\nconst noBraces = /** @type string */ (foo);\nconst parens = /** @type (string | number) */ (foo);\n\n// Prettier just searches for \"@type\" and doesn't check the syntax of types.\nconst v1 = /** @type {} */ (value);\nconst v2 = /** @type {}} */ (value);\nconst v3 = /** @type } */ (value);\nconst v4 = /** @type { */ (value);\nconst v5 = /** @type {{} */ (value);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const curlyBraces = /** @type {string} */ (foo);\nconst curlyBraces2 = /**@type {string} */ (foo);\nconst noWhitespace = /** @type{string} */ (foo);\nconst noWhitespace2 = /**@type{string} */ (foo);\nconst noBraces = /** @type string */ (foo);\nconst parens = /** @type (string | number) */ (foo);\n\n// Prettier just searches for \"@type\" and doesn't check the syntax of types.\nconst v1 = /** @type {} */ (value);\nconst v2 = /** @type {}} */ (value);\nconst v3 = /** @type } */ (value);\nconst v4 = /** @type { */ (value);\nconst v5 = /** @type {{} */ (value);");
}
