#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_typeapp_call_js_format_1_16f9fd2f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("//@flow\nf<T>();\nf<T><U></U>;\nnew C<T>;\nf<T>(e);\no[e]<T>();\nf<T>(x)<U>(y);\nasync <T>() => {};\nasync <T>(): T => {}\nnew C<T>(e);\nf<T>[e];\nnew C<T>();\no.m<T>();\nf<T>.0;\no?.m<T>(e);\no.m?.<T>(e);\nf?.<T>(e);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "//@flow\nf<T>();\nf < T > <U></U>;\nnew C<T>();\nf<T>(e);\no[e]<T>();\nf<T>(x)<U>(y);\nasync <T>() => {};\nasync <T>(): T => {};\nnew C<T>(e);\nf < T > [e];\nnew C<T>();\no.m<T>();\nf < T > 0.0;\no?.m<T>(e);\no.m?.<T>(e);\nf?.<T>(e);");
}
