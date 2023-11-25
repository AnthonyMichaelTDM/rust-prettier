#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_dangling_ts_semifalse_format_1_7f70bc26() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "\nThing?.(/* dangling */);\ndeclare class Foo extends Qux<string> {/* dangling */}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "Thing?.(/* dangling */)\ndeclare class Foo extends Qux<string> {\n  /* dangling */\n}"
    );
}
#[test]
fn test_dangling_ts_format_1_7f70bc26() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "\nThing?.(/* dangling */);\ndeclare class Foo extends Qux<string> {/* dangling */}",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "Thing?.(/* dangling */);\ndeclare class Foo extends Qux<string> {\n  /* dangling */\n}"
    );
}
#[test]
fn test_issues_ts_semifalse_format_1_23eed1a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f(\n  someReallyLongArgument: WithSomeLongType,\n  someReallyLongArgument2: WithSomeLongType,\n  // Trailing comment should stay after\n) {}\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f(\n  someReallyLongArgument: WithSomeLongType,\n  someReallyLongArgument2: WithSomeLongType,\n  // Trailing comment should stay after\n) {}");
}
#[test]
fn test_issues_ts_format_1_23eed1a5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f(\n  someReallyLongArgument: WithSomeLongType,\n  someReallyLongArgument2: WithSomeLongType,\n  // Trailing comment should stay after\n) {}\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f(\n  someReallyLongArgument: WithSomeLongType,\n  someReallyLongArgument2: WithSomeLongType,\n  // Trailing comment should stay after\n) {}");
}
#[test]
fn test_last_arg_ts_semifalse_format_1_07d2f8bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type f1 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => number;\n\nf2 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number => {};\n\nf3 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => {};\n\nf4 = function(\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) {};\n\nclass X {\n  f(\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ) {}\n}\n\nfunction f5(\n  a: number\n// some comment here\n): number {\n  return a + 1;\n}\n\nvar x = {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {\n  }\n}\n\nclass X2 {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData = ['unknown']\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type f1 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => number\n\nf2 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number => {}\n\nf3 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => {}\n\nf4 = function (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) {}\n\nclass X {\n  f(\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ) {}\n}\n\nfunction f5(\n  a: number,\n  // some comment here\n): number {\n  return a + 1\n}\n\nvar x = {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData,\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {},\n}\n\nclass X2 {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData = [\"unknown\"],\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {}\n}");
}
#[test]
fn test_last_arg_ts_format_1_07d2f8bc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type f1 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => number;\n\nf2 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number => {};\n\nf3 = (\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => {};\n\nf4 = function(\n  currentRequest: {a: number},\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) {};\n\nclass X {\n  f(\n    currentRequest: {a: number},\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ) {}\n}\n\nfunction f5(\n  a: number\n// some comment here\n): number {\n  return a + 1;\n}\n\nvar x = {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {\n  }\n}\n\nclass X2 {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData = ['unknown']\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type f1 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => number;\n\nf2 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n): number => {};\n\nf3 = (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) => {};\n\nf4 = function (\n  currentRequest: { a: number },\n  // TODO this is a very very very very long comment that makes it go > 80 columns\n) {};\n\nclass X {\n  f(\n    currentRequest: { a: number },\n    // TODO this is a very very very very long comment that makes it go > 80 columns\n  ) {}\n}\n\nfunction f5(\n  a: number,\n  // some comment here\n): number {\n  return a + 1;\n}\n\nvar x = {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData,\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {},\n};\n\nclass X2 {\n  getSectionMode(\n    pageMetaData: PageMetaData,\n    sectionMetaData: SectionMetaData = [\"unknown\"],\n    /* $FlowFixMe This error was exposed while converting keyMirror\n     * to keyMirrorRecursive */\n  ): $Enum<SectionMode> {}\n}");
}
