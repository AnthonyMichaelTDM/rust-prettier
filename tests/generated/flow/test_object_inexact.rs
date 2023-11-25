#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_trailing_commaall_format_1_c467bee6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype Foo = {\n  // comment\n  ...,\n};\n\ntype Foo = {\n  /* comment */\n  ...,\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = { /* comment */\n  ...};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...,\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...,\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype Foo = {\n  // comment\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};");
    Ok(())
}
#[test]
fn test_comments_js_trailing_commaes_5_format_1_c467bee6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype Foo = {\n  // comment\n  ...,\n};\n\ntype Foo = {\n  /* comment */\n  ...,\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = { /* comment */\n  ...};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...,\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...,\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype Foo = {\n  // comment\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};");
    Ok(())
}
#[test]
fn test_comments_js_trailing_commanone_format_1_c467bee6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype Foo = {\n  // comment\n  ...,\n};\n\ntype Foo = {\n  /* comment */\n  ...,\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = { /* comment */\n  ...};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...,\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...,\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype Foo = {\n  // comment\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = { /* comment */ ... };\n\ntype Foo = {\n  /* comment */\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  ...\n};\n\ntype Foo = {\n  // comment\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  // comment0\n  // comment1\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  foo: string,\n  ...\n};\n\ntype Foo = {\n  /* comment */\n  [string]: string,\n  ...\n};\n\ntype Foo = {\n  /* comment0 */\n  /* comment1 */\n  foo: string,\n  ...\n};");
    Ok(())
}
#[test]
fn test_test_js_trailing_commaall_format_1_2d9b6676() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("all")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//@flow\ntype T = {\n  a: number,\n  ...,\n}\n\ntype I = {\n  [string]: number,\n  ...,\n}\n\ntype U = { a: number, b: number, c: number, d: number, e: number, f: number, g: number, ...};\n\ntype V = {x: {...}, y: {x: {...}, a: number, b: number, c: number, d: number, e: number, f: number, ...}, z: {...}, foo: number, bar: {foo: number, ...}, ...};\n\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, ...}, ...}) { return x; }\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, ...}, ...}) { return x; }\n\ntype W = {...};\ntype X = {\n  ...,\n};") ? ;
    assert_eq ! (formatted , "//@flow\ntype T = {\n  a: number,\n  ...\n};\n\ntype I = {\n  [string]: number,\n  ...\n};\n\ntype U = {\n  a: number,\n  b: number,\n  c: number,\n  d: number,\n  e: number,\n  f: number,\n  g: number,\n  ...\n};\n\ntype V = {\n  x: { ... },\n  y: {\n    x: { ... },\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    ...\n  },\n  z: { ... },\n  foo: number,\n  bar: { foo: number, ... },\n  ...\n};\n\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: { a: number, ... },\n  ...\n}) {\n  return x;\n}\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: {\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    g: number,\n    h: number,\n    i: number,\n    ...\n  },\n  ...\n}) {\n  return x;\n}\n\ntype W = { ... };\ntype X = { ... };");
    Ok(())
}
#[test]
fn test_test_js_trailing_commaes_5_format_1_2d9b6676() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//@flow\ntype T = {\n  a: number,\n  ...,\n}\n\ntype I = {\n  [string]: number,\n  ...,\n}\n\ntype U = { a: number, b: number, c: number, d: number, e: number, f: number, g: number, ...};\n\ntype V = {x: {...}, y: {x: {...}, a: number, b: number, c: number, d: number, e: number, f: number, ...}, z: {...}, foo: number, bar: {foo: number, ...}, ...};\n\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, ...}, ...}) { return x; }\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, ...}, ...}) { return x; }\n\ntype W = {...};\ntype X = {\n  ...,\n};") ? ;
    assert_eq ! (formatted , "//@flow\ntype T = {\n  a: number,\n  ...\n};\n\ntype I = {\n  [string]: number,\n  ...\n};\n\ntype U = {\n  a: number,\n  b: number,\n  c: number,\n  d: number,\n  e: number,\n  f: number,\n  g: number,\n  ...\n};\n\ntype V = {\n  x: { ... },\n  y: {\n    x: { ... },\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    ...\n  },\n  z: { ... },\n  foo: number,\n  bar: { foo: number, ... },\n  ...\n};\n\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: { a: number, ... },\n  ...\n}) {\n  return x;\n}\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: {\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    g: number,\n    h: number,\n    i: number,\n    ...\n  },\n  ...\n}) {\n  return x;\n}\n\ntype W = { ... };\ntype X = { ... };");
    Ok(())
}
#[test]
fn test_test_js_trailing_commanone_format_1_2d9b6676() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//@flow\ntype T = {\n  a: number,\n  ...,\n}\n\ntype I = {\n  [string]: number,\n  ...,\n}\n\ntype U = { a: number, b: number, c: number, d: number, e: number, f: number, g: number, ...};\n\ntype V = {x: {...}, y: {x: {...}, a: number, b: number, c: number, d: number, e: number, f: number, ...}, z: {...}, foo: number, bar: {foo: number, ...}, ...};\n\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, ...}, ...}) { return x; }\nfunction test(x: {foo: number, bar: number, baz: number, qux: nunber, a: number, b: number, c: {a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, ...}, ...}) { return x; }\n\ntype W = {...};\ntype X = {\n  ...,\n};") ? ;
    assert_eq ! (formatted , "//@flow\ntype T = {\n  a: number,\n  ...\n};\n\ntype I = {\n  [string]: number,\n  ...\n};\n\ntype U = {\n  a: number,\n  b: number,\n  c: number,\n  d: number,\n  e: number,\n  f: number,\n  g: number,\n  ...\n};\n\ntype V = {\n  x: { ... },\n  y: {\n    x: { ... },\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    ...\n  },\n  z: { ... },\n  foo: number,\n  bar: { foo: number, ... },\n  ...\n};\n\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: { a: number, ... },\n  ...\n}) {\n  return x;\n}\nfunction test(x: {\n  foo: number,\n  bar: number,\n  baz: number,\n  qux: nunber,\n  a: number,\n  b: number,\n  c: {\n    a: number,\n    b: number,\n    c: number,\n    d: number,\n    e: number,\n    f: number,\n    g: number,\n    h: number,\n    i: number,\n    ...\n  },\n  ...\n}) {\n  return x;\n}\n\ntype W = { ... };\ntype X = { ... };");
    Ok(())
}
