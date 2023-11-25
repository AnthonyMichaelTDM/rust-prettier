#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_computed_optional_member_expression_ng_trailing_commanone_format_1_7e65cf7e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ng")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[\n  a?.[0]\n]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[a?.[0]");
}
#[test]
fn test_logical_expression_ng_trailing_commanone_format_1_e3f02610() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ng")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    advancedSearchService.patientInformationFieldsRow2 && advancedSearchService.patientInformationFieldsRow2.indexOf(advancedSearchService.formElementData.customFieldList[i].customFieldType) !== -1,\n  (x && y) ?? z\n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  advancedSearchService.patientInformationFieldsRow2 &&\n    advancedSearchService.patientInformationFieldsRow2.indexOf(\n      advancedSearchService.formElementData.customFieldList[i].customFieldType\n    ) !== -1,\n  (x && y) ?? z\n");
}
#[test]
fn test_optional_chaining_ng_trailing_commanone_format_1_72e856c4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ng")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[  a?.b[c], (a?.b)[c]  ]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "[a?.b[c], (a?.b)[c]");
}
#[test]
fn test_pipe_expression_ng_trailing_commanone_format_1_f75d9c6e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ng")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[\n    a ? (b | c : d) : (e | f : g),\n    a | b | c | d,\n    ((a | b) | c) | d,\n    a | b:(c | d),\n    { a: b | c },\n    (a + b) | c,\n    (a | b) + c,\n    fn(a | b),\n    a?.b(c | d),\n    a[b | c],\n    ($students | async).items,\n    ($students | async)(),\n    myData | myPipe:'arg1':'arg2':'arg3',\n    value\n      | pipeA: {\n        keyA: reallySuperLongValue,\n        keyB: shortValue | pipeB | pipeC: valueToPipeC\n      } : {\n        keyA: reallySuperLongValue,\n        keyB: shortValue | pipeB | pipeC: valueToPipeC\n      }\n      | aaa,\n   (hideLinqPanel ? \"ReportSelection.HideShowLabel_Show.String\" : \"ReportSelection.HideShowLabel_Hide.String\") | localize:(localizationSection) \n]") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[\n  a ? (b | c: d) : (e | f: g),\n  a | b | c | d,\n  a | b | c | d,\n  a | b: (c | d),\n  { a: b | c },\n  a + b | c,\n  (a | b) + c,\n  fn(a | b),\n  a?.b(c | d),\n  a[b | c],\n  ($students | async).items,\n  ($students | async)(),\n  myData | myPipe: \"arg1\" : \"arg2\" : \"arg3\",\n  value\n    | pipeA\n      : {\n          keyA: reallySuperLongValue,\n          keyB: shortValue | pipeB | pipeC: valueToPipeC\n        }\n      : {\n          keyA: reallySuperLongValue,\n          keyB: shortValue | pipeB | pipeC: valueToPipeC\n        }\n    | aaa,\n  (hideLinqPanel\n    ? \"ReportSelection.HideShowLabel_Show.String\"\n    : \"ReportSelection.HideShowLabel_Hide.String\"\n  ) | localize: localizationSection\n");
}
#[test]
fn test_pipe_in_object_ng_trailing_commanone_format_1_f1fbf16a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ng")
        .print_width(80)
        .trailing_comma("none")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[{ AngularJS: '1.x', 'color': ('#222' | darken)}]");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "[{ AngularJS: \"1.x\", color: (\"#222\" | darken) }"
    );
}
