#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_declare_export_declaration_js_format_1_8060239f() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .range_end(277)
        .range_start(273)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | declare export function graphql<Props, Variables, Component: React$ComponentType<Props>>\n  2 |   (query: GQLDocument, config?: Config<Props, QueryConfigOptions<Variables>>):\n  3 |   (Component: Component) => React$ComponentType<$Diff<React$ElementConfig<Component>, {\n> 4 |     data: Object|void,\n    |                  ^^^^\n  5 |     mutate: Function|void\n  6 |   }>>\n  7 |\n  8 | declare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"\n  9 ") ? ;
    assert_eq ! (formatted , "declare export function graphql<\n  Props,\n  Variables,\n  Component: React$ComponentType<Props>,\n>(\n  query: GQLDocument,\n  config?: Config<Props, QueryConfigOptions<Variables>>,\n): (Component: Component) => React$ComponentType<\n  $Diff<\n    React$ElementConfig<Component>,\n    {\n      data: Object | void,\n      mutate: Function | void,\n    },\n  >,\n>;\n\ndeclare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"");
    Ok(())
}
#[test]
fn test_type_parameter_declaration_js_format_1_72504649() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .range_end(48)
        .range_start(39)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | declare export function graphql<Props, Variables, Component: React$ComponentType<Props>>\n    |                                        ^^^^^^^^^\n  2 |   (query: GQLDocument, config?: Config<Props, QueryConfigOptions<Variables>>):\n  3 |   (Component: Component) => React$ComponentType<$Diff<React$ElementConfig<Component>, {\n  4 |     data: Object|void,\n  5 |     mutate: Function|void\n  6 |   }>>\n  7 |\n  8 | declare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"\n  9 ") ? ;
    assert_eq ! (formatted , "declare export function graphql<\n  Props,\n  Variables,\n  Component: React$ComponentType<Props>,\n>(\n  query: GQLDocument,\n  config?: Config<Props, QueryConfigOptions<Variables>>,\n): (Component: Component) => React$ComponentType<\n  $Diff<\n    React$ElementConfig<Component>,\n    {\n      data: Object | void,\n      mutate: Function | void,\n    },\n  >,\n>;\n\ndeclare type FetchPolicy= \"cache-first\" | \"cache-and-network\" | \"network-only\" | \"cache-only\"");
    Ok(())
}
