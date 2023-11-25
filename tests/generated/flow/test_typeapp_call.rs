#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_typeapp_call_js_format_1_16f9fd2f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("//@flow\nf<T>();\nf<T><U></U>;\nnew C<T>;\nf<T>(e);\no[e]<T>();\nf<T>(x)<U>(y);\nasync <T>() => {};\nasync <T>(): T => {}\nnew C<T>(e);\nf<T>[e];\nnew C<T>();\no.m<T>();\nf<T>.0;\no?.m<T>(e);\no.m?.<T>(e);\nf?.<T>(e);") ? ;
    assert_eq ! (formatted , "//@flow\nf<T>();\nf < T > <U></U>;\nnew C<T>();\nf<T>(e);\no[e]<T>();\nf<T>(x)<U>(y);\nasync <T>() => {};\nasync <T>(): T => {};\nnew C<T>(e);\nf < T > [e];\nnew C<T>();\no.m<T>();\nf < T > 0.0;\no?.m<T>(e);\no.m?.<T>(e);\nf?.<T>(e);");
    Ok(())
}
