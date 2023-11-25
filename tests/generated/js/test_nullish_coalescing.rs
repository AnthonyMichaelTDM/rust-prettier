#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_nullish_coalesing_operator_js_format_1_570fb933() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("obj.foo ?? \"default\";\n\nconst x = (foo, bar = foo ?? bar) => {};\n\nfoo ? bar ?? foo : baz;\n\nfoo ?? (bar ?? baz);\n(foo ?? bar) ?? baz;\n\n// Mixing ?? and (&& or ||) requires parens\n// It's a syntax error without it.\n(foo ?? baz) || baz;\nfoo ?? (baz || baz);\n\n(foo ?? baz) && baz;\nfoo ?? (baz && baz);\n\n(foo || baz) ?? baz;\nfoo || (baz ?? baz);\n\n(foo && baz) ?? baz;\nfoo && (baz ?? baz);") ? ;
    assert_eq ! (formatted , "obj.foo ?? \"default\";\n\nconst x = (foo, bar = foo ?? bar) => {};\n\nfoo ? bar ?? foo : baz;\n\nfoo ?? bar ?? baz;\nfoo ?? bar ?? baz;\n\n// Mixing ?? and (&& or ||) requires parens\n// It's a syntax error without it.\n(foo ?? baz) || baz;\nfoo ?? (baz || baz);\n\n(foo ?? baz) && baz;\nfoo ?? (baz && baz);\n\n(foo || baz) ?? baz;\nfoo || (baz ?? baz);\n\n(foo && baz) ?? baz;\nfoo && (baz ?? baz);");
    Ok(())
}
