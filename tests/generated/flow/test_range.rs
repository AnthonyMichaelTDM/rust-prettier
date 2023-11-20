#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_export_declaration_js_format_1_8060239f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(277)
        .print_width(80)
        .range_start(273)
        .parsers(vec!["flow"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | declare export function graphql<Props, Variables, Component: React$ComponentType<Props>>\n  2 |   (query: GQLDocument, config?: Config<Props, QueryConfigOptions<Variables>>):\n  3 |   (Component: Component) => React$ComponentType<$Diff<React$ElementConfig<Component>, {\n> 4 |     data: Object|void,\n    |                  ^^^^\n  5 |     mutate: Function|void\n  6 |   }>>\n  7 |\n  8 | declare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare export function graphql<\n  Props,\n  Variables,\n  Component: React$ComponentType<Props>,\n>(\n  query: GQLDocument,\n  config?: Config<Props, QueryConfigOptions<Variables>>,\n): (Component: Component) => React$ComponentType<\n  $Diff<\n    React$ElementConfig<Component>,\n    {\n      data: Object | void,\n      mutate: Function | void,\n    },\n  >,\n>;\n\ndeclare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"");
}
#[test]
fn test_type_parameter_declaration_js_format_1_72504649() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(39)
        .print_width(80)
        .parsers(vec!["flow"])
        .range_end(48)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | declare export function graphql<Props, Variables, Component: React$ComponentType<Props>>\n    |                                        ^^^^^^^^^\n  2 |   (query: GQLDocument, config?: Config<Props, QueryConfigOptions<Variables>>):\n  3 |   (Component: Component) => React$ComponentType<$Diff<React$ElementConfig<Component>, {\n  4 |     data: Object|void,\n  5 |     mutate: Function|void\n  6 |   }>>\n  7 |\n  8 | declare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare export function graphql<\n  Props,\n  Variables,\n  Component: React$ComponentType<Props>,\n>(\n  query: GQLDocument,\n  config?: Config<Props, QueryConfigOptions<Variables>>,\n): (Component: Component) => React$ComponentType<\n  $Diff<\n    React$ElementConfig<Component>,\n    {\n      data: Object | void,\n      mutate: Function | void,\n    },\n  >,\n>;\n\ndeclare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"");
}
