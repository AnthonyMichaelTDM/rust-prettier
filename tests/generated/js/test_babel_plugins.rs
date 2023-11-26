#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_async_do_expressions_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_async_do_expressions_js_format_1_41658069() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("async do { await requestAPI().json() };")?;
    assert_eq!(formatted, "(async do {\n  await requestAPI().json();\n});");
    Ok(())
}
#[test]
fn test_async_generators_js_format_1_eba74089() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-async-generator-functions\n\nasync function* agf() {\n  await 1;\n  yield 2;\n}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-async-generator-functions\n\nasync function* agf() {\n  await 1;\n  yield 2;\n}");
    Ok(())
}
#[test]
fn test_bigint_js_format_1_d70fcd72() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/tc39/proposal-bigint\n\nconst previousMaxSafe = BigInt(Number.MAX_SAFE_INTEGER);\n// ↪ 9007199254740991\n\nconst maxPlusOne = previousMaxSafe + 1n;\n// ↪ 9007199254740992n\n\nconst theFuture = previousMaxSafe + 2n;\n// ↪ 9007199254740993n, this works now!\n\nconst multi = previousMaxSafe * 2n;\n// ↪ 18014398509481982n\n\n// `–` is not minus sign,\n// SIC https://github.com/tc39/proposal-bigint#operators\n// const subtr = multi – 10n;\n// ↪ 18014398509481972n\n\nconst mod = multi % 10n;\n// ↪ 2n\n\nconst bigN = 2n ** 54n;\n// ↪ 18014398509481984n\n\nbigN * -1n\n// ↪ –18014398509481984n\n\n0n === 0\n// ↪ false\n\n0n == 0\n// ↪ true\n\n1n < 2\n// ↪ true\n\n2n > 1\n// ↪ true\n\n2 > 2\n// ↪ false\n\n2n > 2\n// ↪ false\n\n2n >= 2\n// ↪ true\n\nconst mixed = [4n, 6, -12n, 10, 4, 0, 0n];\n// ↪  [4n, 6, -12n, 10, 4, 0, 0n]\n\nmixed.sort();\n// ↪ [-12n, 0, 0n, 10, 4n, 4, 6]\n\nif (0n) {\n  console.log('Hello from the if!');\n} else {\n  console.log('Hello from the else!');\n}\n\n// ↪ \"Hello from the else!\"\n\n0n || 12n\n// ↪ 12n\n\n0n && 12n\n// ↪ 0n\n\nBoolean(0n)\n// ↪ false\n\nBoolean(12n)\n// ↪ true\n\n!12n\n// ↪ false\n\n!0n\n// ↪ true\n\nconst view = new BigInt64Array(4);\n// ↪ [0n, 0n, 0n, 0n]\nview.length;\n// ↪ 4\nview[0];\n// ↪ 0n\nview[0] = 42n;\nview[0];\n// ↪ 42n\n\n// Highest possible BigInt value that can be represented as a\n// signed 64-bit integer.\nconst max = 2n ** (64n - 1n) - 1n;\nview[0] = max;\nview[0];\n// ↪ 9_223_372_036_854_775_807n\nview[0] = max + 1n;\nview[0];\n// ↪ -9_223_372_036_854_775_808n\n//   ^ negative because of overflow\n\n1n + 2\n// ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions\n\n1n * 2\n// ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions\n\n+1n\n// ↪ TypeError: Cannot convert a BigInt value to a number\n\nNumber(1n)\n// ↪ 1\n\n1n + '2'\n// ↪ \"12\"\n\n'2' + 1n\n// ↪ \"21\"\n\nconst badPrecision = BigInt(9007199254740993);\n// ↪9007199254740992n\n\nconst goodPrecision = BigInt('9007199254740993');\n// ↪9007199254740993n\n\nconst alsoGoodPrecision = 9007199254740993n;\n// ↪9007199254740993n") ? ;
    assert_eq ! (formatted , "// https://github.com/tc39/proposal-bigint\n\nconst previousMaxSafe = BigInt(Number.MAX_SAFE_INTEGER);\n// ↪ 9007199254740991\n\nconst maxPlusOne = previousMaxSafe + 1n;\n// ↪ 9007199254740992n\n\nconst theFuture = previousMaxSafe + 2n;\n// ↪ 9007199254740993n, this works now!\n\nconst multi = previousMaxSafe * 2n;\n// ↪ 18014398509481982n\n\n// `–` is not minus sign,\n// SIC https://github.com/tc39/proposal-bigint#operators\n// const subtr = multi – 10n;\n// ↪ 18014398509481972n\n\nconst mod = multi % 10n;\n// ↪ 2n\n\nconst bigN = 2n ** 54n;\n// ↪ 18014398509481984n\n\nbigN * -1n;\n// ↪ –18014398509481984n\n\n0n === 0;\n// ↪ false\n\n0n == 0;\n// ↪ true\n\n1n < 2;\n// ↪ true\n\n2n > 1;\n// ↪ true\n\n2 > 2;\n// ↪ false\n\n2n > 2;\n// ↪ false\n\n2n >= 2;\n// ↪ true\n\nconst mixed = [4n, 6, -12n, 10, 4, 0, 0n];\n// ↪  [4n, 6, -12n, 10, 4, 0, 0n]\n\nmixed.sort();\n// ↪ [-12n, 0, 0n, 10, 4n, 4, 6]\n\nif (0n) {\n  console.log(\"Hello from the if!\");\n} else {\n  console.log(\"Hello from the else!\");\n}\n\n// ↪ \"Hello from the else!\"\n\n0n || 12n;\n// ↪ 12n\n\n0n && 12n;\n// ↪ 0n\n\nBoolean(0n);\n// ↪ false\n\nBoolean(12n);\n// ↪ true\n\n!12n;\n// ↪ false\n\n!0n;\n// ↪ true\n\nconst view = new BigInt64Array(4);\n// ↪ [0n, 0n, 0n, 0n]\nview.length;\n// ↪ 4\nview[0];\n// ↪ 0n\nview[0] = 42n;\nview[0];\n// ↪ 42n\n\n// Highest possible BigInt value that can be represented as a\n// signed 64-bit integer.\nconst max = 2n ** (64n - 1n) - 1n;\nview[0] = max;\nview[0];\n// ↪ 9_223_372_036_854_775_807n\nview[0] = max + 1n;\nview[0];\n// ↪ -9_223_372_036_854_775_808n\n//   ^ negative because of overflow\n\n1n + 2;\n// ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions\n\n1n * 2 +\n  // ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions\n\n  1n;\n// ↪ TypeError: Cannot convert a BigInt value to a number\n\nNumber(1n);\n// ↪ 1\n\n1n + \"2\";\n// ↪ \"12\"\n\n\"2\" + 1n;\n// ↪ \"21\"\n\nconst badPrecision = BigInt(9007199254740993);\n// ↪9007199254740992n\n\nconst goodPrecision = BigInt(\"9007199254740993\");\n// ↪9007199254740993n\n\nconst alsoGoodPrecision = 9007199254740993n;\n// ↪9007199254740993n");
    Ok(())
}
#[test]
fn test_class_properties_js_format_1_9d9a3f60() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-class-properties\n\nclass Bork {\n    //Property initializer syntax\n    instanceProperty = \"bork\";\n    boundFunction = () => {\n      return this.instanceProperty;\n    };\n\n    //Static class properties\n    static staticProperty = \"babelIsCool\";\n    static staticFunction = function() {\n      return Bork.staticProperty;\n    };\n  }\n\n  let myBork = new Bork;\n\n  //Property initializers are not on the prototype.\n  console.log(myBork.__proto__.boundFunction); // > undefined\n\n  //Bound functions are bound to the class instance.\n  console.log(myBork.boundFunction.call(undefined)); // > \"bork\"\n\n  //Static function exists on the class.\n  console.log(Bork.staticFunction()); // > \"babelIsCool\"") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-class-properties\n\nclass Bork {\n  //Property initializer syntax\n  instanceProperty = \"bork\";\n  boundFunction = () => {\n    return this.instanceProperty;\n  };\n\n  //Static class properties\n  static staticProperty = \"babelIsCool\";\n  static staticFunction = function () {\n    return Bork.staticProperty;\n  };\n}\n\nlet myBork = new Bork();\n\n//Property initializers are not on the prototype.\nconsole.log(myBork.__proto__.boundFunction); // > undefined\n\n//Bound functions are bound to the class instance.\nconsole.log(myBork.boundFunction.call(undefined)); // > \"bork\"\n\n//Static function exists on the class.\nconsole.log(Bork.staticFunction()); // > \"babelIsCool\"");
    Ok(())
}
#[test]
fn test_class_static_block_js_format_1_ff536fae() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class C {\n  static #x = 42;\n  static y;\n  static {\n    try {\n      this.y = doSomethingWith(this.#x);\n    } catch {\n      this.y = \"unknown\";\n    }\n  }\n}") ? ;
    assert_eq ! (formatted , "class C {\n  static #x = 42;\n  static y;\n  static {\n    try {\n      this.y = doSomethingWith(this.#x);\n    } catch {\n      this.y = \"unknown\";\n    }\n  }\n}");
    Ok(())
}
#[test]
fn test_decimal_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decimal_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decimal_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decimal_js_format_1_6260d1ed() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/11640\n\n100m;\n9223372036854775807m;\n0.m;\n3.1415926535897932m;\n100.000m;\n.1m;\n({ 0m: 0, .1m() {}, get 0.2m(){}, set 3m(_){}, async 4m() {}, *.5m() {} });\n1.m;\n100m;\n9223372036854775807m;\n100.m;\n\n// Invalid decimal\n2e9m;\n016432m;\n089m;\n\n// https://github.com/tc39/proposal-decimal\n.1m + .2m === .3m;\n2.00m;\n-0m;\ntypeof 1m === \"bigdecimal\";\ntypeof 1m === \"decimal128\";\n") ? ;
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/11640\n\n100m;\n9223372036854775807m;\n0m;\n3.1415926535897932m;\n100.0m;\n0.1m;\n({ 0m: 0, 0.1m() {}, get 0.2m() {}, set 3m(_) {}, async 4m() {}, *0.5m() {} });\n1m;\n100m;\n9223372036854775807m;\n100m;\n\n// Invalid decimal\n2e9m;\n016432m;\n089m;\n\n// https://github.com/tc39/proposal-decimal\n0.1m + 0.2m === 0.3m;\n2.0m;\n-0m;\ntypeof 1m === \"bigdecimal\";\ntypeof 1m === \"decimal128\";");
    Ok(())
}
#[test]
fn test_decorator_auto_accessors_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decorator_auto_accessors_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decorator_auto_accessors_js_format_1_407125f4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("class C extends HTMLElement {\n  accessor clicked = false;\n}")?;
    assert_eq!(
        formatted,
        "class C extends HTMLElement {\n  accessor clicked = false;\n}"
    );
    Ok(())
}
#[test]
fn test_decorators_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decorators_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_decorators_js_format_1_902d79cc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-decorators\n\n@annotation\nclass MyClass { }\n\nfunction annotation(target) {\n   target.annotated = true;\n}\n\n@isTestable(true)\nclass MyClass { }\n\nfunction isTestable(value) {\n   return function decorator(target) {\n      target.isTestable = value;\n   }\n}\n\nclass C {\n  @enumerable(false)\n  method() { }\n}\n\nfunction enumerable(value) {\n  return function (target, key, descriptor) {\n     descriptor.enumerable = value;\n     return descriptor;\n  }\n}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-decorators\n\n@annotation\nclass MyClass {}\n\nfunction annotation(target) {\n  target.annotated = true;\n}\n\n@isTestable(true)\nclass MyClass {}\n\nfunction isTestable(value) {\n  return function decorator(target) {\n    target.isTestable = value;\n  };\n}\n\nclass C {\n  @enumerable(false)\n  method() {}\n}\n\nfunction enumerable(value) {\n  return function (target, key, descriptor) {\n    descriptor.enumerable = value;\n    return descriptor;\n  };\n}");
    Ok(())
}
#[test]
fn test_deferred_import_evaluation_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_deferred_import_evaluation_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_deferred_import_evaluation_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_deferred_import_evaluation_js_format_1_20806599() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import defer * as ns from \"x\";")?;
    assert_eq!(formatted, "import defer * as ns from \"x\";");
    Ok(())
}
#[test]
fn test_destructuring_private_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_private_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_private_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_destructuring_private_js_format_1_57d3a6a5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class Foo {\n  #x = 1;\n  constructor() {\n    console.log(this.#x); // => 1\n    const { #x: x } = this;\n    console.log(x); // => 1\n  }\n  equals({ #x: otherX }) {\n    const { #x: currentX } = this;\n    return currentX === otherX;\n  }\n}") ? ;
    assert_eq ! (formatted , "class Foo {\n  #x = 1;\n  constructor() {\n    console.log(this.#x); // => 1\n    const { #x: x } = this;\n    console.log(x); // => 1\n  }\n  equals({ #x: otherX }) {\n    const { #x: currentX } = this;\n    return currentX === otherX;\n  }\n}");
    Ok(())
}
#[test]
fn test_do_expressions_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_do_expressions_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_do_expressions_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_do_expressions_js_format_1_bbd44dcf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-do-expressions\n\nlet a = do {\n  if(x > 10) {\n    'big';\n  } else {\n    'small';\n  }\n};\n// is equivalent to:\nlet a = x > 10 ? 'big' : 'small';") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-do-expressions\n\nlet a = do {\n  if (x > 10) {\n    (\"big\");\n  } else {\n    (\"small\");\n  }\n};\n// is equivalent to:\nlet a = x > 10 ? \"big\" : \"small\";");
    Ok(())
}
#[test]
fn test_dynamic_import_js_format_1_eb0976b7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-syntax-dynamic-import\n\n// There is no example code on babel website\n\nimport('./prettier.mjs');\nimport(prettier);\nimport('./prettier.mjs').then(module => console.log(module));\nimport(prettier).then(module => console.log(module));") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-syntax-dynamic-import\n\n// There is no example code on babel website\n\nimport(\"./prettier.mjs\");\nimport(prettier);\nimport(\"./prettier.mjs\").then((module) => console.log(module));\nimport(prettier).then((module) => console.log(module));");
    Ok(())
}
#[test]
fn test_explicit_resource_management_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_explicit_resource_management_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_explicit_resource_management_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_explicit_resource_management_js_format_1_067ac2a9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function * g() {\n  using handle = acquireFileHandle(); // block-scoped critical resource\n} // cleanup\n\n{\n  using obj = g(); // block-scoped declaration\n  const r = obj.next();\n} // calls finally blocks in `g`\n\n{\n  await using obj = g();\n}") ? ;
    assert_eq ! (formatted , "function* g() {\n  using handle = acquireFileHandle(); // block-scoped critical resource\n} // cleanup\n\n{\n  using obj = g(); // block-scoped declaration\n  const r = obj.next();\n} // calls finally blocks in `g`\n\n{\n  await using obj = g();\n}");
    Ok(())
}
#[test]
fn test_export_default_from_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_default_from_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_default_from_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_default_from_js_format_1_f68bf745() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-export-default-from\n\n\nexport v from 'mod';") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-export-default-from\n\nexport v from \"mod\";");
    Ok(())
}
#[test]
fn test_export_namespace_from_js_format_1_21153682() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-export-namespace-from\n\nexport * as ns from 'mod';") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-export-namespace-from\n\nexport * as ns from \"mod\";");
    Ok(())
}
#[test]
fn test_flow_js_babel_estree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_flow_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_flow_js_babel_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_flow_js_babel_flow_format_1_741a39b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-preset-flow\n\nfunction foo(one: any, two: number, three?): string {}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-preset-flow\n\nfunction foo(one: any, two: number, three?): string {}");
    Ok(())
}
#[test]
fn test_flow_js_babel_ts_format_1_741a39b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-preset-flow\n\nfunction foo(one: any, two: number, three?): string {}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-preset-flow\n\nfunction foo(one: any, two: number, three?): string {}");
    Ok(())
}
#[test]
fn test_flow_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_flow_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_bind_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_bind_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_bind_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_bind_js_format_1_ae628d65() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-function-bind\n\nobj::func\n// is equivalent to:\nfunc.bind(obj)\n\n::obj.func\n// is equivalent to:\nobj.func.bind(obj)\n\nobj::func(val)\n// is equivalent to:\nfunc.call(obj, val)\n\n::obj.func(val)\n// is equivalent to:\nobj.func.call(obj, val)") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-function-bind\n\nobj::func;\n// is equivalent to:\nfunc.bind(obj)::obj.func;\n// is equivalent to:\nobj.func.bind(obj);\n\nobj::func(val);\n// is equivalent to:\nfunc\n  .call(obj, val)\n\n  ::obj.func(val);\n// is equivalent to:\nobj.func.call(obj, val);");
    Ok(())
}
#[test]
fn test_function_sent_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_sent_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_sent_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_function_sent_js_format_1_b35c6cb2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-function-sent\n\nfunction* generator() {\n    console.log(\"Sent\", function.sent);\n    console.log(\"Yield\", yield);\n}\n\nconst iterator = generator();\niterator.next(1); // Logs \"Sent 1\"\niterator.next(2); // Logs \"Yield 2\"") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-function-sent\n\nfunction* generator() {\n  console.log(\"Sent\", function.sent);\n  console.log(\"Yield\", yield);\n}\n\nconst iterator = generator();\niterator.next(1); // Logs \"Sent 1\"\niterator.next(2); // Logs \"Yield 2\"");
    Ok(())
}
#[test]
fn test_import_assertions_dynamic_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_dynamic_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_dynamic_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_dynamic_js_format_1_c70cf81a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import(\"./foo.json\", { assert: { type: \"json\" } });")?;
    assert_eq!(
        formatted,
        "import(\"./foo.json\", { assert: { type: \"json\" } });"
    );
    Ok(())
}
#[test]
fn test_import_assertions_static_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_static_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_static_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_assertions_static_js_format_1_cb2c9c31() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import json from \"./foo.json\" assert { type: \"json\" };")?;
    assert_eq!(
        formatted,
        "import json from \"./foo.json\" assert { type: \"json\" };"
    );
    Ok(())
}
#[test]
fn test_import_attributes_dynamic_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_dynamic_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_dynamic_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_dynamic_js_format_1_6a8fb020() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import(\"./foo.json\", { with: { type: \"json\" } });")?;
    assert_eq!(
        formatted,
        "import(\"./foo.json\", { with: { type: \"json\" } });"
    );
    Ok(())
}
#[test]
fn test_import_attributes_static_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_static_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_static_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_attributes_static_js_format_1_d213dee3() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import json from \"./foo.json\" with { type: \"json\" };")?;
    assert_eq!(
        formatted,
        "import json from \"./foo.json\" with { type: \"json\" };"
    );
    Ok(())
}
#[test]
fn test_import_meta_js_format_1_4e73fb53() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-syntax-import-meta\n\n// Enabled by default https://github.com/babel/babel/pull/11406\n\n// from https://github.com/tc39/proposal-import-meta\n\n(async () => {\n  const response = await fetch(new URL(\"../hamsters.jpg\", import.meta.url));\n  const blob = await response.blob();\n\n  const size = import.meta.scriptElement.dataset.size || 300;\n\n  const image = new Image();\n  image.src = URL.createObjectURL(blob);\n  image.width = image.height = size;\n\n  document.body.appendChild(image);\n})();") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-syntax-import-meta\n\n// Enabled by default https://github.com/babel/babel/pull/11406\n\n// from https://github.com/tc39/proposal-import-meta\n\n(async () => {\n  const response = await fetch(new URL(\"../hamsters.jpg\", import.meta.url));\n  const blob = await response.blob();\n\n  const size = import.meta.scriptElement.dataset.size || 300;\n\n  const image = new Image();\n  image.src = URL.createObjectURL(blob);\n  image.width = image.height = size;\n\n  document.body.appendChild(image);\n})();");
    Ok(())
}
#[test]
fn test_import_reflection_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_import_reflection_js_format_1_ca41c905() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("import module foo from \"./module.wasm\";")?;
    assert_eq!(formatted, "import module foo from \"./module.wasm\";");
    Ok(())
}
#[test]
fn test_jsx_js_format_1_64779c95() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-syntax-jsx\n\nvar profile = <div>\n  <img src=\"avatar.png\" className=\"profile\" />\n  <h3>{[user.firstName, user.lastName].join(' ')}</h3>\n</div>;") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-syntax-jsx\n\nvar profile = (\n  <div>\n    <img src=\"avatar.png\" className=\"profile\" />\n    <h3>{[user.firstName, user.lastName].join(\" \")}</h3>\n  </div>\n);");
    Ok(())
}
#[test]
fn test_logical_assignment_operators_js_format_1_f8bed0dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-logical-assignment-operators\n\na ||= b;\nobj.a.b ||= c;\n\na &&= b;\nobj.a.b &&= c;") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-logical-assignment-operators\n\na ||= b;\nobj.a.b ||= c;\n\na &&= b;\nobj.a.b &&= c;");
    Ok(())
}
#[test]
fn test_module_blocks_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_module_blocks_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_module_blocks_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_module_blocks_js_format_1_fa259d8a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("let m = module {\n  export let m = 2;\n  export let n = 3;\n};")?;
    assert_eq!(
        formatted,
        "let m = module {\n  export let m = 2;\n  export let n = 3;\n};"
    );
    Ok(())
}
#[test]
fn test_module_string_names_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_module_string_names_js_format_1_3d911c85() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { \"😄\" as smile } from \"./emojis.js\";\nexport { smile as \"😄\" } from \"./emojis.js\";") ? ;
    assert_eq ! (formatted , "import { \"😄\" as smile } from \"./emojis.js\";\nexport { smile as \"😄\" } from \"./emojis.js\";");
    Ok(())
}
#[test]
fn test_nullish_coalescing_operator_js_format_1_948e24de() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-nullish-coalescing-operator\n\nvar foo = object.foo ?? \"default\";") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-nullish-coalescing-operator\n\nvar foo = object.foo ?? \"default\";");
    Ok(())
}
#[test]
fn test_numeric_separator_js_format_1_224c83bf() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-numeric-separator\n\nlet budget = 1_000_000_000_000;\n\n// What is the value of `budget`? It's 1 trillion!\n//\n// Let's confirm:\nconsole.log(budget === 10 ** 12); // true\n\nlet nibbles = 0b1010_0001_1000_0101;\n\n// Is bit 7 on? It sure is!\n// 0b1010_0001_1000_0101\n//             ^\n//\n// We can double check:\nconsole.log(!!(nibbles & (1 << 7))); // true\n\n// Messages are sent as 24 bit values, but should be\n// treated as 3 distinct bytes:\nlet message = 0xa0_b0_c0;\n\n// What's the value of the upper most byte? It's A0, or 160.\n// We can confirm that:\nlet a = (message >> 16) & 0xff;\nconsole.log(a.toString(16), a); // a0, 160\n\n// What's the value of the middle byte? It's B0, or 176.\n// Let's just make sure...\nlet b = (message >> 8) & 0xff;\nconsole.log(b.toString(16), b); // b0, 176\n\n// What's the value of the lower most byte? It's C0, or 192.\n// Again, let's prove that:\nlet c = message & 0xff;\nconsole.log(c.toString(16), b); // c0, 192") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-numeric-separator\n\nlet budget = 1_000_000_000_000;\n\n// What is the value of `budget`? It's 1 trillion!\n//\n// Let's confirm:\nconsole.log(budget === 10 ** 12); // true\n\nlet nibbles = 0b1010_0001_1000_0101;\n\n// Is bit 7 on? It sure is!\n// 0b1010_0001_1000_0101\n//             ^\n//\n// We can double check:\nconsole.log(!!(nibbles & (1 << 7))); // true\n\n// Messages are sent as 24 bit values, but should be\n// treated as 3 distinct bytes:\nlet message = 0xa0_b0_c0;\n\n// What's the value of the upper most byte? It's A0, or 160.\n// We can confirm that:\nlet a = (message >> 16) & 0xff;\nconsole.log(a.toString(16), a); // a0, 160\n\n// What's the value of the middle byte? It's B0, or 176.\n// Let's just make sure...\nlet b = (message >> 8) & 0xff;\nconsole.log(b.toString(16), b); // b0, 176\n\n// What's the value of the lower most byte? It's C0, or 192.\n// Again, let's prove that:\nlet c = message & 0xff;\nconsole.log(c.toString(16), b); // c0, 192");
    Ok(())
}
#[test]
fn test_object_rest_spread_js_format_1_a7112c54() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-transform-object-rest-spread\n\nlet { x, y, ...z } = { x: 1, y: 2, a: 3, b: 4 };\nconsole.log(x); // 1\nconsole.log(y); // 2\nconsole.log(z); // { a: 3, b: 4 }\n\nlet n = { x, y, ...z };\nconsole.log(n); // { x: 1, y: 2, a: 3, b: 4 }") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-transform-object-rest-spread\n\nlet { x, y, ...z } = { x: 1, y: 2, a: 3, b: 4 };\nconsole.log(x); // 1\nconsole.log(y); // 2\nconsole.log(z); // { a: 3, b: 4 }\n\nlet n = { x, y, ...z };\nconsole.log(n); // { x: 1, y: 2, a: 3, b: 4 }");
    Ok(())
}
#[test]
fn test_optional_catch_binding_js_format_1_bdaa17c0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-optional-catch-binding\n\ntry {\n  throw 0;\n} catch {\n  doSomethingWhichDoesNotCareAboutTheValueThrown();\n}\n\ntry {\n  throw 0;\n} catch {\n  doSomethingWhichDoesNotCareAboutTheValueThrown();\n} finally {\n  doSomeCleanup();\n}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-optional-catch-binding\n\ntry {\n  throw 0;\n} catch {\n  doSomethingWhichDoesNotCareAboutTheValueThrown();\n}\n\ntry {\n  throw 0;\n} catch {\n  doSomethingWhichDoesNotCareAboutTheValueThrown();\n} finally {\n  doSomeCleanup();\n}");
    Ok(())
}
#[test]
fn test_optional_chaining_js_format_1_93f285ed() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-optional-chaining\n\nconst obj = {\n  foo: {\n    bar: {\n      baz: 42,\n    },\n  },\n};\n\nconst baz = obj?.foo?.bar?.baz; // 42\n\nconst safe = obj?.qux?.baz; // undefined\n\n// Optional chaining and normal chaining can be intermixed\nobj?.foo.bar?.baz; // Only access `foo` if `obj` exists, and `baz` if\n                   // `bar` exists\n\n// Example usage with bracket notation:\nobj?.['foo']?.bar?.baz // 42\n\nconst obj2 = {\n  foo: {\n    bar: {\n      baz() {\n        return 42;\n      },\n    },\n  },\n};\n\nconst baz2 = obj?.foo?.bar?.baz(); // 42\n\nconst safe3 = obj?.qux?.baz(); // undefined\nconst safe4 = obj?.foo.bar.qux?.(); // undefined\n\nconst willThrow = obj?.foo.bar.qux(); // Error: not a function\n\n// Top function can be called directly, too.\nfunction test() {\n  return 42;\n}\ntest?.(); // 42\n\nexists?.(); // undefined\n\nconst obj3 = {\n  foo: {\n    bar: {\n      baz: class {\n      },\n    },\n  },\n};\n\nconst obj4 = {\n  foo: {\n    bar: {}\n  },\n};\n\nconst ret = delete obj?.foo?.bar?.baz; // true") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-optional-chaining\n\nconst obj = {\n  foo: {\n    bar: {\n      baz: 42,\n    },\n  },\n};\n\nconst baz = obj?.foo?.bar?.baz; // 42\n\nconst safe = obj?.qux?.baz; // undefined\n\n// Optional chaining and normal chaining can be intermixed\nobj?.foo.bar?.baz; // Only access `foo` if `obj` exists, and `baz` if\n// `bar` exists\n\n// Example usage with bracket notation:\nobj?.[\"foo\"]?.bar?.baz; // 42\n\nconst obj2 = {\n  foo: {\n    bar: {\n      baz() {\n        return 42;\n      },\n    },\n  },\n};\n\nconst baz2 = obj?.foo?.bar?.baz(); // 42\n\nconst safe3 = obj?.qux?.baz(); // undefined\nconst safe4 = obj?.foo.bar.qux?.(); // undefined\n\nconst willThrow = obj?.foo.bar.qux(); // Error: not a function\n\n// Top function can be called directly, too.\nfunction test() {\n  return 42;\n}\ntest?.(); // 42\n\nexists?.(); // undefined\n\nconst obj3 = {\n  foo: {\n    bar: {\n      baz: class {},\n    },\n  },\n};\n\nconst obj4 = {\n  foo: {\n    bar: {},\n  },\n};\n\nconst ret = delete obj?.foo?.bar?.baz; // true");
    Ok(())
}
#[test]
fn test_optional_chaining_assignment_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_optional_chaining_assignment_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_optional_chaining_assignment_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_optional_chaining_assignment_js_format_1_ed139e87() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("maybeAnObj?.prop = theValue;")?;
    assert_eq!(formatted, "maybeAnObj?.prop = theValue;");
    Ok(())
}
#[test]
fn test_partial_application_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_partial_application_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_partial_application_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_partial_application_js_format_1_f9d9d611() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-partial-application\n\nfunction add(x, y) { return x + y; }\n\nconst addOne = add(1, ?); // apply from the left\naddOne(2); // 3\n\nconst addTen = add(?, 10); // apply from the right\naddTen(2); // 12\n\nlet newScore = player.score\n  |> add(7, ?)\n  |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.\n\nf(x, ?)           // partial application from left\nf(?, x)           // partial application from right\nf(?, x, ?)        // partial application for any arg\no.f(x, ?)         // partial application from left\no.f(?, x)         // partial application from right\no.f(?, x, ?)      // partial application for any arg\nsuper.f(?)        // partial application allowed for call on |SuperProperty|") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-partial-application\n\nfunction add(x, y) {\n  return x + y;\n}\n\nconst addOne = add(1, ?); // apply from the left\naddOne(2); // 3\n\nconst addTen = add(?, 10); // apply from the right\naddTen(2); // 12\n\nlet newScore = player.score |> add(7, ?) |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.\n\nf(x, ?); // partial application from left\nf(?, x); // partial application from right\nf(?, x, ?); // partial application for any arg\no.f(x, ?); // partial application from left\no.f(?, x); // partial application from right\no.f(?, x, ?); // partial application for any arg\nsuper.f(?); // partial application allowed for call on |SuperProperty|");
    Ok(())
}
#[test]
fn test_pipeline_operator_fsharp_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_fsharp_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_fsharp_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_fsharp_js_format_1_07e5791c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/valtech-nyc/proposal-fsharp-pipelines\n\npromise\n  |> await\n  |> x => doubleSay(x, ', ')\n  |> capitalize\n  |> x => x + '!'\n  |> x => new User.Message(x)\n  |> x => stream.write(x)\n  |> await\n  |> console.log;\n\nconst result = exclaim(capitalize(doubleSay(\"hello\")));\nresult //=> \"Hello, hello!\"\n\nconst result = \"hello\"\n  |> doubleSay\n  |> capitalize\n  |> exclaim;\n\nresult //=> \"Hello, hello!\"\n\nconst person = { score: 25 };\n\nconst newScore = person.score\n  |> double\n  |> n => add(7, n)\n  |> n => boundScore(0, 100, n);\n\nnewScore //=> 57\n\n// As opposed to:\nlet newScore = boundScore(0, 100, add(7, double(person.score)));") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/valtech-nyc/proposal-fsharp-pipelines\n\npromise\n  |> await\n  |> (x) => doubleSay(x, \", \")\n  |> capitalize\n  |> (x) => x + \"!\"\n  |> (x) => new User.Message(x)\n  |> (x) => stream.write(x)\n  |> await\n  |> console.log;\n\nconst result = exclaim(capitalize(doubleSay(\"hello\")));\nresult; //=> \"Hello, hello!\"\n\nconst result = \"hello\" |> doubleSay |> capitalize |> exclaim;\n\nresult; //=> \"Hello, hello!\"\n\nconst person = { score: 25 };\n\nconst newScore =\n  person.score |> double |> (n) => add(7, n) |> (n) => boundScore(0, 100, n);\n\nnewScore; //=> 57\n\n// As opposed to:\nlet newScore = boundScore(0, 100, add(7, double(person.score)));");
    Ok(())
}
#[test]
fn test_pipeline_operator_hack_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_hack_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_hack_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_hack_js_format_1_0d38481a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/js-choi/proposal-hack-pipes\n\nreturn list\n |> take(prefix.length, %)\n |> equals(%, prefix);\n\n// (The % token isn't final; it might instead be @ or ? or #.)") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/js-choi/proposal-hack-pipes\n\nreturn list |> take(prefix.length, %) |> equals(%, prefix);\n\n// (The % token isn't final; it might instead be @ or ? or #.)");
    Ok(())
}
#[test]
fn test_pipeline_operator_minimal_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_minimal_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_minimal_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_pipeline_operator_minimal_js_format_1_7e468963() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/tc39/proposal-pipeline-operator/\n\nlet result = exclaim(capitalize(doubleSay(\"hello\")));\nresult //=> \"Hello, hello!\"\n\nlet result = \"hello\"\n  |> doubleSay\n  |> capitalize\n  |> exclaim;\n\nresult //=> \"Hello, hello!\"") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator\n// https://github.com/tc39/proposal-pipeline-operator/\n\nlet result = exclaim(capitalize(doubleSay(\"hello\")));\nresult; //=> \"Hello, hello!\"\n\nlet result = \"hello\" |> doubleSay |> capitalize |> exclaim;\n\nresult; //=> \"Hello, hello!\"");
    Ok(())
}
#[test]
fn test_private_fields_in_in_js_format_1_890e6a97() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/tc39/proposal-private-fields-in-in\n\nclass C {\n  #brand;\n\n  static isC(obj) {\n    try {\n      obj.#brand;\n      return true;\n    } catch {\n      return false;\n    }\n  }\n}\n\nclass C2 {\n  #data = null; // populated later\n\n  get #getter() {\n    if (!this.#data) {\n      throw new Error('no data yet!');\n    }\n    return this.#data;\n  }\n\n  static isC(obj) {\n    try {\n      obj.#getter;\n      return true;\n    } catch {\n      return false; // oops! might have gotten here because `#getter` threw :-(\n    }\n  }\n}\n\nclass C3 {\n  #brand;\n\n  #method() {}\n\n  get #getter() {}\n\n  static isC(obj) {\n    return #brand in obj && #method in obj && #getter in obj;\n  }\n}\n\n// Invalid https://github.com/tc39/proposal-private-fields-in-in#try-statement\n// class C {\n//   #brand;\n\n//   static isC(obj) {\n//     return try obj.#brand;\n//   }\n// }") ? ;
    assert_eq ! (formatted , "// https://github.com/tc39/proposal-private-fields-in-in\n\nclass C {\n  #brand;\n\n  static isC(obj) {\n    try {\n      obj.#brand;\n      return true;\n    } catch {\n      return false;\n    }\n  }\n}\n\nclass C2 {\n  #data = null; // populated later\n\n  get #getter() {\n    if (!this.#data) {\n      throw new Error(\"no data yet!\");\n    }\n    return this.#data;\n  }\n\n  static isC(obj) {\n    try {\n      obj.#getter;\n      return true;\n    } catch {\n      return false; // oops! might have gotten here because `#getter` threw :-(\n    }\n  }\n}\n\nclass C3 {\n  #brand;\n\n  #method() {}\n\n  get #getter() {}\n\n  static isC(obj) {\n    return #brand in obj && #method in obj && #getter in obj;\n  }\n}\n\n// Invalid https://github.com/tc39/proposal-private-fields-in-in#try-statement\n// class C {\n//   #brand;\n\n//   static isC(obj) {\n//     return try obj.#brand;\n//   }\n// }");
    Ok(())
}
#[test]
fn test_private_methods_js_format_1_e9581d9e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-private-methods\n\n// Test for `classPrivateProperties` and `classPrivateMethods`\n\nclass Counter extends HTMLElement {\n  #xValue = 0;\n  #render() {}\n\n  get #x() { return this.#xValue; }\n  set #x(value) {\n    this.#xValue = value;\n    window.requestAnimationFrame(\n      this.#render.bind(this));\n  }\n\n  #clicked() {\n    this.#x++;\n  }\n}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-private-methods\n\n// Test for `classPrivateProperties` and `classPrivateMethods`\n\nclass Counter extends HTMLElement {\n  #xValue = 0;\n  #render() {}\n\n  get #x() {\n    return this.#xValue;\n  }\n  set #x(value) {\n    this.#xValue = value;\n    window.requestAnimationFrame(this.#render.bind(this));\n  }\n\n  #clicked() {\n    this.#x++;\n  }\n}");
    Ok(())
}
#[test]
fn test_record_tuple_record_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_record_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_record_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_record_js_format_1_e95ad044() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const record1 = #{\n    a: 1,\n    b: 2,\n    c: 3,\n};\n\nconst record2 = #{...record1, b: 5};") ? ;
    assert_eq ! (formatted , "const record1 = #{\n  a: 1,\n  b: 2,\n  c: 3,\n};\n\nconst record2 = #{ ...record1, b: 5 };");
    Ok(())
}
#[test]
fn test_record_tuple_tuple_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_tuple_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_tuple_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_record_tuple_tuple_js_format_1_ddd9b2bc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const tuple1 = #[1, 2, 3];")?;
    assert_eq!(formatted, "const tuple1 = #[1, 2, 3];");
    Ok(())
}
#[test]
fn test_regex_v_flag_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regex_v_flag_js_format_1_b3692630() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/a/v;")?;
    assert_eq!(formatted, "/a/v;");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_regexp_modifiers_js_format_1_96351ebb() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const re = /^(?i:[a-z])[a-z]$/;")?;
    assert_eq!(formatted, "const re = /^(?i:[a-z])[a-z]$/;");
    Ok(())
}
#[test]
fn test_source_phase_imports_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_source_phase_imports_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_source_phase_imports_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_source_phase_imports_js_format_1_d427b224() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("import source fooSource from \"foo\";\nimport.source(\"x\");")?;
    assert_eq!(
        formatted,
        "import source fooSource from \"foo\";\nimport.source(\"x\");"
    );
    Ok(())
}
#[test]
fn test_throw_expressions_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expressions_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expressions_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_throw_expressions_js_format_1_47f42a6c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://babeljs.io/docs/en/babel-plugin-proposal-throw-expressions\n\nfunction test(param = throw new Error('required!')) {\n  const test = param === true || throw new Error('Falsy!');\n}") ? ;
    assert_eq ! (formatted , "// https://babeljs.io/docs/en/babel-plugin-proposal-throw-expressions\n\nfunction test(param = throw new Error(\"required!\")) {\n  const test = param === true || throw new Error(\"Falsy!\");\n}");
    Ok(())
}
#[test]
fn test_typescript_js_babel_estree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_typescript_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_typescript_js_babel_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_typescript_js_babel_flow_format_1_d9c6aa85() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// https://babeljs.io/docs/en/babel-preset-typescript\n\nconst x: number = 0;")?;
    assert_eq!(
        formatted,
        "// https://babeljs.io/docs/en/babel-preset-typescript\n\nconst x: number = 0;"
    );
    Ok(())
}
#[test]
fn test_typescript_js_babel_ts_format_1_d9c6aa85() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// https://babeljs.io/docs/en/babel-preset-typescript\n\nconst x: number = 0;")?;
    assert_eq!(
        formatted,
        "// https://babeljs.io/docs/en/babel-preset-typescript\n\nconst x: number = 0;"
    );
    Ok(())
}
#[test]
fn test_typescript_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_typescript_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_8_intrinsic_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_8_intrinsic_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_8_intrinsic_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_v_8_intrinsic_js_format_1_341d25e9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// https://github.com/babel/babel/pull/10148\n\n%DebugPrint(foo);\n\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-bind-expression/options.json\n// ::%DebugPrint(null)\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-member-expression/options.json\n// a.%DebugPrint();\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json\n// const i = %DebugPrint;\n// i(foo);\n\n// https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json\n// %DebugPrint?.(null)\n\nnew %DebugPrint(null);\n\nfunction *foo() {\n  yield %StringParseInt(\"42\", 10)\n}\n\nfoo%bar()") ? ;
    assert_eq ! (formatted , "// https://github.com/babel/babel/pull/10148\n\n%DebugPrint(foo);\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-bind-expression/options.json\n// ::%DebugPrint(null)\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-member-expression/options.json\n// a.%DebugPrint();\n\n// Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json\n// const i = %DebugPrint;\n// i(foo);\n\n// https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json\n// %DebugPrint?.(null)\n\nnew %DebugPrint(null);\n\nfunction* foo() {\n  yield %StringParseInt(\"42\", 10);\n}\n\nfoo % bar();");
    Ok(())
}
