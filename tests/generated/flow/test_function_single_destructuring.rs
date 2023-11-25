#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_24eef00f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function excludeFirstFiveResults4([first, second, third, fourth, fifth, ...rest]: Result[]) {\n  return rest;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function excludeFirstFiveResults4([\n  first,\n  second,\n  third,\n  fourth,\n  fifth,\n  ...rest\n]: Result[]) {\n  return rest;\n}");
}
#[test]
fn test_object_js_format_1_09662034() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function StatelessFunctionalComponent4({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}: MyType | null | void) {\n  return <div />\n}\n\nconst StatelessFunctionalComponent5 = ({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}: {\n  isActive: number,\n  onFiltersUpdated: number,\n  onSelect: number,\n  onSubmitAndDeselect: number,\n  onCancel: number,\n  searchFilters: number,\n  title: number,\n  items: number,\n}) => {\n  return <div />\n};\n\ntype T = ({\n  isActive: number,\n  onFiltersUpdated: number,\n  onSelect: number,\n  onSubmitAndDeselect: number,\n  onCancel: number,\n  searchFilters: number,\n  title: number,\n  items: number,\n}) => void;\n\nconst X = (props: {\n  a: boolean,\n}) =>\n  <A />;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function StatelessFunctionalComponent4({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}: MyType | null | void) {\n  return <div />;\n}\n\nconst StatelessFunctionalComponent5 = ({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}: {\n  isActive: number,\n  onFiltersUpdated: number,\n  onSelect: number,\n  onSubmitAndDeselect: number,\n  onCancel: number,\n  searchFilters: number,\n  title: number,\n  items: number,\n}) => {\n  return <div />;\n};\n\ntype T = ({\n  isActive: number,\n  onFiltersUpdated: number,\n  onSelect: number,\n  onSubmitAndDeselect: number,\n  onCancel: number,\n  searchFilters: number,\n  title: number,\n  items: number,\n}) => void;\n\nconst X = (props: { a: boolean }) => <A />;");
}
#[test]
fn test_object_type_in_declare_function_js_format_1_59daf8de() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare function foo(this: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bazFlip({ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...{ a: boolean, b: string, c: number }):\n  Promise<Array<foo>>\n\ndeclare function bar(...x: { a: boolean, b: string, c: number }):\n  Promise<Array<foo>>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare function foo(this: {\n  a: boolean,\n  b: string,\n  c: number,\n}): Promise<Array<foo>>;\n\ndeclare function bazFlip({\n  a: boolean,\n  b: string,\n  c: number,\n}): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...{ a: boolean, b: string, c: number }\n): Promise<Array<foo>>;\n\ndeclare function bar(\n  ...x: { a: boolean, b: string, c: number }\n): Promise<Array<foo>>;");
}
