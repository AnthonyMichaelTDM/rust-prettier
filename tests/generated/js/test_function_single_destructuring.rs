#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_9cb10500() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function excludeFirstFiveResults([first, second, third, fourth, fifth, ...rest]) {\n  return rest;\n}\n\nfunction excludeFirstFiveResults2([first, second, third, fourth, fifth, ...rest] = DEFAULT_FIVE_RESULTS) {\n  return rest;\n}\n\nfunction excludeFirstFiveResults3([firstResult, secondResult, thirdResult, fourthResult, fifthResult, ...rest] = [1, 2, 3, 4, 5]) {\n  return rest;\n}\n\nconst excludeFirstFiveResults5 = ([first, second, third, fourth, fifth, ...rest]) => {\n  return rest;\n}\n\nclass A {\n  excludeFirstFiveResults([first, second, third, fourth, fifth, ...restOfResults]) {\n    return restOfResults;\n  }\n}\n\npromise.then(([firstResult, secondResult, thirdResult, fourthResult, fifthResult, ...rest]) => {\n  return rest;\n});") ? ;
    assert_eq ! (formatted , "function excludeFirstFiveResults([\n  first,\n  second,\n  third,\n  fourth,\n  fifth,\n  ...rest\n]) {\n  return rest;\n}\n\nfunction excludeFirstFiveResults2([\n  first,\n  second,\n  third,\n  fourth,\n  fifth,\n  ...rest\n] = DEFAULT_FIVE_RESULTS) {\n  return rest;\n}\n\nfunction excludeFirstFiveResults3(\n  [\n    firstResult,\n    secondResult,\n    thirdResult,\n    fourthResult,\n    fifthResult,\n    ...rest\n  ] = [1, 2, 3, 4, 5],\n) {\n  return rest;\n}\n\nconst excludeFirstFiveResults5 = ([\n  first,\n  second,\n  third,\n  fourth,\n  fifth,\n  ...rest\n]) => {\n  return rest;\n};\n\nclass A {\n  excludeFirstFiveResults([\n    first,\n    second,\n    third,\n    fourth,\n    fifth,\n    ...restOfResults\n  ]) {\n    return restOfResults;\n  }\n}\n\npromise.then(\n  ([\n    firstResult,\n    secondResult,\n    thirdResult,\n    fourthResult,\n    fifthResult,\n    ...rest\n  ]) => {\n    return rest;\n  },\n);");
    Ok(())
}
#[test]
fn test_object_js_format_1_0acba3ae() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function StatelessFunctionalComponent({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}) {\n  return <div />\n}\n\nfunction StatelessFunctionalComponent2({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = '',\n  items = [],\n} = {}) {\n  return <div />\n}\n\nfunction StatelessFunctionalComponent3(\n  {\n    isActive,\n    onFiltersUpdated = () => null,\n    onSelect = () => null,\n    onSubmitAndDeselect = () => null,\n    onCancel = () => null,\n    searchFilters = null,\n    title = '',\n    items = [],\n  } = {\n    isActive: true\n  }\n) {\n  return <div />\n}\n\n\nclass C {\n  StatelessFunctionalComponent({\n    isActive,\n    onFiltersUpdated,\n    onSelect,\n    onSubmitAndDeselect,\n    onCancel,\n    searchFilters,\n    title,\n    items,\n  }) {\n    return <div />\n  }\n}") ? ;
    assert_eq ! (formatted , "function StatelessFunctionalComponent({\n  isActive,\n  onFiltersUpdated,\n  onSelect,\n  onSubmitAndDeselect,\n  onCancel,\n  searchFilters,\n  title,\n  items,\n}) {\n  return <div />;\n}\n\nfunction StatelessFunctionalComponent2({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = \"\",\n  items = [],\n} = {}) {\n  return <div />;\n}\n\nfunction StatelessFunctionalComponent3(\n  {\n    isActive,\n    onFiltersUpdated = () => null,\n    onSelect = () => null,\n    onSubmitAndDeselect = () => null,\n    onCancel = () => null,\n    searchFilters = null,\n    title = \"\",\n    items = [],\n  } = {\n    isActive: true,\n  },\n) {\n  return <div />;\n}\n\nclass C {\n  StatelessFunctionalComponent({\n    isActive,\n    onFiltersUpdated,\n    onSelect,\n    onSubmitAndDeselect,\n    onCancel,\n    searchFilters,\n    title,\n    items,\n  }) {\n    return <div />;\n  }\n}");
    Ok(())
}
#[test]
fn test_tuple_and_record_js_acorn_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_and_record_js_espree_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_and_record_js_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_and_record_js_meriyah_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_tuple_and_record_js_format_1_e4773a56() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function StatelessFunctionalComponent1({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = '',\n  items = [],\n} = {}) {\n}\n\nfunction StatelessFunctionalComponent2({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = '',\n  items = [],\n} = #{}) {\n}\n\nfunction StatelessFunctionalComponent3([\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = '',\n  items = [],\n] = []) {\n}\n\nfunction StatelessFunctionalComponent4([\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = '',\n  items = [],\n] = #[]) {\n}") ? ;
    assert_eq ! (formatted , "function StatelessFunctionalComponent1({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = \"\",\n  items = [],\n} = {}) {}\n\nfunction StatelessFunctionalComponent2({\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = \"\",\n  items = [],\n} = #{}) {}\n\nfunction StatelessFunctionalComponent3([\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = \"\",\n  items = [],\n] = []) {}\n\nfunction StatelessFunctionalComponent4([\n  isActive = true,\n  onFiltersUpdated = () => null,\n  onSelect = () => null,\n  onSubmitAndDeselect = () => null,\n  onCancel = () => null,\n  searchFilters = null,\n  title = \"\",\n  items = [],\n] = #[]) {}");
    Ok(())
}
