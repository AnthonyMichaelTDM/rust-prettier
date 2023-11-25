#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_a_1_js_format_1_7be30da8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfunction foo(x: number): string { return 5; }\n\nfoo(0);\n\nmodule.exports = foo;") ? ;
    assert_eq ! (formatted , "// @flow\n\nfunction foo(x: number): string {\n  return 5;\n}\n\nfoo(0);\n\nmodule.exports = foo;");
    Ok(())
}
#[test]
fn test_a_2_js_format_1_7d8bf860() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nconst foo = require('./a1');\n\nmodule.exports = foo(\"\");")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst foo = require(\"./a1\");\n\nmodule.exports = foo(\"\");"
    );
    Ok(())
}
#[test]
fn test_a_3_js_format_1_468b2398() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nconst five = require('./a2');\n\n(five + five: string);")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst five = require(\"./a2\");\n\n(five + five: string);"
    );
    Ok(())
}
#[test]
fn test_b_0_js_format_1_a17e5154() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nclass C { x: C; }\nclass E { x: C; }\n\nmodule.exports = { C, E };")?;
    assert_eq!(
        formatted,
        "// @flow\n\nclass C {\n  x: C;\n}\nclass E {\n  x: C;\n}\n\nmodule.exports = { C, E };"
    );
    Ok(())
}
#[test]
fn test_b_1_js_format_1_8c084511() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport { C, E } from \"./b0\";\nfunction foo() { return C; }\nfunction bar() { return E; }\nlet X = foo();\nclass F extends X { }\nclass D extends F { }\nmodule.exports = { C, D };") ? ;
    assert_eq ! (formatted , "// @flow\n\nimport { C, E } from \"./b0\";\nfunction foo() {\n  return C;\n}\nfunction bar() {\n  return E;\n}\nlet X = foo();\nclass F extends X {}\nclass D extends F {}\nmodule.exports = { C, D };");
    Ok(())
}
#[test]
fn test_b_2_js_format_1_eadf049b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nmodule.exports = require(\"./b1\");")?;
    assert_eq!(formatted, "// @flow\n\nmodule.exports = require(\"./b1\");");
    Ok(())
}
#[test]
fn test_b_3_js_format_1_2e39fc19() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nimport { C, D } from \"./b2\";\n\n(new D: C);")?;
    assert_eq!(
        formatted,
        "// @flow\n\nimport { C, D } from \"./b2\";\n\n(new D(): C);"
    );
    Ok(())
}
#[test]
fn test_c_1_js_format_1_bf61046a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nexport function foo(props: { x: number }) { }")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport function foo(props: { x: number }) {}"
    );
    Ok(())
}
#[test]
fn test_c_2_js_format_1_1d5008a8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport { foo } from \"./c1\";\n\nexport function bar(props: { x: number }) {\n  foo({ x: 0 });\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nimport { foo } from \"./c1\";\n\nexport function bar(props: { x: number }) {\n  foo({ x: 0 });\n}");
    Ok(())
}
#[test]
fn test_c_3_js_format_1_dbd0181e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nimport { bar } from \"./c2\";\n\nbar({ x: 0 });")?;
    assert_eq!(
        formatted,
        "// @flow\n\nimport { bar } from \"./c2\";\n\nbar({ x: 0 });"
    );
    Ok(())
}
#[test]
fn test_d_1_js_format_1_6f8ad4a1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nexport class A {}\nexport class B {}\nexport var x = new A;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport class A {}\nexport class B {}\nexport var x = new A();"
    );
    Ok(())
}
#[test]
fn test_d_2_js_format_1_0b5a8ea9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nimport {A, x} from \"./d1\";\nexport var y: A = x;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nimport { A, x } from \"./d1\";\nexport var y: A = x;"
    );
    Ok(())
}
#[test]
fn test_e_1_js_format_1_6743ab08() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nexport type Action =\n  | { type: 'FOO' }\n  | { type: 'BAR' }\n;\n\nexport const LIFE = 42;") ? ;
    assert_eq ! (formatted , "// @flow\n\nexport type Action = { type: \"FOO\" } | { type: \"BAR\" };\n\nexport const LIFE = 42;");
    Ok(())
}
#[test]
fn test_e_2_js_format_1_1b9ad34c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport type { Action } from './e1';\n\nconst f = (): Action => {\n  return { type: 'FOO' };\n}\n\nimport { LIFE } from './e1';\n\n(LIFE: 42);") ? ;
    assert_eq ! (formatted , "// @flow\n\nimport type { Action } from \"./e1\";\n\nconst f = (): Action => {\n  return { type: \"FOO\" };\n};\n\nimport { LIFE } from \"./e1\";\n\n(LIFE: 42);");
    Ok(())
}
#[test]
fn test_f_1_js_format_1_5fa14acc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\ntype T = { x: number };\ntype S = { x: string };\n\ndeclare var a: T;\ndeclare var b: S;\ndeclare var c: T;\n\nmodule.exports = { a, b, c };") ? ;
    assert_eq ! (formatted , "// @flow\n\ntype T = { x: number };\ntype S = { x: string };\n\ndeclare var a: T;\ndeclare var b: S;\ndeclare var c: T;\n\nmodule.exports = { a, b, c };");
    Ok(())
}
#[test]
fn test_f_2_js_format_1_46134764() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nvar { a, b, c } = require('./f1');\n(c: { x: number });")?;
    assert_eq!(
        formatted,
        "// @flow\n\nvar { a, b, c } = require(\"./f1\");\n(c: { x: number });"
    );
    Ok(())
}
#[test]
fn test_g_1_js_format_1_dc39ee2f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nexport class C { }")?;
    assert_eq!(formatted, "// @flow\n\nexport class C {}");
    Ok(())
}
#[test]
fn test_g_2_js_format_1_d782c94a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// @flow\n\nimport { C } from './g1';\n\nclass D extends C { }\n\nmodule.exports = { D };",
    )?;
    assert_eq ! (formatted , "// @flow\n\nimport { C } from \"./g1\";\n\nclass D extends C {}\n\nmodule.exports = { D };");
    Ok(())
}
#[test]
fn test_g_3_js_format_1_e96e7e1f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nimport { C } from './g1';\nimport { D } from './g2';\n\n(new D: C)")?;
    assert_eq!(
        formatted,
        "// @flow\n\nimport { C } from \"./g1\";\nimport { D } from \"./g2\";\n\n(new D(): C);"
    );
    Ok(())
}
#[test]
fn test_h_1_js_format_1_be68019c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nexport type Foo = number;")?;
    assert_eq!(formatted, "// @flow\n\nexport type Foo = number;");
    Ok(())
}
#[test]
fn test_h_2_js_format_1_55bbc7a1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("// @flow\n\nimport type { Foo } from './h1';")?;
    assert_eq!(formatted, "// @flow\n\nimport type { Foo } from \"./h1\";");
    Ok(())
}
#[test]
fn test_i_1_js_format_1_ab883f57() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nconst foo: { p: number } = { p: 0 };\n\nmodule.exports = foo;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst foo: { p: number } = { p: 0 };\n\nmodule.exports = foo;"
    );
    Ok(())
}
#[test]
fn test_i_2_js_format_1_8961d38e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nconst foo = require('./i1');\n\nfoo.p = 0;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst foo = require(\"./i1\");\n\nfoo.p = 0;"
    );
    Ok(())
}
#[test]
fn test_j_1_js_format_1_f65d2607() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nconst foo: { [string]: number } = {};\n\nmodule.exports = foo;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst foo: { [string]: number } = {};\n\nmodule.exports = foo;"
    );
    Ok(())
}
#[test]
fn test_j_2_js_format_1_3d2f3a75() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("// @flow\n\nconst foo = require('./j1');\n\nfoo.p = 0;")?;
    assert_eq!(
        formatted,
        "// @flow\n\nconst foo = require(\"./j1\");\n\nfoo.p = 0;"
    );
    Ok(())
}
#[test]
fn test_k_js_format_1_e3121712() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("/* @flow */")?;
    assert_eq!(formatted, "/* @flow */");
    Ok(())
}
