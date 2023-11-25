#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attributes_html_semifalse_format_1_d45402bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n  </template>\n<template if:true={value.error}>\n    <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n</template>\n<template if:true={value.error}>\n  <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}></a>");
}
#[test]
fn test_attributes_html_trailing_commaes_5_format_1_d45402bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .trailing_comma("es5")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n  </template>\n<template if:true={value.error}>\n    <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n</template>\n<template if:true={value.error}>\n  <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}></a>");
}
#[test]
fn test_attributes_html_trailing_commanone_format_1_d45402bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n  </template>\n<template if:true={value.error}>\n    <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div\n    data-for={value}\n    data-for={value[0]}\n    data-for={value.toString()}\n    data-for={value()}\n    class=\"test\"\n  ></div>\n</template>\n<template if:true={value.error}>\n  <c-error-panel errors={value.error}></c-error-panel>\n</template>\n<a href=\"#\" onclick={aFunction}></a>");
}
