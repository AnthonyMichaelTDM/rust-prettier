#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_anonymous_js_format_1_035f9f52() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const f1 = function* () {\n  yield 0;\n};\n  \nconst f2 = function * () {\n  yield 0;\n};\n\nconst f3 = function* () {\n};\n  \n(function* () {\n  yield 0;\n});\n  \n(function * () {\n  yield 0;\n});\n\n(function* () {\n});\n ") ? ;
    assert_eq ! (formatted , "const f1 = function* () {\n  yield 0;\n};\n\nconst f2 = function* () {\n  yield 0;\n};\n\nconst f3 = function* () {};\n\n(function* () {\n  yield 0;\n});\n\n(function* () {\n  yield 0;\n});\n\n(function* () {});");
    Ok(())
}
#[test]
fn test_async_js_format_1_f303d11f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/meriyah/meriyah/commit/f21882c312284572ac6d7e7630c4a677d6caed92\n\nconst f = async function * (source, block, opts) {\n  for await (const entry of source) {\n    yield async function () {\n      const cid = await persist(entry.content.serialize(), block, opts)\n      return {\n        cid,\n        path: entry.path,\n        unixfs: UnixFS.unmarshal(entry.content.Data),\n        node: entry.content\n      }\n    }\n  }\n}") ? ;
    assert_eq ! (formatted , "// https://github.com/meriyah/meriyah/commit/f21882c312284572ac6d7e7630c4a677d6caed92\n\nconst f = async function* (source, block, opts) {\n  for await (const entry of source) {\n    yield async function () {\n      const cid = await persist(entry.content.serialize(), block, opts);\n      return {\n        cid,\n        path: entry.path,\n        unixfs: UnixFS.unmarshal(entry.content.Data),\n        node: entry.content,\n      };\n    };\n  }\n};");
    Ok(())
}
#[test]
fn test_function_name_starts_with_get_js_format_1_79e098d3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/meriyah/meriyah/issues/164\n\nfunction get() {}\n\nfunction* getData() {\n    return yield get();\n}") ? ;
    assert_eq ! (formatted , "// https://github.com/meriyah/meriyah/issues/164\n\nfunction get() {}\n\nfunction* getData() {\n  return yield get();\n}");
    Ok(())
}
