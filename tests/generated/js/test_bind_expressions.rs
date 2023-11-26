#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_await_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_await_js_semifalse_format_1_b869cd94() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const doBothThings = async () => {\n    const request = doAsyncThing();\n    return (await request)::doSyncThing();\n};") ? ;
    assert_eq ! (formatted , "const doBothThings = async () => {\n  const request = doAsyncThing()\n  return (await request)::doSyncThing()\n}");
    Ok(())
}
#[test]
fn test_await_js_format_1_b869cd94() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const doBothThings = async () => {\n    const request = doAsyncThing();\n    return (await request)::doSyncThing();\n};") ? ;
    assert_eq ! (formatted , "const doBothThings = async () => {\n  const request = doAsyncThing();\n  return (await request)::doSyncThing();\n};");
    Ok(())
}
#[test]
fn test_bind_parens_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_bind_parens_js_semifalse_format_1_d3662bf1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a || b)::c;\na || (b::c);\n::obj.prop;\n(void 0)::func();\n(+0)::is(-0);\na::(b.c);\na::(b.c());\na::b.c();\na::(b.c()());\na::((b.c())());\na::(b.c())();\na::(b.c().d);\na::(c().d.e);\na::(b());\na::(b::c());\na::(b()::c);\na::(b().c::d);\na::(b.c::d);\na::(b::c.d);\na::(b.c::d::e);\na::(b::c::d);\na::(b::c::d.e);\na::((b::c::d).e);\na::(void 0);\na::(b.c()::d.e);\na::(b.c::d.e);\na::(b.c::d.e)::f.g;\nb.c::d.e;\n(b.c::d).e;\n(b::c::d).e;\nnew (a::b)();\nnew f(a::b);\nf[a::b];\nf[a::b()];") ? ;
    assert_eq ! (formatted , ";(a || b)::c\na || b::c\n;::obj.prop\n;(void 0)::func()\n;(+0)::is(-0)\na::b.c\na::(b.c())\na::b.c()\na::(b.c()())\na::(b.c()())\na::(b.c())()\na::(b.c().d)\na::(c().d.e)\na::(b())\na::(b::c())\na::(b()::c)\na::(b().c::d)\na::(b.c::d)\na::(b::c.d)\na::(b.c::d::e)\na::(b::c::d)\na::(b::c::d.e)\na::(b::c::d).e\na::(void 0)\na::(b.c()::d.e)\na::(b.c::d.e)\na::(b.c::d.e)::f.g\nb.c::d.e\n;(b.c::d).e\n;(b::c::d).e\nnew (a::b)()\nnew f(a::b)\nf[a::b]\nf[a::b()]");
    Ok(())
}
#[test]
fn test_bind_parens_js_format_1_d3662bf1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a || b)::c;\na || (b::c);\n::obj.prop;\n(void 0)::func();\n(+0)::is(-0);\na::(b.c);\na::(b.c());\na::b.c();\na::(b.c()());\na::((b.c())());\na::(b.c())();\na::(b.c().d);\na::(c().d.e);\na::(b());\na::(b::c());\na::(b()::c);\na::(b().c::d);\na::(b.c::d);\na::(b::c.d);\na::(b.c::d::e);\na::(b::c::d);\na::(b::c::d.e);\na::((b::c::d).e);\na::(void 0);\na::(b.c()::d.e);\na::(b.c::d.e);\na::(b.c::d.e)::f.g;\nb.c::d.e;\n(b.c::d).e;\n(b::c::d).e;\nnew (a::b)();\nnew f(a::b);\nf[a::b];\nf[a::b()];") ? ;
    assert_eq ! (formatted , "(a || b)::c;\na || b::c;\n::obj.prop;\n(void 0)::func();\n(+0)::is(-0);\na::b.c;\na::(b.c());\na::b.c();\na::(b.c()());\na::(b.c()());\na::(b.c())();\na::(b.c().d);\na::(c().d.e);\na::(b());\na::(b::c());\na::(b()::c);\na::(b().c::d);\na::(b.c::d);\na::(b::c.d);\na::(b.c::d::e);\na::(b::c::d);\na::(b::c::d.e);\na::(b::c::d).e;\na::(void 0);\na::(b.c()::d.e);\na::(b.c::d.e);\na::(b.c::d.e)::f.g;\nb.c::d.e;\n(b.c::d).e;\n(b::c::d).e;\nnew (a::b)();\nnew f(a::b);\nf[a::b];\nf[a::b()];");
    Ok(())
}
#[test]
fn test_long_name_method_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_name_method_js_semifalse_format_1_e18ecda8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  constructor() {\n    this.testLongNameMethodAndSomethingElseLallala = ::this.testLongNameMethodAndSomethingElseLallala;\n  }\n  \n  testLongNameMethodAndSomethingElseLallala() {\n    return true;\n  }\n") ? ;
    assert_eq ! (formatted , "class X {\n  constructor() {\n    this.testLongNameMethodAndSomethingElseLallala =\n      ::this.testLongNameMethodAndSomethingElseLallala\n  }\n\n  testLongNameMethodAndSomethingElseLallala() {\n    return true\n  }\n}");
    Ok(())
}
#[test]
fn test_long_name_method_js_format_1_e18ecda8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  constructor() {\n    this.testLongNameMethodAndSomethingElseLallala = ::this.testLongNameMethodAndSomethingElseLallala;\n  }\n  \n  testLongNameMethodAndSomethingElseLallala() {\n    return true;\n  }\n") ? ;
    assert_eq ! (formatted , "class X {\n  constructor() {\n    this.testLongNameMethodAndSomethingElseLallala =\n      ::this.testLongNameMethodAndSomethingElseLallala;\n  }\n\n  testLongNameMethodAndSomethingElseLallala() {\n    return true;\n  }\n}");
    Ok(())
}
#[test]
fn test_method_chain_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_method_chain_js_semifalse_format_1_4c8784d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {interval} from 'rxjs/observable/interval';\nimport {filter} from 'rxjs/operator/filter';\nimport {take} from 'rxjs/operator/take';\nimport {map} from 'rxjs/operator/map';\nimport {throttle} from 'rxjs/operator/throttle';\nimport {takeUntil} from 'rxjs/operator/takeUntil';\n\nfunction test(observable) {\n    return observable\n        ::filter(data => data.someTest)\n        ::throttle(() =>\n            interval(10)\n                ::take(1)\n                ::takeUntil(observable::filter(data => someOtherTest))\n        )\n        ::map(someFunction);\n}") ? ;
    assert_eq ! (formatted , "import { interval } from \"rxjs/observable/interval\"\nimport { filter } from \"rxjs/operator/filter\"\nimport { take } from \"rxjs/operator/take\"\nimport { map } from \"rxjs/operator/map\"\nimport { throttle } from \"rxjs/operator/throttle\"\nimport { takeUntil } from \"rxjs/operator/takeUntil\"\n\nfunction test(observable) {\n  return observable\n    ::filter((data) => data.someTest)\n    ::throttle(() =>\n      interval(10)\n        ::take(1)\n        ::takeUntil(observable::filter((data) => someOtherTest)),\n    )\n    ::map(someFunction)\n}");
    Ok(())
}
#[test]
fn test_method_chain_js_format_1_4c8784d9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import {interval} from 'rxjs/observable/interval';\nimport {filter} from 'rxjs/operator/filter';\nimport {take} from 'rxjs/operator/take';\nimport {map} from 'rxjs/operator/map';\nimport {throttle} from 'rxjs/operator/throttle';\nimport {takeUntil} from 'rxjs/operator/takeUntil';\n\nfunction test(observable) {\n    return observable\n        ::filter(data => data.someTest)\n        ::throttle(() =>\n            interval(10)\n                ::take(1)\n                ::takeUntil(observable::filter(data => someOtherTest))\n        )\n        ::map(someFunction);\n}") ? ;
    assert_eq ! (formatted , "import { interval } from \"rxjs/observable/interval\";\nimport { filter } from \"rxjs/operator/filter\";\nimport { take } from \"rxjs/operator/take\";\nimport { map } from \"rxjs/operator/map\";\nimport { throttle } from \"rxjs/operator/throttle\";\nimport { takeUntil } from \"rxjs/operator/takeUntil\";\n\nfunction test(observable) {\n  return observable\n    ::filter((data) => data.someTest)\n    ::throttle(() =>\n      interval(10)\n        ::take(1)\n        ::takeUntil(observable::filter((data) => someOtherTest)),\n    )\n    ::map(someFunction);\n}");
    Ok(())
}
#[test]
fn test_short_name_method_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_short_name_method_js_semifalse_format_1_f17408f0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  constructor() {\n    this.shortMethod = ::this.shortMethod;\n  }\n  \n  shortMethod() {\n    return true;\n  }\n") ? ;
    assert_eq ! (formatted , "class X {\n  constructor() {\n    this.shortMethod = ::this.shortMethod\n  }\n\n  shortMethod() {\n    return true\n  }\n}");
    Ok(())
}
#[test]
fn test_short_name_method_js_format_1_f17408f0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class X {\n  constructor() {\n    this.shortMethod = ::this.shortMethod;\n  }\n  \n  shortMethod() {\n    return true;\n  }\n") ? ;
    assert_eq ! (formatted , "class X {\n  constructor() {\n    this.shortMethod = ::this.shortMethod;\n  }\n\n  shortMethod() {\n    return true;\n  }\n}");
    Ok(())
}
#[test]
fn test_unary_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_semifalse_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_semifalse_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_semifalse_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_unary_js_semifalse_format_1_d59680a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("!x::y;\n!(x::y /* foo */);\n!(/* foo */ x::y);\n!(\n  /* foo */\n  x::y\n);\n!(\n  x::y\n  /* foo */\n);\n!(\n  x::y // foo\n);") ? ;
    assert_eq ! (formatted , "!x::y\n!(x::y /* foo */)\n!(/* foo */ x::y)\n!(\n  /* foo */\n  x::y\n)\n!(\n  x::y\n  /* foo */\n)\n!(\n  x::y // foo\n)");
    Ok(())
}
#[test]
fn test_unary_js_format_1_d59680a7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("!x::y;\n!(x::y /* foo */);\n!(/* foo */ x::y);\n!(\n  /* foo */\n  x::y\n);\n!(\n  x::y\n  /* foo */\n);\n!(\n  x::y // foo\n);") ? ;
    assert_eq ! (formatted , "!x::y;\n!(x::y /* foo */);\n!(/* foo */ x::y);\n!(\n  /* foo */\n  x::y\n);\n!(\n  x::y\n  /* foo */\n);\n!(\n  x::y // foo\n);");
    Ok(())
}
