#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_example_html_format_1_24107d53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!--interpolations in html should be treated as normal text--><div>Fuga magnam facilis. Voluptatem quaerat porro.{{\n\n\nx => {\n    const hello = 'world'\n    return hello;\n}\n\n\n\n}} Magni consectetur in et molestias neque esse voluptatibus voluptas. {{  \n    \n    \n    some_variable \n\n\n\n}} Eum quia nihil nulla esse. Dolorem asperiores vero est error {{\n\n                    preserve\n\n                    invalid\n                    \n                    interpolation\n\n}} reprehenderit voluptates minus {{console.log(  short_interpolation )}} nemo.</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<!--interpolations in html should be treated as normal text-->\n<div>\n  Fuga magnam facilis. Voluptatem quaerat porro.{{ x => { const hello = 'world'\n  return hello; } }} Magni consectetur in et molestias neque esse voluptatibus\n  voluptas. {{ some_variable }} Eum quia nihil nulla esse. Dolorem asperiores\n  vero est error {{ preserve invalid interpolation }} reprehenderit voluptates\n  minus {{console.log( short_interpolation )}} nemo.\n</div>");
}
