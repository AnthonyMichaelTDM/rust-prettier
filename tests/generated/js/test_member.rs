#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_conditional_js_format_1_b28fc685() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser))\n.prop;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser)\n).prop;");
}
#[test]
fn test_expand_js_format_1_003d29b0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const veryVeryVeryVeryVeryVeryVeryLong = doc.expandedStates[doc.expandedStates.length - 1];\nconst small = doc.expandedStates[doc.expandedStates.length - 1];\n\nconst promises = [\n  promise.resolve().then(console.log).catch(err => {\n    console.log(err)\n    return null\n  }),\n  redis.fetch(),\n  other.fetch(),\n];\n\nconst promises2 = [\n  promise.resolve().veryLongFunctionCall().veryLongFunctionCall().then(console.log).catch(err => {\n    console.log(err)\n    return null\n  }),\n  redis.fetch(),\n  other.fetch(),\n];\n\nwindow.FooClient.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token\n}).initVerify(\"foo_container\");\n\nwindow.something.FooClient.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token\n}).initVerify(\"foo_container\");\n\nwindow.FooClient.something.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token\n}).initVerify(\"foo_container\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const veryVeryVeryVeryVeryVeryVeryLong =\n  doc.expandedStates[doc.expandedStates.length - 1];\nconst small = doc.expandedStates[doc.expandedStates.length - 1];\n\nconst promises = [\n  promise\n    .resolve()\n    .then(console.log)\n    .catch((err) => {\n      console.log(err);\n      return null;\n    }),\n  redis.fetch(),\n  other.fetch(),\n];\n\nconst promises2 = [\n  promise\n    .resolve()\n    .veryLongFunctionCall()\n    .veryLongFunctionCall()\n    .then(console.log)\n    .catch((err) => {\n      console.log(err);\n      return null;\n    }),\n  redis.fetch(),\n  other.fetch(),\n];\n\nwindow.FooClient.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token,\n}).initVerify(\"foo_container\");\n\nwindow.something.FooClient.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token,\n}).initVerify(\"foo_container\");\n\nwindow.FooClient.something\n  .setVars({\n    locale: getFooLocale({ page }),\n    authorizationToken: data.token,\n  })\n  .initVerify(\"foo_container\");");
}
#[test]
fn test_logical_js_format_1_232fec79() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(veryLongVeryLongVeryLong || e).prop;\n\n(veryLongVeryLongVeryLong || anotherVeryLongVeryLongVeryLong || veryVeryVeryLongError).prop;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(veryLongVeryLongVeryLong || e).prop;\n\n(\n  veryLongVeryLongVeryLong ||\n  anotherVeryLongVeryLongVeryLong ||\n  veryVeryVeryLongError\n).prop;");
}
