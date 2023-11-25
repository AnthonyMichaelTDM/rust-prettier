#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_as_expression_ts_format_1_fe9b5b36() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = \\`\\${(foo + bar) as baz}\\`;\nconst b = \\`\\${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) as baz}\\`;\nconst b = \\`\\${(foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as baz}\\`;\nconst b = \\`\\${(foo + bar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}\\`;\nconst b = \\`\\${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}\\`;") ? ;
    assert_eq ! (formatted , "const a = \\`\\${(foo + bar) as baz}\\`;\nconst b = \\`\\${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) as baz\n}\\`;\nconst b = \\`\\${\n  (foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as baz\n}\\`;\nconst b = \\`\\${\n  (foo + bar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}\\`;\nconst b = \\`\\${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo +\n    veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}\\`;");
    Ok(())
}
#[test]
fn test_expressions_ts_format_1_3013da05() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar = tag<number>\\`but where will prettier wrap such a long tagged template literal? \\${foo.bar.baz} long long long long long long long long long long long long long long\\`") ? ;
    assert_eq ! (formatted , "const bar = tag<number>\\`but where will prettier wrap such a long tagged template literal? \\${foo.bar.baz} long long long long long long long long long long long long long long\\`;");
    Ok(())
}
