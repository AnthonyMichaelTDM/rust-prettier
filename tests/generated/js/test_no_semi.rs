#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_class_js_semifalse_format_1_54147b0d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// TODO: upgrade parser\n// class A {\n//   async; // The semicolon is *not* necessary\n//   x(){}\n// }\n// class B {\n//   static; // The semicolon *is* necessary\n//   x(){}\n// }\n\nclass C1 {\n  get; // The semicolon *is* necessary\n  x(){}\n}\nclass C2 {\n  get = () => {}; // The semicolon is *not* necessary\n  x(){}\n}\nclass C3 {\n  set; // The semicolon *is* necessary\n  x(){}\n}\nclass C4 {\n  set = () => {}; // The semicolon is *not* necessary\n  x(){}\n}\n\n\n\nclass A1 {\n  a = 0;\n  [b](){}\n\n  c = 0;\n  *d(){}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {};\n  [h](){}\n\n  p() {};\n  *i(){}\n\n  a = 1;\n  get ['y']() {}\n\n  a = 1;\n  static ['y']() {}\n\n  a = 1;\n  set ['z'](z) {}\n\n  a = 1;\n  async ['a']() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\nclass A2 {\n  a = 0;\n  [b](){}\n\n  c = 0;\n  *d(){}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {};\n  [h](){}\n\n  p() {};\n  *i(){}\n\n  a = 1;\n  get ['y']() {}\n\n  a = 1;\n  static ['y']() {}\n\n  a = 1;\n  set ['z'](z) {}\n\n  a = 1;\n  async ['a']() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\n// being first/last shouldn't break things\nclass G1 {\n  x = 1\n}\nclass G2 {\n  x() {}\n}\nclass G3 {\n  *x() {}\n}\nclass G4 {\n  [x] = 1\n}") ? ;
    assert_eq ! (formatted , "// TODO: upgrade parser\n// class A {\n//   async; // The semicolon is *not* necessary\n//   x(){}\n// }\n// class B {\n//   static; // The semicolon *is* necessary\n//   x(){}\n// }\n\nclass C1 {\n  get; // The semicolon *is* necessary\n  x() {}\n}\nclass C2 {\n  get = () => {} // The semicolon is *not* necessary\n  x() {}\n}\nclass C3 {\n  set; // The semicolon *is* necessary\n  x() {}\n}\nclass C4 {\n  set = () => {} // The semicolon is *not* necessary\n  x() {}\n}\n\nclass A1 {\n  a = 0;\n  [b]() {}\n\n  c = 0;\n  *d() {}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {}\n  [h]() {}\n\n  p() {}\n  *i() {}\n\n  a = 1\n  get [\"y\"]() {}\n\n  a = 1\n  static [\"y\"]() {}\n\n  a = 1\n  set [\"z\"](z) {}\n\n  a = 1\n  async [\"a\"]() {}\n\n  a = 1\n  async *g() {}\n\n  a = 0\n  b = 1\n}\n\nclass A2 {\n  a = 0;\n  [b]() {}\n\n  c = 0;\n  *d() {}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {}\n  [h]() {}\n\n  p() {}\n  *i() {}\n\n  a = 1\n  get [\"y\"]() {}\n\n  a = 1\n  static [\"y\"]() {}\n\n  a = 1\n  set [\"z\"](z) {}\n\n  a = 1\n  async [\"a\"]() {}\n\n  a = 1\n  async *g() {}\n\n  a = 0\n  b = 1\n}\n\n// being first/last shouldn't break things\nclass G1 {\n  x = 1\n}\nclass G2 {\n  x() {}\n}\nclass G3 {\n  *x() {}\n}\nclass G4 {\n  [x] = 1\n}");
    Ok(())
}
#[test]
fn test_class_js_format_1_54147b0d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// TODO: upgrade parser\n// class A {\n//   async; // The semicolon is *not* necessary\n//   x(){}\n// }\n// class B {\n//   static; // The semicolon *is* necessary\n//   x(){}\n// }\n\nclass C1 {\n  get; // The semicolon *is* necessary\n  x(){}\n}\nclass C2 {\n  get = () => {}; // The semicolon is *not* necessary\n  x(){}\n}\nclass C3 {\n  set; // The semicolon *is* necessary\n  x(){}\n}\nclass C4 {\n  set = () => {}; // The semicolon is *not* necessary\n  x(){}\n}\n\n\n\nclass A1 {\n  a = 0;\n  [b](){}\n\n  c = 0;\n  *d(){}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {};\n  [h](){}\n\n  p() {};\n  *i(){}\n\n  a = 1;\n  get ['y']() {}\n\n  a = 1;\n  static ['y']() {}\n\n  a = 1;\n  set ['z'](z) {}\n\n  a = 1;\n  async ['a']() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\nclass A2 {\n  a = 0;\n  [b](){}\n\n  c = 0;\n  *d(){}\n\n  e = 0;\n  [f] = 0\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {};\n  [h](){}\n\n  p() {};\n  *i(){}\n\n  a = 1;\n  get ['y']() {}\n\n  a = 1;\n  static ['y']() {}\n\n  a = 1;\n  set ['z'](z) {}\n\n  a = 1;\n  async ['a']() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\n// being first/last shouldn't break things\nclass G1 {\n  x = 1\n}\nclass G2 {\n  x() {}\n}\nclass G3 {\n  *x() {}\n}\nclass G4 {\n  [x] = 1\n}") ? ;
    assert_eq ! (formatted , "// TODO: upgrade parser\n// class A {\n//   async; // The semicolon is *not* necessary\n//   x(){}\n// }\n// class B {\n//   static; // The semicolon *is* necessary\n//   x(){}\n// }\n\nclass C1 {\n  get; // The semicolon *is* necessary\n  x() {}\n}\nclass C2 {\n  get = () => {}; // The semicolon is *not* necessary\n  x() {}\n}\nclass C3 {\n  set; // The semicolon *is* necessary\n  x() {}\n}\nclass C4 {\n  set = () => {}; // The semicolon is *not* necessary\n  x() {}\n}\n\nclass A1 {\n  a = 0;\n  [b]() {}\n\n  c = 0;\n  *d() {}\n\n  e = 0;\n  [f] = 0;\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {}\n  [h]() {}\n\n  p() {}\n  *i() {}\n\n  a = 1;\n  get [\"y\"]() {}\n\n  a = 1;\n  static [\"y\"]() {}\n\n  a = 1;\n  set [\"z\"](z) {}\n\n  a = 1;\n  async [\"a\"]() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\nclass A2 {\n  a = 0;\n  [b]() {}\n\n  c = 0;\n  *d() {}\n\n  e = 0;\n  [f] = 0;\n\n  // none of the semicolons above this comment can be omitted.\n  // none of the semicolons below this comment are necessary.\n\n  q() {}\n  [h]() {}\n\n  p() {}\n  *i() {}\n\n  a = 1;\n  get [\"y\"]() {}\n\n  a = 1;\n  static [\"y\"]() {}\n\n  a = 1;\n  set [\"z\"](z) {}\n\n  a = 1;\n  async [\"a\"]() {}\n\n  a = 1;\n  async *g() {}\n\n  a = 0;\n  b = 1;\n}\n\n// being first/last shouldn't break things\nclass G1 {\n  x = 1;\n}\nclass G2 {\n  x() {}\n}\nclass G3 {\n  *x() {}\n}\nclass G4 {\n  [x] = 1;\n}");
    Ok(())
}
#[test]
fn test_comments_js_semifalse_format_1_e6ad7522() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let error = new Error(response.statusText);\n// comment\n[].response = response\n\nx;\n\n/* comment */ [].response = response\n\nx;\n\n[].response = response; /* comment */") ? ;
    assert_eq ! (formatted , "let error = new Error(response.statusText)\n// comment\n;[].response = response\n\nx\n\n/* comment */ ;[].response = response\n\nx\n\n;[].response = response /* comment */");
    Ok(())
}
#[test]
fn test_comments_js_format_1_e6ad7522() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let error = new Error(response.statusText);\n// comment\n[].response = response\n\nx;\n\n/* comment */ [].response = response\n\nx;\n\n[].response = response; /* comment */") ? ;
    assert_eq ! (formatted , "let error = new Error(response.statusText);\n// comment\n[].response = response;\n\nx;\n\n/* comment */ [].response = response;\n\nx;\n\n[].response = response; /* comment */");
    Ok(())
}
#[test]
fn test_issue_2006_js_semifalse_format_1_a546abe5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (n) {\n  case 11:\n    var c = a.e;\n    (i.a += Ga(c.e)), F(i, c.i, 0);\n}\n\nvar c = a.e;\n(i.a += Ga(c.e)), F(i, c.i, 0);") ? ;
    assert_eq ! (formatted , "switch (n) {\n  case 11:\n    var c = a.e\n    ;(i.a += Ga(c.e)), F(i, c.i, 0)\n}\n\nvar c = a.e\n;(i.a += Ga(c.e)), F(i, c.i, 0)");
    Ok(())
}
#[test]
fn test_issue_2006_js_format_1_a546abe5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("switch (n) {\n  case 11:\n    var c = a.e;\n    (i.a += Ga(c.e)), F(i, c.i, 0);\n}\n\nvar c = a.e;\n(i.a += Ga(c.e)), F(i, c.i, 0);") ? ;
    assert_eq ! (formatted , "switch (n) {\n  case 11:\n    var c = a.e;\n    (i.a += Ga(c.e)), F(i, c.i, 0);\n}\n\nvar c = a.e;\n(i.a += Ga(c.e)), F(i, c.i, 0);");
    Ok(())
}
#[test]
fn test_no_semi_js_semifalse_format_1_97f4d576() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n// with preexisting semi\n\nx; [1, 2, 3].forEach(fn)\nx; [a, b, ...c] = [1, 2]\nx; /r/i.test('r')\nx; +1\nx; - 1\nx; ('h' + 'i').repeat(10)\nx; (1, 2)\nx; (() => {})()\nx; ({ a: 1 }).entries()\nx; ({ a: 1 }).entries()\nx; <Hello />\nx; `string`\nx; (x, y) => x\n\n// doesn't have to be preceded by a semicolon\n\nclass X {} [1, 2, 3].forEach(fn)\n\n\n// don't semicolon if it doesn't start statement\n\nif (true) (() => {})()\n\n\n// check indentation\n\nif (true) {\n  x; (() => {})()\n}\n\n// check statement clauses\n\ndo break; while (false)\nif (true) do break; while (false)\n\nif (true) 1; else 2\nfor (;;) ;\nfor (x of y) ;\n\ndebugger\n\n// check that it doesn't break non-ASI\n\n1\n- 1\n\n1\n+ 1\n\n1\n/ 1\n\narr\n[0]\n\nfn\n(x)\n\n!1\n\n1\n< 1\n\ntag\n`string`\n\nx; x => x\n\nx; (a || b).c++\n\nx; ++(a || b).c\n\nwhile (false)\n  (function(){}())\n\naReallyLongLine012345678901234567890123456789012345678901234567890123456789 *\n  (b + c)") ? ;
    assert_eq ! (formatted , "// with preexisting semi\n\nx\n;[1, 2, 3].forEach(fn)\nx\n;[a, b, ...c] = [1, 2]\nx\n;/r/i.test(\"r\")\nx\n;+1\nx\n;-1\nx\n;(\"h\" + \"i\").repeat(10)\nx\n1, 2\nx\n;(() => {})()\nx\n;({ a: 1 }).entries()\nx\n;({ a: 1 }).entries()\nx\n;<Hello />\nx\n;`string`\nx\n;(x, y) => x\n\n// doesn't have to be preceded by a semicolon\n\nclass X {}\n;[1, 2, 3].forEach(fn)\n\n// don't semicolon if it doesn't start statement\n\nif (true) (() => {})()\n\n// check indentation\n\nif (true) {\n  x\n  ;(() => {})()\n}\n\n// check statement clauses\n\ndo break\nwhile (false)\nif (true)\n  do break\n  while (false)\n\nif (true) 1\nelse 2\nfor (;;);\nfor (x of y);\n\ndebugger\n\n// check that it doesn't break non-ASI\n\n1 - 1\n\n1 + 1\n\n1 / 1\n\narr[0]\n\nfn(x)\n\n!1\n\n1 < 1\n\ntag`string`\n\nx\n;(x) => x\n\nx\n;(a || b).c++\n\nx\n++(a || b).c\n\nwhile (false) (function () {})()\n\naReallyLongLine012345678901234567890123456789012345678901234567890123456789 *\n  (b + c)");
    Ok(())
}
#[test]
fn test_no_semi_js_format_1_97f4d576() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\n// with preexisting semi\n\nx; [1, 2, 3].forEach(fn)\nx; [a, b, ...c] = [1, 2]\nx; /r/i.test('r')\nx; +1\nx; - 1\nx; ('h' + 'i').repeat(10)\nx; (1, 2)\nx; (() => {})()\nx; ({ a: 1 }).entries()\nx; ({ a: 1 }).entries()\nx; <Hello />\nx; `string`\nx; (x, y) => x\n\n// doesn't have to be preceded by a semicolon\n\nclass X {} [1, 2, 3].forEach(fn)\n\n\n// don't semicolon if it doesn't start statement\n\nif (true) (() => {})()\n\n\n// check indentation\n\nif (true) {\n  x; (() => {})()\n}\n\n// check statement clauses\n\ndo break; while (false)\nif (true) do break; while (false)\n\nif (true) 1; else 2\nfor (;;) ;\nfor (x of y) ;\n\ndebugger\n\n// check that it doesn't break non-ASI\n\n1\n- 1\n\n1\n+ 1\n\n1\n/ 1\n\narr\n[0]\n\nfn\n(x)\n\n!1\n\n1\n< 1\n\ntag\n`string`\n\nx; x => x\n\nx; (a || b).c++\n\nx; ++(a || b).c\n\nwhile (false)\n  (function(){}())\n\naReallyLongLine012345678901234567890123456789012345678901234567890123456789 *\n  (b + c)") ? ;
    assert_eq ! (formatted , "// with preexisting semi\n\nx;\n[1, 2, 3].forEach(fn);\nx;\n[a, b, ...c] = [1, 2];\nx;\n/r/i.test(\"r\");\nx;\n+1;\nx;\n-1;\nx;\n(\"h\" + \"i\").repeat(10);\nx;\n1, 2;\nx;\n(() => {})();\nx;\n({ a: 1 }).entries();\nx;\n({ a: 1 }).entries();\nx;\n<Hello />;\nx;\n`string`;\nx;\n(x, y) => x;\n\n// doesn't have to be preceded by a semicolon\n\nclass X {}\n[1, 2, 3].forEach(fn);\n\n// don't semicolon if it doesn't start statement\n\nif (true) (() => {})();\n\n// check indentation\n\nif (true) {\n  x;\n  (() => {})();\n}\n\n// check statement clauses\n\ndo break;\nwhile (false);\nif (true)\n  do break;\n  while (false);\n\nif (true) 1;\nelse 2;\nfor (;;);\nfor (x of y);\n\ndebugger;\n\n// check that it doesn't break non-ASI\n\n1 - 1;\n\n1 + 1;\n\n1 / 1;\n\narr[0];\n\nfn(x);\n\n!1;\n\n1 < 1;\n\ntag`string`;\n\nx;\n(x) => x;\n\nx;\n(a || b).c++;\n\nx;\n++(a || b).c;\n\nwhile (false) (function () {})();\n\naReallyLongLine012345678901234567890123456789012345678901234567890123456789 *\n  (b + c);");
    Ok(())
}
#[test]
fn test_private_field_js_semifalse_format_1_63431ff1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class C {\n  #field = 'value';\n  [\"method\"]() {}\n}")?;
    assert_eq!(
        formatted,
        "class C {\n  #field = \"value\";\n  [\"method\"]() {}\n}"
    );
    Ok(())
}
#[test]
fn test_private_field_js_format_1_63431ff1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class C {\n  #field = 'value';\n  [\"method\"]() {}\n}")?;
    assert_eq!(
        formatted,
        "class C {\n  #field = \"value\";\n  [\"method\"]() {}\n}"
    );
    Ok(())
}
