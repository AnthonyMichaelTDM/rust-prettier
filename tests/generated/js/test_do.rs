#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_call_arguments_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_call_arguments_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_call_arguments_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_call_arguments_js_format_1_8fd54454() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// from https://github.com/babel/babel/pull/13122/\nexpect(\n  do {\n    var bar = \"foo\";\n    if (!bar) throw new Error(\n      \"unreachable\"\n    )\n    bar;\n  }\n).toBe(\"foo\");\nexpect(bar).toBe(\"foo\");\n\nvar x = do {\n  var bar = \"foo\";\n  if (!bar) throw new Error(\n    \"unreachable\"\n  )\n  bar;\n};\n\nexpect(\n  do {\n    var bar = \"foo\";\n    bar;\n  }\n).toBe(\"foo\");\nexpect(bar).toBe(\"foo\");\n\nvar x = do {\n  var bar = \"foo\";\n  bar;\n};\n\nexpect(\n  () => do {\n    () => {\n      var bar = \"foo\";\n    };\n    bar;\n  }\n).toThrow(ReferenceError);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// from https://github.com/babel/babel/pull/13122/\nexpect(do {\n  var bar = \"foo\";\n  if (!bar) throw new Error(\"unreachable\");\n  bar;\n}).toBe(\"foo\");\nexpect(bar).toBe(\"foo\");\n\nvar x = do {\n  var bar = \"foo\";\n  if (!bar) throw new Error(\"unreachable\");\n  bar;\n};\n\nexpect(do {\n  var bar = \"foo\";\n  bar;\n}).toBe(\"foo\");\nexpect(bar).toBe(\"foo\");\n\nvar x = do {\n  var bar = \"foo\";\n  bar;\n};\n\nexpect(\n  () => do {\n    () => {\n      var bar = \"foo\";\n    };\n    bar;\n  },\n).toThrow(ReferenceError);");
}
#[test]
fn test_do_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_do_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_do_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_do_js_format_1_cbacd289() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const envSpecific = {\n  domain:\n    do {\n      if(env === 'production') 'https://abc.mno.com/';\n      else if(env === 'development') 'http://localhost:4000';\n    }\n};\n\nlet x = do {\n  let tmp = f();\n  tmp * tmp + 1\n};\n\nlet y = do {\n  if (foo()) { f() }\n  else if (bar()) { g() }\n  else { h() }\n};\n\nfunction foo() {\n  return (\n    <nav>\n      <Home />\n      {\n        do {\n          if (loggedIn) {\n            <LogoutButton />\n          } else {\n            <LoginButton />\n          }\n        }\n      }\n    </nav>\n  );\n}\n\n(do {});\n(do {} + 1);\n(1 + do {});\n() => do {};\n\n(do {\n  switch(0) {\n    case 0: \"foo\";\n    case 1: break;\n  }\n});\n\n() => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};\n\n() => (    ) => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};\n\na =>b=>     c => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const envSpecific = {\n  domain: do {\n    if (env === \"production\") \"https://abc.mno.com/\";\n    else if (env === \"development\") \"http://localhost:4000\";\n  },\n};\n\nlet x = do {\n  let tmp = f();\n  tmp * tmp + 1;\n};\n\nlet y = do {\n  if (foo()) {\n    f();\n  } else if (bar()) {\n    g();\n  } else {\n    h();\n  }\n};\n\nfunction foo() {\n  return (\n    <nav>\n      <Home />\n      {do {\n        if (loggedIn) {\n          <LogoutButton />;\n        } else {\n          <LoginButton />;\n        }\n      }}\n    </nav>\n  );\n}\n\n(do {});\n(do {}) + 1;\n1 + do {};\n() => do {};\n\n(do {\n  switch (0) {\n    case 0:\n      \"foo\";\n    case 1:\n      break;\n  }\n});\n\n() => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};\n\n() => () => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};\n\n(a) => (b) => (c) => do {\n  var obj = { foo: \"bar\", bar: \"foo\" };\n  for (var key in obj) {\n    obj[key];\n  }\n};");
}
