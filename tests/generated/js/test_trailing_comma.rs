#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dynamic_import_js_trailing_commaall_format_1_180fa07d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import(\n  'myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename'\n);") ? ;
    assert_eq ! (formatted , "import(\n  \"myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename\"\n);");
    Ok(())
}
#[test]
fn test_dynamic_import_js_trailing_commaes_5_format_1_180fa07d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import(\n  'myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename'\n);") ? ;
    assert_eq ! (formatted , "import(\n  \"myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename\"\n);");
    Ok(())
}
#[test]
fn test_dynamic_import_js_trailing_commanone_format_1_180fa07d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import(\n  'myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename'\n);") ? ;
    assert_eq ! (formatted , "import(\n  \"myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename\"\n);");
    Ok(())
}
#[test]
fn test_es_5_js_trailing_commaall_format_1_662437d6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to\n) {\n  send_single_email_implementation(  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to);\n  return \"nothing\";\n}") ? ;
    assert_eq ! (formatted , "function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to,\n) {\n  send_single_email_implementation(\n    app,\n    email_id,\n    email_address,\n    subject,\n    html,\n    reply_to,\n  );\n  return \"nothing\";\n}");
    Ok(())
}
#[test]
fn test_es_5_js_trailing_commaes_5_format_1_662437d6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to\n) {\n  send_single_email_implementation(  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to);\n  return \"nothing\";\n}") ? ;
    assert_eq ! (formatted , "function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to\n) {\n  send_single_email_implementation(\n    app,\n    email_id,\n    email_address,\n    subject,\n    html,\n    reply_to\n  );\n  return \"nothing\";\n}");
    Ok(())
}
#[test]
fn test_es_5_js_trailing_commanone_format_1_662437d6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to\n) {\n  send_single_email_implementation(  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to);\n  return \"nothing\";\n}") ? ;
    assert_eq ! (formatted , "function send_single_email(\n  app,\n  email_id,\n  email_address,\n  subject,\n  html,\n  reply_to\n) {\n  send_single_email_implementation(\n    app,\n    email_id,\n    email_address,\n    subject,\n    html,\n    reply_to\n  );\n  return \"nothing\";\n}");
    Ok(())
}
#[test]
fn test_function_calls_js_trailing_commaall_format_1_abcf138f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = (param1, param2, param3) => {}\n\na('value', 'value2', 'value3');\n\na(\n  'a-long-value',\n  'a-really-really-long-value',\n  'a-really-really-really-long-value',\n);\n\na('value', 'value2', a('long-nested-value', 'long-nested-value2', 'long-nested-value3'));\n\na.b().c(\n  {\n    d,\n  },\n  () => {}\n);") ? ;
    assert_eq ! (formatted , "const a = (param1, param2, param3) => {};\n\na(\"value\", \"value2\", \"value3\");\n\na(\n  \"a-long-value\",\n  \"a-really-really-long-value\",\n  \"a-really-really-really-long-value\",\n);\n\na(\n  \"value\",\n  \"value2\",\n  a(\"long-nested-value\", \"long-nested-value2\", \"long-nested-value3\"),\n);\n\na.b().c(\n  {\n    d,\n  },\n  () => {},\n);");
    Ok(())
}
#[test]
fn test_function_calls_js_trailing_commaes_5_format_1_abcf138f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = (param1, param2, param3) => {}\n\na('value', 'value2', 'value3');\n\na(\n  'a-long-value',\n  'a-really-really-long-value',\n  'a-really-really-really-long-value',\n);\n\na('value', 'value2', a('long-nested-value', 'long-nested-value2', 'long-nested-value3'));\n\na.b().c(\n  {\n    d,\n  },\n  () => {}\n);") ? ;
    assert_eq ! (formatted , "const a = (param1, param2, param3) => {};\n\na(\"value\", \"value2\", \"value3\");\n\na(\n  \"a-long-value\",\n  \"a-really-really-long-value\",\n  \"a-really-really-really-long-value\"\n);\n\na(\n  \"value\",\n  \"value2\",\n  a(\"long-nested-value\", \"long-nested-value2\", \"long-nested-value3\")\n);\n\na.b().c(\n  {\n    d,\n  },\n  () => {}\n);");
    Ok(())
}
#[test]
fn test_function_calls_js_trailing_commanone_format_1_abcf138f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = (param1, param2, param3) => {}\n\na('value', 'value2', 'value3');\n\na(\n  'a-long-value',\n  'a-really-really-long-value',\n  'a-really-really-really-long-value',\n);\n\na('value', 'value2', a('long-nested-value', 'long-nested-value2', 'long-nested-value3'));\n\na.b().c(\n  {\n    d,\n  },\n  () => {}\n);") ? ;
    assert_eq ! (formatted , "const a = (param1, param2, param3) => {};\n\na(\"value\", \"value2\", \"value3\");\n\na(\n  \"a-long-value\",\n  \"a-really-really-long-value\",\n  \"a-really-really-really-long-value\"\n);\n\na(\n  \"value\",\n  \"value2\",\n  a(\"long-nested-value\", \"long-nested-value2\", \"long-nested-value3\")\n);\n\na.b().c(\n  {\n    d\n  },\n  () => {}\n);");
    Ok(())
}
#[test]
fn test_jsx_js_trailing_commaall_format_1_3a86c9b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<div\n  onClick={() =>\n    doSomething({\n      foo: bar\n    })\n  }\n/>;")?;
    assert_eq!(
        formatted,
        "<div\n  onClick={() =>\n    doSomething({\n      foo: bar,\n    })\n  }\n/>;"
    );
    Ok(())
}
#[test]
fn test_jsx_js_trailing_commaes_5_format_1_3a86c9b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<div\n  onClick={() =>\n    doSomething({\n      foo: bar\n    })\n  }\n/>;")?;
    assert_eq!(
        formatted,
        "<div\n  onClick={() =>\n    doSomething({\n      foo: bar,\n    })\n  }\n/>;"
    );
    Ok(())
}
#[test]
fn test_jsx_js_trailing_commanone_format_1_3a86c9b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<div\n  onClick={() =>\n    doSomething({\n      foo: bar\n    })\n  }\n/>;")?;
    assert_eq!(
        formatted,
        "<div\n  onClick={() =>\n    doSomething({\n      foo: bar\n    })\n  }\n/>;"
    );
    Ok(())
}
#[test]
fn test_object_js_trailing_commaall_format_1_43473172() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  b: true,\n  c: {\n    c1: 'hello'\n  },\n  d: false\n};\n\nconst aLong = {\n  bHasALongName: 'a-long-value',\n  cHasALongName: {\n    c1: 'a-really-long-value',\n    c2: 'a-really-really-long-value',\n  },\n  dHasALongName: 'a-long-value-too'\n};\n\nconst bLong = {\n  dHasALongName: 'a-long-value-too',\n  eHasABooleanExpression: a === a,\n};") ? ;
    assert_eq ! (formatted , "const a = {\n  b: true,\n  c: {\n    c1: \"hello\",\n  },\n  d: false,\n};\n\nconst aLong = {\n  bHasALongName: \"a-long-value\",\n  cHasALongName: {\n    c1: \"a-really-long-value\",\n    c2: \"a-really-really-long-value\",\n  },\n  dHasALongName: \"a-long-value-too\",\n};\n\nconst bLong = {\n  dHasALongName: \"a-long-value-too\",\n  eHasABooleanExpression: a === a,\n};");
    Ok(())
}
#[test]
fn test_object_js_trailing_commaes_5_format_1_43473172() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  b: true,\n  c: {\n    c1: 'hello'\n  },\n  d: false\n};\n\nconst aLong = {\n  bHasALongName: 'a-long-value',\n  cHasALongName: {\n    c1: 'a-really-long-value',\n    c2: 'a-really-really-long-value',\n  },\n  dHasALongName: 'a-long-value-too'\n};\n\nconst bLong = {\n  dHasALongName: 'a-long-value-too',\n  eHasABooleanExpression: a === a,\n};") ? ;
    assert_eq ! (formatted , "const a = {\n  b: true,\n  c: {\n    c1: \"hello\",\n  },\n  d: false,\n};\n\nconst aLong = {\n  bHasALongName: \"a-long-value\",\n  cHasALongName: {\n    c1: \"a-really-long-value\",\n    c2: \"a-really-really-long-value\",\n  },\n  dHasALongName: \"a-long-value-too\",\n};\n\nconst bLong = {\n  dHasALongName: \"a-long-value-too\",\n  eHasABooleanExpression: a === a,\n};");
    Ok(())
}
#[test]
fn test_object_js_trailing_commanone_format_1_43473172() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = {\n  b: true,\n  c: {\n    c1: 'hello'\n  },\n  d: false\n};\n\nconst aLong = {\n  bHasALongName: 'a-long-value',\n  cHasALongName: {\n    c1: 'a-really-long-value',\n    c2: 'a-really-really-long-value',\n  },\n  dHasALongName: 'a-long-value-too'\n};\n\nconst bLong = {\n  dHasALongName: 'a-long-value-too',\n  eHasABooleanExpression: a === a,\n};") ? ;
    assert_eq ! (formatted , "const a = {\n  b: true,\n  c: {\n    c1: \"hello\"\n  },\n  d: false\n};\n\nconst aLong = {\n  bHasALongName: \"a-long-value\",\n  cHasALongName: {\n    c1: \"a-really-long-value\",\n    c2: \"a-really-really-long-value\"\n  },\n  dHasALongName: \"a-long-value-too\"\n};\n\nconst bLong = {\n  dHasALongName: \"a-long-value-too\",\n  eHasABooleanExpression: a === a\n};");
    Ok(())
}
#[test]
fn test_trailing_whitespace_js_trailing_commaall_format_1_6a375696() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let example = [\n  'FOO',\n  'BAR',\n  // Comment\n];\n\nfoo({},\n  // Comment\n);\n\no = {\n  state,\n  // Comment\n};\n\no = {\n  state,\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB,\n  // Comment\n) {\n  a\n}\n\nthis.getAttribute(function(s)\n  /*string*/ {\n  console.log()\n});\nthis.getAttribute(function(s) /*string*/ {\n  console.log()\n});") ? ;
    assert_eq ! (formatted , "let example = [\n  \"FOO\",\n  \"BAR\",\n  // Comment\n];\n\nfoo(\n  {},\n  // Comment\n);\n\no = {\n  state,\n  // Comment\n};\n\no = {\n  state,\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB,\n  // Comment\n) {\n  a;\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB,\n  // Comment\n) {\n  a;\n}\n\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});");
    Ok(())
}
#[test]
fn test_trailing_whitespace_js_trailing_commaes_5_format_1_6a375696() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let example = [\n  'FOO',\n  'BAR',\n  // Comment\n];\n\nfoo({},\n  // Comment\n);\n\no = {\n  state,\n  // Comment\n};\n\no = {\n  state,\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB,\n  // Comment\n) {\n  a\n}\n\nthis.getAttribute(function(s)\n  /*string*/ {\n  console.log()\n});\nthis.getAttribute(function(s) /*string*/ {\n  console.log()\n});") ? ;
    assert_eq ! (formatted , "let example = [\n  \"FOO\",\n  \"BAR\",\n  // Comment\n];\n\nfoo(\n  {}\n  // Comment\n);\n\no = {\n  state,\n  // Comment\n};\n\no = {\n  state,\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a;\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a;\n}\n\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});");
    Ok(())
}
#[test]
fn test_trailing_whitespace_js_trailing_commanone_format_1_6a375696() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let example = [\n  'FOO',\n  'BAR',\n  // Comment\n];\n\nfoo({},\n  // Comment\n);\n\no = {\n  state,\n  // Comment\n};\n\no = {\n  state,\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB,\n  // Comment\n) {\n  a\n}\n\nthis.getAttribute(function(s)\n  /*string*/ {\n  console.log()\n});\nthis.getAttribute(function(s) /*string*/ {\n  console.log()\n});") ? ;
    assert_eq ! (formatted , "let example = [\n  \"FOO\",\n  \"BAR\"\n  // Comment\n];\n\nfoo(\n  {}\n  // Comment\n);\n\no = {\n  state\n  // Comment\n};\n\no = {\n  state\n\n  // Comment\n};\n\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a;\n}\nfunction supersupersupersuperLongF(\n  supersupersupersuperLongA,\n  supersupersupersuperLongB\n  // Comment\n) {\n  a;\n}\n\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});\nthis.getAttribute(function (s) /*string*/ {\n  console.log();\n});");
    Ok(())
}
