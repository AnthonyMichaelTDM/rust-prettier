#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_escaped_js_trailing_commaall_format_1_7da35aab() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =\n  goog.getMsg(\"That's all we know\");\n\nexport const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =\n  goog.getMsg(\"That\\\\'s all we know\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =\n  goog.getMsg(\"That's all we know\");\n\nexport const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =\n  goog.getMsg(\"That's all we know\");");
}
#[test]
fn test_escaped_js_trailing_commaes_5_format_1_7da35aab() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =\n  goog.getMsg(\"That's all we know\");\n\nexport const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =\n  goog.getMsg(\"That\\\\'s all we know\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export const MSG_GENERIC_OPERATION_FAILURE_BODY_1 =\n  goog.getMsg(\"That's all we know\");\n\nexport const MSG_GENERIC_OPERATION_FAILURE_BODY_2 =\n  goog.getMsg(\"That's all we know\");");
}
#[test]
fn test_multiline_literal_js_trailing_commaall_format_1_74bb47c4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// https://github.com/prettier/prettier/pull/13274\n\nconst loremIpsumFooBazBar1 = 'Multiline string\\\\\n         Multiline string\\\\\n'\n\nconst loremIpsumFooBazBar2 = 'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/prettier/prettier/pull/13274\n\nconst loremIpsumFooBazBar1 =\n  \"Multiline string\\\\\n         Multiline string\\\\\n\";\n\nconst loremIpsumFooBazBar2 =\n  \"Multiline string\\\\\n         Multiline string\\\\\n         Multiline string\";");
}
#[test]
fn test_multiline_literal_js_trailing_commaes_5_format_1_74bb47c4() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// https://github.com/prettier/prettier/pull/13274\n\nconst loremIpsumFooBazBar1 = 'Multiline string\\\\\n         Multiline string\\\\\n'\n\nconst loremIpsumFooBazBar2 = 'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/prettier/prettier/pull/13274\n\nconst loremIpsumFooBazBar1 =\n  \"Multiline string\\\\\n         Multiline string\\\\\n\";\n\nconst loremIpsumFooBazBar2 =\n  \"Multiline string\\\\\n         Multiline string\\\\\n         Multiline string\";");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaall_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaall_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaall_format_1_959bf89a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11852\n\n\"\\\\8\",\"\\\\9\";\n() => {\n  \"use strict\";\n  \"\\\\8\", \"\\\\9\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11852\n\n\"8\", \"9\";\n() => {\n  \"use strict\";\n  \"8\", \"9\";\n};");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaes_5_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaes_5_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_non_octal_eight_and_nine_js_trailing_commaes_5_format_1_959bf89a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11852\n\n\"\\\\8\",\"\\\\9\";\n() => {\n  \"use strict\";\n  \"\\\\8\", \"\\\\9\";\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11852\n\n\"8\", \"9\";\n() => {\n  \"use strict\";\n  \"8\", \"9\";\n};");
}
#[test]
fn test_strings_js_trailing_commaall_format_1_fa3beb71() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("[\n  \"abc\",\n  'abc',\n\n  '\\\\'',\n\n  '\"',\n  '\\\\\"',\n  '\\\\\\\\\"',\n\n  \"'\",\n  \"\\\\'\",\n  \"\\\\\\\\'\",\n\n  \"'\\\\\"\",\n  '\\\\'\"',\n\n  '\\\\\\\\',\n  \"\\\\\\\\\",\n\n  '\\\\0',\n  '🐶',\n\n  '\\\\uD801\\\\uDC28',\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"abc\",\n  \"abc\",\n\n  \"'\",\n\n  '\"',\n  '\"',\n  '\\\\\\\\\"',\n\n  \"'\",\n  \"'\",\n  \"\\\\\\\\'\",\n\n  \"'\\\\\"\",\n  \"'\\\\\"\",\n\n  \"\\\\\\\\\",\n  \"\\\\\\\\\",\n\n  \"\\\\0\",\n  \"🐶\",\n\n  \"\\\\uD801\\\\uDC28\",\n];");
}
#[test]
fn test_strings_js_trailing_commaes_5_format_1_fa3beb71() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("[\n  \"abc\",\n  'abc',\n\n  '\\\\'',\n\n  '\"',\n  '\\\\\"',\n  '\\\\\\\\\"',\n\n  \"'\",\n  \"\\\\'\",\n  \"\\\\\\\\'\",\n\n  \"'\\\\\"\",\n  '\\\\'\"',\n\n  '\\\\\\\\',\n  \"\\\\\\\\\",\n\n  '\\\\0',\n  '🐶',\n\n  '\\\\uD801\\\\uDC28',\n];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  \"abc\",\n  \"abc\",\n\n  \"'\",\n\n  '\"',\n  '\"',\n  '\\\\\\\\\"',\n\n  \"'\",\n  \"'\",\n  \"\\\\\\\\'\",\n\n  \"'\\\\\"\",\n  \"'\\\\\"\",\n\n  \"\\\\\\\\\",\n  \"\\\\\\\\\",\n\n  \"\\\\0\",\n  \"🐶\",\n\n  \"\\\\uD801\\\\uDC28\",\n];");
}
#[test]
fn test_template_literals_js_trailing_commaall_format_1_f3f169a1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("foo(\\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 } with expr\\`);\n\nconst x = \\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + ( function() {return 3 })() + 3 + 2 + 3 + 2 + 3 } with expr\\`;\n\nfoo(\\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + ( function() {\n  const x = 5;\n\n  return x;\n })() + 3 + 2 + 3 + 2 + 3 } with expr\\`);\n\npipe.write(\n  \\`\\\\n  \\${chalk.dim(\\`\\\\u203A and \\${more} more \\${more} more \\${more} more \\${more}\\`)}\\`,\n);\n\n// https://github.com/prettier/prettier/issues/1662#issue-230406820\nconst content = \\`\nconst env = \\${ JSON.stringify({\n\tassetsRootUrl: env.assetsRootUrl,\n\tenv: env.env,\n\trole: \"client\",\n\tadsfafa: \"sdfsdff\",\n\tasdfasff: \"wefwefw\",\n  \tfefef: \"sf sdfs fdsfdsf s dfsfds\"\n}, null, \"\\\\t\") });\n\\`;\n\n// https://github.com/prettier/prettier/issues/821#issue-210557749\nf(\\`\\${{\n  a: 4,\n  b: 9,\n}}\\`);\n\n// https://github.com/prettier/prettier/issues/1183#issue-220863505\nconst makeBody = (store, assets, html) =>\n  \\`<!doctype html>\\${\n    ReactDOMServer.renderToStaticMarkup(\n      <Html\n        headScripts={compact([ assets.javascript.head ])}\n        headStyles={compact([ assets.styles.body, assets.styles.head ])}\n        bodyScripts={compact([ assets.javascript.body ])}\n        bodyStyles={[]}\n        stringScripts={[\n          \\`window.__INITIAL_STATE__ = \\${\n            JSON.stringify(store.getState(), null, 2)\n          };\\`,\n        ]}\n        content={[\n          { id: 'app-container', dangerouslySetInnerHTML: { __html: html } },\n        ]}\n      />\n    )\n  }\\`\n\n// https://github.com/prettier/prettier/issues/1626#issue-229655106\nconst Bar = styled.div\\`\n  color: \\${props => (props.highlight.length > 0 ? palette(['text', 'dark', 'tertiary'])(props) : palette(['text', 'dark', 'primary'])(props))} !important;\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo(\n  \\`a long string \\${\n    1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3\n  } with expr\\`,\n);\n\nconst x = \\`a long string \\${\n  1 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  (function () {\n    return 3;\n  })() +\n  3 +\n  2 +\n  3 +\n  2 +\n  3\n} with expr\\`;\n\nfoo(\n  \\`a long string \\${\n    1 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    (function () {\n      const x = 5;\n\n      return x;\n    })() +\n    3 +\n    2 +\n    3 +\n    2 +\n    3\n  } with expr\\`,\n);\n\npipe.write(\n  \\`\\\\n  \\${chalk.dim(\n    \\`\\\\u203A and \\${more} more \\${more} more \\${more} more \\${more}\\`,\n  )}\\`,\n);\n\n// https://github.com/prettier/prettier/issues/1662#issue-230406820\nconst content = \\`\nconst env = \\${JSON.stringify(\n  {\n    assetsRootUrl: env.assetsRootUrl,\n    env: env.env,\n    role: \"client\",\n    adsfafa: \"sdfsdff\",\n    asdfasff: \"wefwefw\",\n    fefef: \"sf sdfs fdsfdsf s dfsfds\",\n  },\n  null,\n  \"\\\\t\",\n)});\n\\`;\n\n// https://github.com/prettier/prettier/issues/821#issue-210557749\nf(\n  \\`\\${{\n    a: 4,\n    b: 9,\n  }}\\`,\n);\n\n// https://github.com/prettier/prettier/issues/1183#issue-220863505\nconst makeBody = (store, assets, html) =>\n  \\`<!doctype html>\\${ReactDOMServer.renderToStaticMarkup(\n    <Html\n      headScripts={compact([assets.javascript.head])}\n      headStyles={compact([assets.styles.body, assets.styles.head])}\n      bodyScripts={compact([assets.javascript.body])}\n      bodyStyles={[]}\n      stringScripts={[\n        \\`window.__INITIAL_STATE__ = \\${JSON.stringify(\n          store.getState(),\n          null,\n          2,\n        )};\\`,\n      ]}\n      content={[\n        { id: \"app-container\", dangerouslySetInnerHTML: { __html: html } },\n      ]}\n    />,\n  )}\\`;\n\n// https://github.com/prettier/prettier/issues/1626#issue-229655106\nconst Bar = styled.div\\`\n  color: \\${(props) =>\n    props.highlight.length > 0\n      ? palette([\"text\", \"dark\", \"tertiary\"])(props)\n      : palette([\"text\", \"dark\", \"primary\"])(props)} !important;\n\\`;");
}
#[test]
fn test_template_literals_js_trailing_commaes_5_format_1_f3f169a1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("foo(\\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 } with expr\\`);\n\nconst x = \\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + ( function() {return 3 })() + 3 + 2 + 3 + 2 + 3 } with expr\\`;\n\nfoo(\\`a long string \\${ 1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + ( function() {\n  const x = 5;\n\n  return x;\n })() + 3 + 2 + 3 + 2 + 3 } with expr\\`);\n\npipe.write(\n  \\`\\\\n  \\${chalk.dim(\\`\\\\u203A and \\${more} more \\${more} more \\${more} more \\${more}\\`)}\\`,\n);\n\n// https://github.com/prettier/prettier/issues/1662#issue-230406820\nconst content = \\`\nconst env = \\${ JSON.stringify({\n\tassetsRootUrl: env.assetsRootUrl,\n\tenv: env.env,\n\trole: \"client\",\n\tadsfafa: \"sdfsdff\",\n\tasdfasff: \"wefwefw\",\n  \tfefef: \"sf sdfs fdsfdsf s dfsfds\"\n}, null, \"\\\\t\") });\n\\`;\n\n// https://github.com/prettier/prettier/issues/821#issue-210557749\nf(\\`\\${{\n  a: 4,\n  b: 9,\n}}\\`);\n\n// https://github.com/prettier/prettier/issues/1183#issue-220863505\nconst makeBody = (store, assets, html) =>\n  \\`<!doctype html>\\${\n    ReactDOMServer.renderToStaticMarkup(\n      <Html\n        headScripts={compact([ assets.javascript.head ])}\n        headStyles={compact([ assets.styles.body, assets.styles.head ])}\n        bodyScripts={compact([ assets.javascript.body ])}\n        bodyStyles={[]}\n        stringScripts={[\n          \\`window.__INITIAL_STATE__ = \\${\n            JSON.stringify(store.getState(), null, 2)\n          };\\`,\n        ]}\n        content={[\n          { id: 'app-container', dangerouslySetInnerHTML: { __html: html } },\n        ]}\n      />\n    )\n  }\\`\n\n// https://github.com/prettier/prettier/issues/1626#issue-229655106\nconst Bar = styled.div\\`\n  color: \\${props => (props.highlight.length > 0 ? palette(['text', 'dark', 'tertiary'])(props) : palette(['text', 'dark', 'primary'])(props))} !important;\n\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo(\n  \\`a long string \\${\n    1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3\n  } with expr\\`\n);\n\nconst x = \\`a long string \\${\n  1 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  3 +\n  2 +\n  (function () {\n    return 3;\n  })() +\n  3 +\n  2 +\n  3 +\n  2 +\n  3\n} with expr\\`;\n\nfoo(\n  \\`a long string \\${\n    1 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    3 +\n    2 +\n    (function () {\n      const x = 5;\n\n      return x;\n    })() +\n    3 +\n    2 +\n    3 +\n    2 +\n    3\n  } with expr\\`\n);\n\npipe.write(\n  \\`\\\\n  \\${chalk.dim(\n    \\`\\\\u203A and \\${more} more \\${more} more \\${more} more \\${more}\\`\n  )}\\`\n);\n\n// https://github.com/prettier/prettier/issues/1662#issue-230406820\nconst content = \\`\nconst env = \\${JSON.stringify(\n  {\n    assetsRootUrl: env.assetsRootUrl,\n    env: env.env,\n    role: \"client\",\n    adsfafa: \"sdfsdff\",\n    asdfasff: \"wefwefw\",\n    fefef: \"sf sdfs fdsfdsf s dfsfds\",\n  },\n  null,\n  \"\\\\t\"\n)});\n\\`;\n\n// https://github.com/prettier/prettier/issues/821#issue-210557749\nf(\n  \\`\\${{\n    a: 4,\n    b: 9,\n  }}\\`\n);\n\n// https://github.com/prettier/prettier/issues/1183#issue-220863505\nconst makeBody = (store, assets, html) =>\n  \\`<!doctype html>\\${ReactDOMServer.renderToStaticMarkup(\n    <Html\n      headScripts={compact([assets.javascript.head])}\n      headStyles={compact([assets.styles.body, assets.styles.head])}\n      bodyScripts={compact([assets.javascript.body])}\n      bodyStyles={[]}\n      stringScripts={[\n        \\`window.__INITIAL_STATE__ = \\${JSON.stringify(\n          store.getState(),\n          null,\n          2\n        )};\\`,\n      ]}\n      content={[\n        { id: \"app-container\", dangerouslySetInnerHTML: { __html: html } },\n      ]}\n    />\n  )}\\`;\n\n// https://github.com/prettier/prettier/issues/1626#issue-229655106\nconst Bar = styled.div\\`\n  color: \\${(props) =>\n    props.highlight.length > 0\n      ? palette([\"text\", \"dark\", \"tertiary\"])(props)\n      : palette([\"text\", \"dark\", \"primary\"])(props)} !important;\n\\`;");
}
