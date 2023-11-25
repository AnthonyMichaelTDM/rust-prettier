#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_as_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_as_js_format_1_73341263() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst name = (description as Description).name || (description as string);\nthis.isTabActionBar((e.target || e.srcElement) as HTMLElement);\n(originalError ? wrappedError(errMsg, originalError) : Error(errMsg)) as InjectionError;\n'current' in (props.pagination as {...});\n('current' in props.pagination) as {...};\nstart + (yearSelectTotal as number);\n(start + yearSelectTotal) as number;\nscrollTop > (visibilityHeight as number);\n(scrollTop > visibilityHeight) as number;\nexport default class Column<T> extends (RcTable.Column as Long.Thing<ColumnProps<T>,ColumnProps<T>,ColumnProps<T>,ColumnProps<T>>) {}\n({}) as {};\nfunction*g() {\n  const test = (yield 'foo') as number;\n}\nasync function g1() {\n  const test = (await 'foo') as number;\n}\n({}) as X;\n() => ({}) as X;\nconst state = JSON.stringify({\n  next: window.location.href,\n  nonce,\n} as State);\n\n(bValue as boolean) ? 0 : -1;\n\nconst value1 = thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;\nconst value2 = thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;\nconst value3 = thisIsAReallyLongIdentifier as (SomeInterface | SomeOtherInterface);\nconst value4 = thisIsAReallyLongIdentifier as { prop1: string, prop2: number, prop3: number }[];\nconst value5 = thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [string, number];\n\nconst iter1 = createIterator(this.controller, child, this.tag as BlahFunctionComponent);\nconst iter2 = createIterator(self.controller, child, self.tag as BlahFunctionComponent);\n\nx as any as T;\n\n(type) as T;") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst name = (description as Description).name || (description as string);\nthis.isTabActionBar((e.target || e.srcElement) as HTMLElement);\n(originalError\n  ? wrappedError(errMsg, originalError)\n  : Error(errMsg)) as InjectionError;\n\"current\" in (props.pagination as { ... });\n(\"current\" in props.pagination) as { ... };\nstart + (yearSelectTotal as number);\n(start + yearSelectTotal) as number;\nscrollTop > (visibilityHeight as number);\n(scrollTop > visibilityHeight) as number;\nexport default class Column<T> extends (RcTable.Column as Long.Thing<\n  ColumnProps<T>,\n  ColumnProps<T>,\n  ColumnProps<T>,\n  ColumnProps<T>,\n>) {}\n({}) as {};\nfunction* g() {\n  const test = (yield \"foo\") as number;\n}\nasync function g1() {\n  const test = (await \"foo\") as number;\n}\n({}) as X;\n() => ({}) as X;\nconst state = JSON.stringify({\n  next: window.location.href,\n  nonce,\n} as State);\n\n(bValue as boolean) ? 0 : -1;\n\nconst value1 =\n  thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;\nconst value2 =\n  thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;\nconst value3 = thisIsAReallyLongIdentifier as\n  | SomeInterface\n  | SomeOtherInterface;\nconst value4 = thisIsAReallyLongIdentifier as {\n  prop1: string,\n  prop2: number,\n  prop3: number,\n}[];\nconst value5 =\n  thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [\n    string,\n    number,\n  ];\n\nconst iter1 = createIterator(\n  this.controller,\n  child,\n  this.tag as BlahFunctionComponent,\n);\nconst iter2 = createIterator(\n  self.controller,\n  child,\n  self.tag as BlahFunctionComponent,\n);\n\nx as any as T;\n\n(type) as T;");
    Ok(())
}
#[test]
fn test_as_const_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_as_const_js_format_1_c0b5be18() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nexport const LOG_LEVEL = {\n    EMERGENCY: 0,\n    ALERT: 1,\n    CRITICAL: 2,\n    ERROR: 3,\n    WARNING: 4,\n    NOTICE: 5,\n    INFO: 6,\n    DEBUG: 7,\n} as const;\n\n(type) as const;") ? ;
    assert_eq ! (formatted , "// @flow\n\nexport const LOG_LEVEL = {\n  EMERGENCY: 0,\n  ALERT: 1,\n  CRITICAL: 2,\n  ERROR: 3,\n  WARNING: 4,\n  NOTICE: 5,\n  INFO: 6,\n  DEBUG: 7,\n} as const;\n\n(type) as const;");
    Ok(())
}
#[test]
fn test_assignment_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_assignment_js_format_1_76154220() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst TYPE_MAP = {\n    'character device': 'special',\n    'character special file': 'special',\n    directory: 'directory',\n    'regular file': 'file',\n    socket: 'socket',\n    'symbolic link': 'link',\n} as Foo;\n\nthis.previewPlayerHandle = (setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) as any);\n\nthis.intervalID = (setInterval(() => {\n  self.step();\n}, 30) as any);") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst TYPE_MAP = {\n  \"character device\": \"special\",\n  \"character special file\": \"special\",\n  directory: \"directory\",\n  \"regular file\": \"file\",\n  socket: \"socket\",\n  \"symbolic link\": \"link\",\n} as Foo;\n\nthis.previewPlayerHandle = setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) as any;\n\nthis.intervalID = setInterval(() => {\n  self.step();\n}, 30) as any;");
    Ok(())
}
#[test]
fn test_export_default_as_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_export_default_as_js_format_1_3027c287() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("// @flow\n\nexport default (function log() {} as typeof console.log)")?;
    assert_eq!(
        formatted,
        "// @flow\n\nexport default (function log() {} as typeof console.log);"
    );
    Ok(())
}
#[test]
fn test_long_identifiers_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_long_identifiers_js_format_1_b9f0d436() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst bifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;\n\naverredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =\n  annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;\n\naverredBathersBoxroomBuggyNurl = {\n  anodyneCondosMalateOverateRetinol:\n    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave\n};\n\naverredBathersBoxroomBuggyNurl(\n  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as\n    kochabCooieGameOnOboleUnweave\n);") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst bifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;\n\naverredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =\n  annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;\n\naverredBathersBoxroomBuggyNurl = {\n  anodyneCondosMalateOverateRetinol:\n    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,\n};\n\naverredBathersBoxroomBuggyNurl(\n  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,\n);");
    Ok(())
}
#[test]
fn test_nested_await_and_as_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_nested_await_and_as_js_format_1_d1acd537() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst getAccountCount = async () =>\n  (await\n    ((await (\n      await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n    ).findItem(\"My bookmarks\")) as TreeItem\n  ).getChildren()\n  ).length") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst getAccountCount = async () =>\n  (\n    await (\n      (await (\n        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n      ).findItem(\"My bookmarks\")) as TreeItem\n    ).getChildren()\n  ).length;");
    Ok(())
}
#[test]
fn test_return_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_return_js_format_1_770b7771() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "// @flow\n\nfunction foo() {\n  return {\n    foo: 1,\n    bar: 2,\n  } as Foo;\n}",
    )?;
    assert_eq!(
        formatted,
        "// @flow\n\nfunction foo() {\n  return {\n    foo: 1,\n    bar: 2,\n  } as Foo;\n}"
    );
    Ok(())
}
#[test]
fn test_satisfies_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_satisfies_js_format_1_a4c5a4f5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nconst x = y satisfies T;\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = (type: 'foo' | 'bar') => {\nswitch (type) {\n  case 'foo':\n    return 1;\n  case 'bar':\n    return 2;\n  default:\n    // exhaustiveness check idiom\n    (type) satisfies empty;\n    throw new Error('unreachable');\n}\n}\n\nfunction needParens() {\n(let) satisfies mixed;\n(interface) satisfies mixed;\n(module) satisfies mixed;\n(using) satisfies mixed;\n(yield) satisfies mixed;\n(await) satisfies mixed;\n}\n\nfunction noNeedParens() {\nasync satisfies mixed;\nsatisfies satisfies mixed;\nas satisfies mixed;\nopaque satisfies mixed;\n\nabc satisfies mixed; // not a keyword\n}\n\nfunction satisfiesChain() {\nsatisfies satisfies satisfies satisfies satisfies;\n(type) satisfies empty satisfies mixed;\n}") ? ;
    assert_eq ! (formatted , "// @flow\n\nconst x = y satisfies T;\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = (type: \"foo\" | \"bar\") => {\n  switch (type) {\n    case \"foo\":\n      return 1;\n    case \"bar\":\n      return 2;\n    default:\n      // exhaustiveness check idiom\n      (type) satisfies empty;\n      throw new Error(\"unreachable\");\n  }\n};\n\nfunction needParens() {\n  (let) satisfies mixed;\n  (interface) satisfies mixed;\n  (module) satisfies mixed;\n  (using) satisfies mixed;\n  (yield) satisfies mixed;\n  (await) satisfies mixed;\n}\n\nfunction noNeedParens() {\n  async satisfies mixed;\n  satisfies satisfies mixed;\n  as satisfies mixed;\n  opaque satisfies mixed;\n\n  abc satisfies mixed; // not a keyword\n}\n\nfunction satisfiesChain() {\n  satisfies satisfies satisfies satisfies satisfies;\n  (type) satisfies empty satisfies mixed;\n}");
    Ok(())
}
#[test]
fn test_ternary_js_babel_flow_format_1_d41d8cd9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("")?;
    assert_eq!(formatted, "");
    Ok(())
}
#[test]
fn test_ternary_js_format_1_f18bca57() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nfoo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) as SomeType;\n\nconst foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n  ? baaaaaaaaaaaaaaaaaaaaar\n  : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n\nfunction foo() {\n  return (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n}\n\nfunction foo() {\n  throw (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n}\n\nfunction foo() {\n  void ((coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);") ? ;
    assert_eq ! (formatted , "// @flow\n\nfoo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) as Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) as SomeType;\n\nconst foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) as Fooooooooooo;\n\nfunction foo() {\n  return (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo;\n}\n\nfunction foo() {\n  throw (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo;\n}\n\nfunction foo() {\n  void ((\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);");
    Ok(())
}
