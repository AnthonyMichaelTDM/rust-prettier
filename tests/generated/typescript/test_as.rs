#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_pattern_ts_format_1_cdfed8e7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("[x as any] = x;")?;
    assert_eq!(formatted, "[x as any] = x;");
    Ok(())
}
#[test]
fn test_as_ts_format_1_f544d24a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const name = (description as DescriptionObject).name || (description as string);\nthis.isTabActionBar((e.target || e.srcElement) as HTMLElement);\n(originalError ? wrappedError(errMsg, originalError) : Error(errMsg)) as InjectionError;\n'current' in (props.pagination as Object);\n('current' in props.pagination) as Object;\nstart + (yearSelectTotal as number);\n(start + yearSelectTotal) as number;\nscrollTop > (visibilityHeight as number);\n(scrollTop > visibilityHeight) as number;\nexport default class Column<T> extends (RcTable.Column as React.ComponentClass<ColumnProps<T>,ColumnProps<T>,ColumnProps<T>,ColumnProps<T>>) {}\nexport const MobxTypedForm = class extends (Form as { new (): any }) {}\nexport abstract class MobxTypedForm1 extends (Form as { new (): any }) {}\n({}) as {};\nfunction*g() {\n  const test = (yield 'foo') as number;\n}\nasync function g1() {\n  const test = (await 'foo') as number;\n}\n({}) as X;\n() => ({}) as X;\nconst state = JSON.stringify({\n  next: window.location.href,\n  nonce,\n} as State);\n\n(foo.bar as Baz) = [bar];\n(foo.bar as any)++;\n\n(bValue as boolean) ? 0 : -1;\n<boolean>bValue ? 0 : -1;\n\nconst value1 = thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;\nconst value2 = thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;\nconst value3 = thisIsAReallyLongIdentifier as (SomeInterface | SomeOtherInterface);\nconst value4 = thisIsAReallyLongIdentifier as { prop1: string, prop2: number, prop3: number }[];\nconst value5 = thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [string, number];\n\nconst iter1 = createIterator(this.controller, child, this.tag as SyncFunctionComponent);\nconst iter2 = createIterator(self.controller, child, self.tag as SyncFunctionComponent);") ? ;
    assert_eq ! (formatted , "const name = (description as DescriptionObject).name || (description as string);\nthis.isTabActionBar((e.target || e.srcElement) as HTMLElement);\n(originalError\n  ? wrappedError(errMsg, originalError)\n  : Error(errMsg)) as InjectionError;\n\"current\" in (props.pagination as Object);\n(\"current\" in props.pagination) as Object;\nstart + (yearSelectTotal as number);\n(start + yearSelectTotal) as number;\nscrollTop > (visibilityHeight as number);\n(scrollTop > visibilityHeight) as number;\nexport default class Column<T> extends (RcTable.Column as React.ComponentClass<\n  ColumnProps<T>,\n  ColumnProps<T>,\n  ColumnProps<T>,\n  ColumnProps<T>\n>) {}\nexport const MobxTypedForm = class extends (Form as { new (): any }) {};\nexport abstract class MobxTypedForm1 extends (Form as { new (): any }) {}\n({}) as {};\nfunction* g() {\n  const test = (yield \"foo\") as number;\n}\nasync function g1() {\n  const test = (await \"foo\") as number;\n}\n({}) as X;\n() => ({}) as X;\nconst state = JSON.stringify({\n  next: window.location.href,\n  nonce,\n} as State);\n\n(foo.bar as Baz) = [bar];\n(foo.bar as any)++;\n\n(bValue as boolean) ? 0 : -1;\n<boolean>bValue ? 0 : -1;\n\nconst value1 =\n  thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;\nconst value2 =\n  thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;\nconst value3 = thisIsAReallyLongIdentifier as\n  | SomeInterface\n  | SomeOtherInterface;\nconst value4 = thisIsAReallyLongIdentifier as {\n  prop1: string;\n  prop2: number;\n  prop3: number;\n}[];\nconst value5 =\n  thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [\n    string,\n    number,\n  ];\n\nconst iter1 = createIterator(\n  this.controller,\n  child,\n  this.tag as SyncFunctionComponent,\n);\nconst iter2 = createIterator(\n  self.controller,\n  child,\n  self.tag as SyncFunctionComponent,\n);");
    Ok(())
}
#[test]
fn test_as_const_embedded_ts_format_1_cde30fa7() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const GQL_QUERY_WITH_CONST = /* GraphQL */ `\n  query S { shop }\n` as const;\n\nconst HTML_WITH_CONST = /* HTML */ `\n<div>\n<h1>foo</h1>\n  <p>foo</p>\n</div>\n` as const;") ? ;
    assert_eq ! (formatted , "const GQL_QUERY_WITH_CONST = /* GraphQL */ `\n  query S {\n    shop\n  }\n` as const;\n\nconst HTML_WITH_CONST = /* HTML */ `\n  <div>\n    <h1>foo</h1>\n    <p>foo</p>\n  </div>\n` as const;");
    Ok(())
}
#[test]
fn test_assignment_ts_format_1_6aaa0e65() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export const LOG_LEVEL = {\n    EMERGENCY: 0,\n    ALERT: 1,\n    CRITICAL: 2,\n    ERROR: 3,\n    WARNING: 4,\n    NOTICE: 5,\n    INFO: 6,\n    DEBUG: 7,\n} as const;\n\nconst TYPE_MAP = {\n    'character device': 'special',\n    'character special file': 'special',\n    directory: 'directory',\n    'regular file': 'file',\n    socket: 'socket',\n    'symbolic link': 'link',\n} as Foo;\n\nthis.previewPlayerHandle = (setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) as unknown) as number;\n\nthis.intervalID = (setInterval(() => {\n  self.step();\n}, 30) as unknown) as number;") ? ;
    assert_eq ! (formatted , "export const LOG_LEVEL = {\n  EMERGENCY: 0,\n  ALERT: 1,\n  CRITICAL: 2,\n  ERROR: 3,\n  WARNING: 4,\n  NOTICE: 5,\n  INFO: 6,\n  DEBUG: 7,\n} as const;\n\nconst TYPE_MAP = {\n  \"character device\": \"special\",\n  \"character special file\": \"special\",\n  directory: \"directory\",\n  \"regular file\": \"file\",\n  socket: \"socket\",\n  \"symbolic link\": \"link\",\n} as Foo;\n\nthis.previewPlayerHandle = setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) as unknown as number;\n\nthis.intervalID = setInterval(() => {\n  self.step();\n}, 30) as unknown as number;");
    Ok(())
}
#[test]
fn test_assignment_2_ts_format_1_e6af6110() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const defaultMaskGetter = $parse(attrs[directiveName]) as (\n  scope: ng.IScope\n) => Mask;\n\n(this.configuration as any) = (this.editor as any) = (this\n  .editorBody as any) = undefined;\n\nangular.module(\"foo\").directive(\"formIsolator\", () => {\n  return {\n    name: \"form\",\n    controller: class FormIsolatorController {\n      $addControl = angular.noop;\n    } as ng.IControllerConstructor,\n  };\n});\n\n(this.selectorElem as any) = this.multiselectWidget = this.initialValues = undefined;\n\nconst extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) as FieldService.RendererAttributes;\n\nconst annotate = (angular.injector as any).$$annotate as (\n  fn: Function\n) => string[];\n\nconst originalPrototype = originalConstructor.prototype as TComponent & InjectionTarget,\n  propertyToServiceName = originalPrototype._inject;") ? ;
    assert_eq ! (formatted , "const defaultMaskGetter = $parse(attrs[directiveName]) as (\n  scope: ng.IScope,\n) => Mask;\n\n(this.configuration as any) =\n  (this.editor as any) =\n  (this.editorBody as any) =\n    undefined;\n\nangular.module(\"foo\").directive(\"formIsolator\", () => {\n  return {\n    name: \"form\",\n    controller: class FormIsolatorController {\n      $addControl = angular.noop;\n    } as ng.IControllerConstructor,\n  };\n});\n\n(this.selectorElem as any) =\n  this.multiselectWidget =\n  this.initialValues =\n    undefined;\n\nconst extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) as FieldService.RendererAttributes;\n\nconst annotate = (angular.injector as any).$$annotate as (\n  fn: Function,\n) => string[];\n\nconst originalPrototype = originalConstructor.prototype as TComponent &\n    InjectionTarget,\n  propertyToServiceName = originalPrototype._inject;");
    Ok(())
}
#[test]
fn test_export_default_as_ts_format_1_690ee556() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export default (function log() {} as typeof console.log)")?;
    assert_eq!(
        formatted,
        "export default (function log() {} as typeof console.log);"
    );
    Ok(())
}
#[test]
fn test_expression_statement_ts_format_1_ee28fbf9() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// expression statemnt of \"as\" expression hardly ever makes sense, but it's still valid.\nconst [type, x] = [0, 0];\n(type) as unknown;\nx as unknown;") ? ;
    assert_eq ! (formatted , "// expression statemnt of \"as\" expression hardly ever makes sense, but it's still valid.\nconst [type, x] = [0, 0];\n(type) as unknown;\nx as unknown;");
    Ok(())
}
#[test]
fn test_long_identifiers_ts_format_1_0f3beef2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;\n\naverredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =\n  annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;\n\naverredBathersBoxroomBuggyNurl = {\n  anodyneCondosMalateOverateRetinol:\n    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave\n};\n\naverredBathersBoxroomBuggyNurl(\n  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as\n    kochabCooieGameOnOboleUnweave\n);") ? ;
    assert_eq ! (formatted , "const bifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;\n\naverredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =\n  annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;\n\naverredBathersBoxroomBuggyNurl = {\n  anodyneCondosMalateOverateRetinol:\n    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,\n};\n\naverredBathersBoxroomBuggyNurl(\n  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,\n);");
    Ok(())
}
#[test]
fn test_nested_await_and_as_ts_format_1_64e5a38a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getAccountCount = async () =>\n  (await\n    ((await (\n      await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n    ).findItem(\"My bookmarks\")) as TreeItem\n  ).getChildren()\n  ).length") ? ;
    assert_eq ! (formatted , "const getAccountCount = async () =>\n  (\n    await (\n      (await (\n        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n      ).findItem(\"My bookmarks\")) as TreeItem\n    ).getChildren()\n  ).length;");
    Ok(())
}
#[test]
fn test_return_ts_format_1_b00d5bcd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("function foo() {\n  return {\n    foo: 1,\n    bar: 2,\n  } as Foo;\n}")?;
    assert_eq!(
        formatted,
        "function foo() {\n  return {\n    foo: 1,\n    bar: 2,\n  } as Foo;\n}"
    );
    Ok(())
}
#[test]
fn test_ternary_ts_format_1_ca62096a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) as SomeType;\n\nconst foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n  ? baaaaaaaaaaaaaaaaaaaaar\n  : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n\nfunction foo() {\n  return (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n}\n\nfunction foo() {\n  throw (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo;\n}\n\nfunction foo() {\n  void ((coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) as Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);") ? ;
    assert_eq ! (formatted , "foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) as Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) as SomeType;\n\nconst foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) as Fooooooooooo;\n\nfunction foo() {\n  return (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo;\n}\n\nfunction foo() {\n  throw (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo;\n}\n\nfunction foo() {\n  void ((\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) as Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);");
    Ok(())
}
