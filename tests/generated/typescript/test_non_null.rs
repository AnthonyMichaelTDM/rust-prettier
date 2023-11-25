use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_braces_ts_format_1_b92efe7d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const myFunction2 = (key: string): number =>\n  ({\n    a: 42,\n    b: 42,\n  }[key]!)\n\nconst myFunction3 = key => ({}!.a);\n\nconst f = ((a) => {log(a)})!;\n\nif (a) ({ a, ...b }.a())!.c();\n\n(function() {})!()\n\nclass a extends ({}!) {}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const myFunction2 = (key: string): number =>\n  ({\n    a: 42,\n    b: 42,\n  })[key]!;\n\nconst myFunction3 = (key) => ({})!.a;\n\nconst f = ((a) => {\n  log(a);\n})!;\n\nif (a) ({ a, ...b }).a()!.c();\n\n(function () {})!();\n\nclass a extends ({}!) {}");
}
#[test]
fn test_member_chain_ts_format_1_83719e1f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const { somePropThatHasAReallyLongName, anotherPropThatHasALongName } = this.props.imReallySureAboutThis!;\n\nconst { somePropThatHasAReallyLongName2, anotherPropThatHasALongName2 } = this.props.imReallySureAboutThis!.anotherObject;\n\nthis.foo.get(\"bar\")!.doThings().more();\n\nfoo!.bar().baz().what();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const { somePropThatHasAReallyLongName, anotherPropThatHasALongName } =\n  this.props.imReallySureAboutThis!;\n\nconst { somePropThatHasAReallyLongName2, anotherPropThatHasALongName2 } =\n  this.props.imReallySureAboutThis!.anotherObject;\n\nthis.foo.get(\"bar\")!.doThings().more();\n\nfoo!.bar().baz().what();");
}
#[test]
fn test_optional_chain_ts_format_1_63d8aa6b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("a?.b!.c;\na?.b!.c.d;\na?.b.c!.d;\na!.b?.c;\na?.b!?.c;\na?.b!.c?.c;\n(a?.b!).c;\n(a?.b)!.c;\n\na?.().b!.c;\na?.().b!.c.d;\na?.().b.c!.d;\na?.().b!?.c;\na?.().b!.c?.c;\n(a?.().b!).c;\n(a?.().b)!.c;\n\n(a?.b)![c?.d!]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "a?.b!.c;\na?.b!.c.d;\na?.b.c!.d;\na!.b?.c;\na?.b!?.c;\na?.b!.c?.c;\n(a?.b)!.c;\n(a?.b)!.c;\n\na?.().b!.c;\na?.().b!.c.d;\na?.().b.c!.d;\na?.().b!?.c;\na?.().b!.c?.c;\n(a?.().b)!.c;\n(a?.().b)!.c;\n\n(a?.b)![c?.d!];");
}
#[test]
fn test_parens_ts_format_1_93bf15b9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a ? b : c) ![tokenKey];\n(a || b) ![tokenKey];\n(void 0)!;\n\nasync function f() {\n    return (await foo())!;\n}\n\nfunction* g() {\n    return (yield * foo())!;\n}\n\nconst a = (b()!)(); // parens aren't necessary\nconst b = c!();\n\n// parens are necessary if the expression result is called as a constructor\nconst c1 = new (d()!)();\nconst c2 = new (d()!);\nconst c3 = new (d()!.e)();\nnew (x()\\`\\`.y!)();\nnew (x()\\`\\`!.y)();\nnew (x()!\\`\\`.y)();\nnew (x!()\\`\\`.y)();\n\nxyz.a(b!).a(b!).a(b!)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(a ? b : c)![tokenKey];\n(a || b)![tokenKey];\n(void 0)!;\n\nasync function f() {\n  return (await foo())!;\n}\n\nfunction* g() {\n  return (yield* foo())!;\n}\n\nconst a = b()!(); // parens aren't necessary\nconst b = c!();\n\n// parens are necessary if the expression result is called as a constructor\nconst c1 = new (d()!)();\nconst c2 = new (d()!)();\nconst c3 = new (d()!.e)();\nnew (x()\\`\\`.y!)();\nnew (x()\\`\\`!.y)();\nnew (x()!\\`\\`.y)();\nnew (x!()\\`\\`.y)();\n\nxyz.a(b!).a(b!).a(b!);");
}
