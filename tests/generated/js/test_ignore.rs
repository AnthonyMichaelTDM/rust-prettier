#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_class_expression_decorator_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_class_expression_decorator_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_class_expression_decorator_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_class_expression_decorator_js_format_1_9190b171() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("(\n  // prettier-ignore\n  @decorator\n  class {}\n);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// prettier-ignore\n(\n  @decorator\n  class {}\n);"
    );
}
#[test]
fn test_decorator_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_decorator_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_decorator_js_format_1_862c2a84() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("// prettier-ignore\n@decorator\nclass A {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "// prettier-ignore\n@decorator\nclass A {}");
}
#[test]
fn test_ignore_js_format_1_4dae6a4f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function a() {\n  // prettier-ignore\n  var fnString =\n    '\"' + this.USE + ' ' + this.STRICT + '\";\\\\n' +\n    this.filterPrefix() +\n    'var fn=' + this.generateFunction('fn', 's,l,a,i') +\n    extra +\n    this.watchFns() +\n    'return fn;';\n\n  // prettier-ignore\n  const identity = Matrix.create(\n    1, 0, 0,\n    0, 1, 0,\n    0, 0, 0\n  );\n\n  // Let's make sure that this comment doesn't interfere\n\n  // prettier-ignore\n  const commentsWithPrettierIgnore =   {\n    \"ewww\":\n            \"gross-formatting\",\n  };\n\n  function giveMeSome() {\n    a(  a  ); // prettier-ignore\n    // shouldn't I return something?  :shrug:\n  }\n\n  // prettier-ignore\n  console.error(\n    'In order to use ' + prompt + ', you need to configure a ' +\n    'few environment variables to be able to commit to the ' +\n    'repository. Follow those steps to get you setup:\\\\n' +\n    '\\\\n' +\n    'Go to https://github.com/settings/tokens/new\\\\n' +\n    ' - Fill \"Token description\" with \"' + prompt + ' for ' +\n      repoSlug + '\"\\\\n' +\n    ' - Check \"public_repo\"\\\\n' +\n    ' - Press \"Generate Token\"\\\\n' +\n    '\\\\n' +\n    'In a different tab, go to https://travis-ci.org/' +\n      repoSlug + '/settings\\\\n' +\n    ' - Make sure \"Build only if .travis.yml is present\" is ON\\\\n' +\n    ' - Fill \"Name\" with \"GITHUB_USER\" and \"Value\" with the name of the ' +\n      'account you generated the token with. Press \"Add\"\\\\n' +\n    '\\\\n' +\n    'Once this is done, commit anything to the repository to restart ' +\n      'Travis and it should work :)'\n  );\n}\n\nconst response = {\n  // prettier-ignore\n  '_text': 'Turn on the lights',\n  intent: 'lights',\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function a() {\n  // prettier-ignore\n  var fnString =\n    '\"' + this.USE + ' ' + this.STRICT + '\";\\\\n' +\n    this.filterPrefix() +\n    'var fn=' + this.generateFunction('fn', 's,l,a,i') +\n    extra +\n    this.watchFns() +\n    'return fn;';\n\n  // prettier-ignore\n  const identity = Matrix.create(\n    1, 0, 0,\n    0, 1, 0,\n    0, 0, 0\n  );\n\n  // Let's make sure that this comment doesn't interfere\n\n  // prettier-ignore\n  const commentsWithPrettierIgnore =   {\n    \"ewww\":\n            \"gross-formatting\",\n  };\n\n  function giveMeSome() {\n    a(  a  ); // prettier-ignore\n    // shouldn't I return something?  :shrug:\n  }\n\n  // prettier-ignore\n  console.error(\n    'In order to use ' + prompt + ', you need to configure a ' +\n    'few environment variables to be able to commit to the ' +\n    'repository. Follow those steps to get you setup:\\\\n' +\n    '\\\\n' +\n    'Go to https://github.com/settings/tokens/new\\\\n' +\n    ' - Fill \"Token description\" with \"' + prompt + ' for ' +\n      repoSlug + '\"\\\\n' +\n    ' - Check \"public_repo\"\\\\n' +\n    ' - Press \"Generate Token\"\\\\n' +\n    '\\\\n' +\n    'In a different tab, go to https://travis-ci.org/' +\n      repoSlug + '/settings\\\\n' +\n    ' - Make sure \"Build only if .travis.yml is present\" is ON\\\\n' +\n    ' - Fill \"Name\" with \"GITHUB_USER\" and \"Value\" with the name of the ' +\n      'account you generated the token with. Press \"Add\"\\\\n' +\n    '\\\\n' +\n    'Once this is done, commit anything to the repository to restart ' +\n      'Travis and it should work :)'\n  );\n}\n\nconst response = {\n  // prettier-ignore\n  '_text': 'Turn on the lights',\n  intent: \"lights\",\n};");
}
#[test]
fn test_ignore_2_js_format_1_e2b0eaac() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("// #8736\n\nfunction HelloWorld() {\n  return (\n    <div\n      {...{} /*\n      // @ts-ignore */ /* prettier-ignore */}\n      invalidProp=\"HelloWorld\"\n    >\n      test\n    </div>\n  );\n}\n\na = <div {.../* prettier-ignore */b}/>\na = <div {...b/* prettier-ignore */}/>\na = <div {.../* prettier-ignore */{}}/>\na = <div {...{/* prettier-ignore */}}/>\na = <div {...{}/* prettier-ignore */}/>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// #8736\n\nfunction HelloWorld() {\n  return (\n    <div\n      {...{} /*\n      // @ts-ignore */ /* prettier-ignore */}\n      invalidProp=\"HelloWorld\"\n    >\n      test\n    </div>\n  );\n}\n\na = <div {.../* prettier-ignore */ b} />;\na = <div {...b /* prettier-ignore */} />;\na = <div {.../* prettier-ignore */ {}} />;\na = <div {...{/* prettier-ignore */}} />;\na = <div {...{} /* prettier-ignore */} />;");
}
#[test]
fn test_issue_9335_js_format_1_26d4dc4c() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("class foo extends\n  // prettier-ignore\n  f {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "class foo\n  // prettier-ignore\n  extends f {}");
}
#[test]
fn test_issue_9877_js_format_1_3e7b9730() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export default function test() {\n  return {\n    matrix: // prettier-ignore\n      new Float32Array([\n      0, 0,\n      1, 0,\n      1, 1,\n      0, 1\n    ]),\n  };\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function test() {\n  return {\n    matrix: // prettier-ignore\n      new Float32Array([\n      0, 0,\n      1, 0,\n      1, 1,\n      0, 1\n    ]),\n  };\n}");
}
#[test]
fn test_issue_10661_js_format_1_e510d18a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("verylongidentifierthatwillwrap123123123123123(\n  a.b\n    // prettier-ignore\n    // Some other comment here\n    .c\n);\n\ncall(\n  // comment\n  a.\n    // prettier-ignore\n    b\n)\n\ncall(\n  a(\n/*\nthis won't get formatted too,\nbecause the prettier-ignore comment is attached as MemberExpression leading comment\n*/\n1,\n2.0000, 3\n)\n    // prettier-ignore\n    .c\n)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "verylongidentifierthatwillwrap123123123123123(\n  a.b\n    // prettier-ignore\n    // Some other comment here\n    .c,\n);\n\ncall(\n  // comment\n  a.\n    // prettier-ignore\n    b,\n);\n\ncall(\n  a(\n/*\nthis won't get formatted too,\nbecause the prettier-ignore comment is attached as MemberExpression leading comment\n*/\n1,\n2.0000, 3\n)\n    // prettier-ignore\n    .c,\n);");
}
#[test]
fn test_issue_11077_js_format_1_e1f8e1cb() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("function HelloWorld(x) {\n\n  (\n    // prettier-ignore\n    // eslint-disable-next-line\n    x.a |\n    x.b\n  ).call(null)\n\n}\n\nfunction HelloWorld(x) {\n  // prettier-ignore\n  (\n    // eslint-disable-next-line\n    x.a |\n    x.b\n  ).call(null)\n\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function HelloWorld(x) {\n  // prettier-ignore\n  // eslint-disable-next-line\n  (x.a |\n    x.b).call(null);\n}\n\nfunction HelloWorld(x) {\n  // prettier-ignore\n  (\n    // eslint-disable-next-line\n    x.a |\n    x.b\n  ).call(null)\n}");
}
#[test]
fn test_issue_13737_js_format_1_e377e60f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "oneArgument(\n // prettier-ignore\n (0, 1)\n)\n\na=(\n // prettier-ignore\n (0, 1)\n)",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "oneArgument(\n  // prettier-ignore\n  (0, 1),\n);\n\na =\n  // prettier-ignore\n  (0, 1);"
    );
}
#[test]
fn test_issue_14404_js_format_1_6d28c8bd() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format(
        "async function foo() {\n(\n  \t// prettier-ignore\n  \t// b\n\tawait thing()\n).blah\n}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "async function foo() {\n  // prettier-ignore\n  // b\n  (await thing()).blah;\n}"
    );
}
