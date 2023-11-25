#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_iteration_js_format_1_71888016() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nasync function * a() {\n    yield* b();\n}\n\nclass X {\n    async * b() {\n        yield* a();\n    }\n}") ? ;
    assert_eq ! (formatted , "async function* a() {\n  yield* b();\n}\n\nclass X {\n  async *b() {\n    yield* a();\n  }\n}");
    Ok(())
}
#[test]
fn test_async_shorthand_method_js_format_1_8bb3ad65() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("({\n  async get() {},\n  async set() {}\n});")?;
    assert_eq!(formatted, "({\n  async get() {},\n  async set() {},\n});");
    Ok(())
}
#[test]
fn test_await_parse_js_format_1_007dd4aa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f1() { (await f()).length }\nasync function g() {\n  invariant(\n    (await driver.navigator.getUrl()).substr(-7)\n  );\n}\nfunction *f2(){\n  !(yield a);\n}\nasync function f3() {\n  a = !await f();\n}\nasync () => {\n  new A(await x);\n  obj[await x];\n}") ? ;
    assert_eq ! (formatted , "async function f1() {\n  (await f()).length;\n}\nasync function g() {\n  invariant((await driver.navigator.getUrl()).substr(-7));\n}\nfunction* f2() {\n  !(yield a);\n}\nasync function f3() {\n  a = !(await f());\n}\nasync () => {\n  new A(await x);\n  obj[await x];\n};");
    Ok(())
}
#[test]
fn test_conditional_expression_js_format_1_0723f3f4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\n  const result = typeof fn === 'function' ? await fn() : null;\n}\n\n(async function() {\n  console.log(\n    await (true ? Promise.resolve(\"A\") : Promise.resolve(\"B\"))\n  );\n})()\n\nasync function f2() {\n  await (spellcheck && spellcheck.setChecking(false));\n  await spellcheck && spellcheck.setChecking(false)\n}") ? ;
    assert_eq ! (formatted , "async function f() {\n  const result = typeof fn === \"function\" ? await fn() : null;\n}\n\n(async function () {\n  console.log(await (true ? Promise.resolve(\"A\") : Promise.resolve(\"B\")));\n})();\n\nasync function f2() {\n  await (spellcheck && spellcheck.setChecking(false));\n  (await spellcheck) && spellcheck.setChecking(false);\n}");
    Ok(())
}
#[test]
fn test_exponentiation_js_format_1_a5e8a41b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("async () => (await 5) ** 6;")?;
    assert_eq!(formatted, "async () => (await 5) ** 6;");
    Ok(())
}
#[test]
fn test_inline_await_js_format_1_815e1e5e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\n  const admins = (await(db.select('*').from('admins').leftJoin('bla').where('id', 'in', [1,2,3,4]))).map(({id, name})=>({id, name}))\n}") ? ;
    assert_eq ! (formatted , "async function f() {\n  const admins = (\n    await db\n      .select(\"*\")\n      .from(\"admins\")\n      .leftJoin(\"bla\")\n      .where(\"id\", \"in\", [1, 2, 3, 4])\n  ).map(({ id, name }) => ({ id, name }));\n}");
    Ok(())
}
#[test]
fn test_nested_js_format_1_54f41953() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getAccountCount = async () =>\n  (await\n    (await (\n      await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n    ).findItem(\"My bookmarks\")\n  ).getChildren()\n  ).length") ? ;
    assert_eq ! (formatted , "const getAccountCount = async () =>\n  (\n    await (\n      await (\n        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n      ).findItem(\"My bookmarks\")\n    ).getChildren()\n  ).length;");
    Ok(())
}
#[test]
fn test_nested_2_js_format_1_d296cd2e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\nawait Promise.all(\n  (await readdir(\"src\")).map(async (path) => {\n    import(`./${path}`);\n  })\n);}") ? ;
    assert_eq ! (formatted , "async function f() {\n  await Promise.all(\n    (await readdir(\"src\")).map(async (path) => {\n      import(`./${path}`);\n    }),\n  );\n}");
    Ok(())
}
#[test]
fn test_parens_js_format_1_2693d21d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "async function *f(){ await (yield x); }\n\nasync function f2(){ await (() => {}); }",
    )?;
    assert_eq ! (formatted , "async function* f() {\n  await (yield x);\n}\n\nasync function f2() {\n  await (() => {});\n}");
    Ok(())
}
#[test]
fn test_simple_nested_await_js_format_1_765ad5ec() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function f() {\n  const a = await (await request()).json();\n  const b = await fs.writeFile(file, await (await request()).text());\n}") ? ;
    assert_eq ! (formatted , "async function f() {\n  const a = await (await request()).json();\n  const b = await fs.writeFile(file, await (await request()).text());\n}");
    Ok(())
}
