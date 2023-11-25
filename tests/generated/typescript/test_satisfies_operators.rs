#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_argument_expansion_ts_semifalse_format_1_2f3688bd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar1 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([] satisfies unknown) satisfies number[]);\n\nconst bar2 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([1, 2, 3] satisfies unknown) satisfies number[]);\n\nconst bar3 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({} satisfies unknown) satisfies {[key: number]: boolean});\n\nconst bar4 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({1: true} satisfies unknown) satisfies {[key: number]: boolean});\n\nconst bar5 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, [] satisfies foo);") ? ;
    assert_eq ! (formatted , "const bar1 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value]\n  },\n  [] satisfies unknown satisfies number[],\n)\n\nconst bar2 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value]\n  },\n  [1, 2, 3] satisfies unknown satisfies number[],\n)\n\nconst bar3 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true }\n  },\n  {} satisfies unknown satisfies { [key: number]: boolean },\n)\n\nconst bar4 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true }\n  },\n  { 1: true } satisfies unknown satisfies { [key: number]: boolean },\n)\n\nconst bar5 = [1, 2, 3].reduce((carry, value) => {\n  return [...carry, value]\n}, [] satisfies foo)");
    Ok(())
}
#[test]
fn test_argument_expansion_ts_format_1_2f3688bd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const bar1 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([] satisfies unknown) satisfies number[]);\n\nconst bar2 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, ([1, 2, 3] satisfies unknown) satisfies number[]);\n\nconst bar3 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({} satisfies unknown) satisfies {[key: number]: boolean});\n\nconst bar4 = [1,2,3].reduce((carry, value) => {\n  return {...carry, [value]: true};\n}, ({1: true} satisfies unknown) satisfies {[key: number]: boolean});\n\nconst bar5 = [1,2,3].reduce((carry, value) => {\n  return [...carry, value];\n}, [] satisfies foo);") ? ;
    assert_eq ! (formatted , "const bar1 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  [] satisfies unknown satisfies number[],\n);\n\nconst bar2 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return [...carry, value];\n  },\n  [1, 2, 3] satisfies unknown satisfies number[],\n);\n\nconst bar3 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  {} satisfies unknown satisfies { [key: number]: boolean },\n);\n\nconst bar4 = [1, 2, 3].reduce(\n  (carry, value) => {\n    return { ...carry, [value]: true };\n  },\n  { 1: true } satisfies unknown satisfies { [key: number]: boolean },\n);\n\nconst bar5 = [1, 2, 3].reduce((carry, value) => {\n  return [...carry, value];\n}, [] satisfies foo);");
    Ok(())
}
#[test]
fn test_assignment_ts_semifalse_format_1_9b2ecb7d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) satisfies FieldService.RendererAttributes;\n\nconst annotate = (angular.injector satisfies any).$$annotate satisfies (\n  fn: Function\n) => string[];\n  \nconst originalPrototype = originalConstructor.prototype satisfies TComponent & InjectionTarget,\n  propertyToServiceName = originalPrototype._inject;\n\nthis.previewPlayerHandle = (setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) satisfies unknown) satisfies number;\n\nthis.intervalID = (setInterval(() => {\n  self.step();\n}, 30) satisfies unknown) satisfies number;") ? ;
    assert_eq ! (formatted , "const extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) satisfies FieldService.RendererAttributes\n\nconst annotate = (angular.injector satisfies any).$$annotate satisfies (\n  fn: Function,\n) => string[]\n\nconst originalPrototype = originalConstructor.prototype satisfies TComponent &\n    InjectionTarget,\n  propertyToServiceName = originalPrototype._inject\n\nthis.previewPlayerHandle = setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews()\n    this.currentPreviewIndex++\n  }\n}, this.refreshDelay) satisfies unknown satisfies number\n\nthis.intervalID = setInterval(() => {\n  self.step()\n}, 30) satisfies unknown satisfies number");
    Ok(())
}
#[test]
fn test_assignment_ts_format_1_9b2ecb7d() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) satisfies FieldService.RendererAttributes;\n\nconst annotate = (angular.injector satisfies any).$$annotate satisfies (\n  fn: Function\n) => string[];\n  \nconst originalPrototype = originalConstructor.prototype satisfies TComponent & InjectionTarget,\n  propertyToServiceName = originalPrototype._inject;\n\nthis.previewPlayerHandle = (setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) satisfies unknown) satisfies number;\n\nthis.intervalID = (setInterval(() => {\n  self.step();\n}, 30) satisfies unknown) satisfies number;") ? ;
    assert_eq ! (formatted , "const extraRendererAttrs = ((attrs.rendererAttrs &&\n  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||\n  Object.create(null)) satisfies FieldService.RendererAttributes;\n\nconst annotate = (angular.injector satisfies any).$$annotate satisfies (\n  fn: Function,\n) => string[];\n\nconst originalPrototype = originalConstructor.prototype satisfies TComponent &\n    InjectionTarget,\n  propertyToServiceName = originalPrototype._inject;\n\nthis.previewPlayerHandle = setInterval(async () => {\n  if (this.previewIsPlaying) {\n    await this.fetchNextPreviews();\n    this.currentPreviewIndex++;\n  }\n}, this.refreshDelay) satisfies unknown satisfies number;\n\nthis.intervalID = setInterval(() => {\n  self.step();\n}, 30) satisfies unknown satisfies number;");
    Ok(())
}
#[test]
fn test_basic_ts_semifalse_format_1_547e0a7e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t1 = { a: 1 } satisfies I1;\nconst t2 = { a: 1, b: 1 } satisfies I1;\nconst t3 = { } satisfies I1;\nconst t4: T1 = { a: \"a\" } satisfies T1;\nconst t5 = (m => m.substring(0)) satisfies T2;\nconst t6 = [1, 2] satisfies [number, number];\nlet t7 = { a: 'test' } satisfies A;\nlet t8 = { a: 'test', b: 'test' } satisfies A;\n\nconst p = {\n  isEven: n => n % 2 === 0,\n  isOdd: n => n % 2 === 1\n} satisfies Predicates;\n\nlet obj: { f(s: string): void } & Record<string, unknown> = {\n    f(s) { },\n    g(s) { }\n} satisfies { g(s: string): void } & Record<string, unknown>;\n\n({ f(x) { } }) satisfies { f(s: string): void };\n\nconst car = {\n    start() { },\n    move(d) {\n        // d should be number\n    },\n    stop() { }\n} satisfies Movable & Record<string, unknown>;\n\nvar v = undefined satisfies 1;") ? ;
    assert_eq ! (formatted , "const t1 = { a: 1 } satisfies I1\nconst t2 = { a: 1, b: 1 } satisfies I1\nconst t3 = {} satisfies I1\nconst t4: T1 = { a: \"a\" } satisfies T1\nconst t5 = ((m) => m.substring(0)) satisfies T2\nconst t6 = [1, 2] satisfies [number, number]\nlet t7 = { a: \"test\" } satisfies A\nlet t8 = { a: \"test\", b: \"test\" } satisfies A\n\nconst p = {\n  isEven: (n) => n % 2 === 0,\n  isOdd: (n) => n % 2 === 1,\n} satisfies Predicates\n\nlet obj: { f(s: string): void } & Record<string, unknown> = {\n  f(s) {},\n  g(s) {},\n} satisfies { g(s: string): void } & Record<string, unknown>\n\n;({ f(x) {} }) satisfies { f(s: string): void }\n\nconst car = {\n  start() {},\n  move(d) {\n    // d should be number\n  },\n  stop() {},\n} satisfies Movable & Record<string, unknown>\n\nvar v = undefined satisfies 1");
    Ok(())
}
#[test]
fn test_basic_ts_format_1_547e0a7e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t1 = { a: 1 } satisfies I1;\nconst t2 = { a: 1, b: 1 } satisfies I1;\nconst t3 = { } satisfies I1;\nconst t4: T1 = { a: \"a\" } satisfies T1;\nconst t5 = (m => m.substring(0)) satisfies T2;\nconst t6 = [1, 2] satisfies [number, number];\nlet t7 = { a: 'test' } satisfies A;\nlet t8 = { a: 'test', b: 'test' } satisfies A;\n\nconst p = {\n  isEven: n => n % 2 === 0,\n  isOdd: n => n % 2 === 1\n} satisfies Predicates;\n\nlet obj: { f(s: string): void } & Record<string, unknown> = {\n    f(s) { },\n    g(s) { }\n} satisfies { g(s: string): void } & Record<string, unknown>;\n\n({ f(x) { } }) satisfies { f(s: string): void };\n\nconst car = {\n    start() { },\n    move(d) {\n        // d should be number\n    },\n    stop() { }\n} satisfies Movable & Record<string, unknown>;\n\nvar v = undefined satisfies 1;") ? ;
    assert_eq ! (formatted , "const t1 = { a: 1 } satisfies I1;\nconst t2 = { a: 1, b: 1 } satisfies I1;\nconst t3 = {} satisfies I1;\nconst t4: T1 = { a: \"a\" } satisfies T1;\nconst t5 = ((m) => m.substring(0)) satisfies T2;\nconst t6 = [1, 2] satisfies [number, number];\nlet t7 = { a: \"test\" } satisfies A;\nlet t8 = { a: \"test\", b: \"test\" } satisfies A;\n\nconst p = {\n  isEven: (n) => n % 2 === 0,\n  isOdd: (n) => n % 2 === 1,\n} satisfies Predicates;\n\nlet obj: { f(s: string): void } & Record<string, unknown> = {\n  f(s) {},\n  g(s) {},\n} satisfies { g(s: string): void } & Record<string, unknown>;\n\n({ f(x) {} }) satisfies { f(s: string): void };\n\nconst car = {\n  start() {},\n  move(d) {\n    // d should be number\n  },\n  stop() {},\n} satisfies Movable & Record<string, unknown>;\n\nvar v = undefined satisfies 1;");
    Ok(())
}
#[test]
fn test_comments_ts_semifalse_format_1_9e1111da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t2 = {} /* comment */ satisfies {};\nconst t3 = {} satisfies /* comment */ {};\nconst t4 = {} /* comment1 */ satisfies /* comment2 */ {};") ? ;
    assert_eq ! (formatted , "const t2 = {} /* comment */ satisfies {}\nconst t3 = {} satisfies /* comment */ {}\nconst t4 = {} /* comment1 */ satisfies /* comment2 */ {}");
    Ok(())
}
#[test]
fn test_comments_ts_format_1_9e1111da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t2 = {} /* comment */ satisfies {};\nconst t3 = {} satisfies /* comment */ {};\nconst t4 = {} /* comment1 */ satisfies /* comment2 */ {};") ? ;
    assert_eq ! (formatted , "const t2 = {} /* comment */ satisfies {};\nconst t3 = {} satisfies /* comment */ {};\nconst t4 = {} /* comment1 */ satisfies /* comment2 */ {};");
    Ok(())
}
#[test]
fn test_comments_unstable_ts_semifalse_format_1_639cbe79() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t1 = {\n    prop1: 1,\n    prop2: 2,\n    prop3: 3\n} satisfies\n// Comment\nRecord<string, number>;") ? ;
    assert_eq ! (formatted , "const t1 = {\n  prop1: 1,\n  prop2: 2,\n  prop3: 3,\n} satisfies // Comment\nRecord<string, number>");
    Ok(())
}
#[test]
fn test_comments_unstable_ts_format_1_639cbe79() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const t1 = {\n    prop1: 1,\n    prop2: 2,\n    prop3: 3\n} satisfies\n// Comment\nRecord<string, number>;") ? ;
    assert_eq ! (formatted , "const t1 = {\n  prop1: 1,\n  prop2: 2,\n  prop3: 3,\n} satisfies // Comment\nRecord<string, number>;");
    Ok(())
}
#[test]
fn test_export_default_as_ts_semifalse_format_1_861561b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export default (function log() {} satisfies typeof console.log)")?;
    assert_eq!(
        formatted,
        "export default (function log() {} satisfies typeof console.log)"
    );
    Ok(())
}
#[test]
fn test_export_default_as_ts_format_1_861561b5() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("export default (function log() {} satisfies typeof console.log)")?;
    assert_eq!(
        formatted,
        "export default (function log() {} satisfies typeof console.log);"
    );
    Ok(())
}
#[test]
fn test_expression_statement_ts_semifalse_format_1_931009ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nlet type: 'foo' | 'bar' = 'foo';\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = () => {\nswitch (type) {\n  case 'foo':\n    return 1;\n  case 'bar':\n    return 2;\n  default:\n    // exhaustiveness check idiom\n    (type) satisfies never;\n    throw new Error('unreachable');\n}\n}\n\nfunction needParens() {\n(let) satisfies unknown;\n(interface) satisfies unknown;\n(module) satisfies unknown;\n(using) satisfies unknown;\n(yield) satisfies unknown;\n(await) satisfies unknown;\n}\n\nfunction noNeedParens() {\nasync satisfies unknown;\nsatisfies satisfies unknown;\nas satisfies unknown;\n\nabc satisfies unknown; // not a keyword\n}\n\nfunction satisfiesChain() {\nsatisfies satisfies satisfies satisfies satisfies;\n(type) satisfies never satisfies unknown;\n}") ? ;
    assert_eq ! (formatted , "let type: \"foo\" | \"bar\" = \"foo\"\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = () => {\n  switch (type) {\n    case \"foo\":\n      return 1\n    case \"bar\":\n      return 2\n    default:\n      // exhaustiveness check idiom\n      ;(type) satisfies never\n      throw new Error(\"unreachable\")\n  }\n}\n\nfunction needParens() {\n  ;(let) satisfies unknown\n  ;(interface) satisfies unknown\n  ;(module) satisfies unknown\n  ;(using) satisfies unknown\n  ;(yield) satisfies unknown\n  ;(await) satisfies unknown\n}\n\nfunction noNeedParens() {\n  async satisfies unknown\n  satisfies satisfies unknown\n  as satisfies unknown\n\n  abc satisfies unknown // not a keyword\n}\n\nfunction satisfiesChain() {\n  satisfies satisfies satisfies satisfies satisfies\n  ;(type) satisfies never satisfies unknown\n}");
    Ok(())
}
#[test]
fn test_expression_statement_ts_format_1_931009ff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("\nlet type: 'foo' | 'bar' = 'foo';\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = () => {\nswitch (type) {\n  case 'foo':\n    return 1;\n  case 'bar':\n    return 2;\n  default:\n    // exhaustiveness check idiom\n    (type) satisfies never;\n    throw new Error('unreachable');\n}\n}\n\nfunction needParens() {\n(let) satisfies unknown;\n(interface) satisfies unknown;\n(module) satisfies unknown;\n(using) satisfies unknown;\n(yield) satisfies unknown;\n(await) satisfies unknown;\n}\n\nfunction noNeedParens() {\nasync satisfies unknown;\nsatisfies satisfies unknown;\nas satisfies unknown;\n\nabc satisfies unknown; // not a keyword\n}\n\nfunction satisfiesChain() {\nsatisfies satisfies satisfies satisfies satisfies;\n(type) satisfies never satisfies unknown;\n}") ? ;
    assert_eq ! (formatted , "let type: \"foo\" | \"bar\" = \"foo\";\n\n// demonstrating how \"satisfies\" expression can be practically used as expression statement.\nconst _ = () => {\n  switch (type) {\n    case \"foo\":\n      return 1;\n    case \"bar\":\n      return 2;\n    default:\n      // exhaustiveness check idiom\n      (type) satisfies never;\n      throw new Error(\"unreachable\");\n  }\n};\n\nfunction needParens() {\n  (let) satisfies unknown;\n  (interface) satisfies unknown;\n  (module) satisfies unknown;\n  (using) satisfies unknown;\n  (yield) satisfies unknown;\n  (await) satisfies unknown;\n}\n\nfunction noNeedParens() {\n  async satisfies unknown;\n  satisfies satisfies unknown;\n  as satisfies unknown;\n\n  abc satisfies unknown; // not a keyword\n}\n\nfunction satisfiesChain() {\n  satisfies satisfies satisfies satisfies satisfies;\n  (type) satisfies never satisfies unknown;\n}");
    Ok(())
}
#[test]
fn test_gt_lt_ts_semifalse_format_1_deed2e47() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("x satisfies boolean <= y; // (x satisfies boolean) <= y;\nx satisfies boolean ?? y; // (x satisfies boolean) ?? y;") ? ;
    assert_eq ! (formatted , ";(x satisfies boolean) <= y // (x satisfies boolean) <= y;\n;(x satisfies boolean) ?? y // (x satisfies boolean) ?? y;");
    Ok(())
}
#[test]
fn test_gt_lt_ts_format_1_deed2e47() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("x satisfies boolean <= y; // (x satisfies boolean) <= y;\nx satisfies boolean ?? y; // (x satisfies boolean) ?? y;") ? ;
    assert_eq ! (formatted , "(x satisfies boolean) <= y; // (x satisfies boolean) <= y;\n(x satisfies boolean) ?? y; // (x satisfies boolean) ?? y;");
    Ok(())
}
#[test]
fn test_hug_args_ts_semifalse_format_1_8a346cff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("window.postMessage(\n    {\n      context: item.context,\n      topic: item.topic\n    } satisfies IActionMessage\n  );") ? ;
    assert_eq ! (formatted , "window.postMessage({\n  context: item.context,\n  topic: item.topic,\n} satisfies IActionMessage)");
    Ok(())
}
#[test]
fn test_hug_args_ts_format_1_8a346cff() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("window.postMessage(\n    {\n      context: item.context,\n      topic: item.topic\n    } satisfies IActionMessage\n  );") ? ;
    assert_eq ! (formatted , "window.postMessage({\n  context: item.context,\n  topic: item.topic,\n} satisfies IActionMessage);");
    Ok(())
}
#[test]
fn test_lhs_ts_semifalse_format_1_1f6f3438() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a satisfies number) = 42;\n({ a: (b satisfies any) = 2000 } = x);\n(this.selectorElem satisfies any) = this.multiselectWidget = this.initialValues = undefined;\n(this.configuration satisfies any) = (this.editor satisfies any) = (this\n  .editorBody satisfies any) = undefined;") ? ;
    assert_eq ! (formatted , ";(a satisfies number) = 42\n;({ a: (b satisfies any) = 2000 } = x)\n;(this.selectorElem satisfies any) =\n  this.multiselectWidget =\n  this.initialValues =\n    undefined\n;(this.configuration satisfies any) =\n  (this.editor satisfies any) =\n  (this.editorBody satisfies any) =\n    undefined");
    Ok(())
}
#[test]
fn test_lhs_ts_format_1_1f6f3438() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a satisfies number) = 42;\n({ a: (b satisfies any) = 2000 } = x);\n(this.selectorElem satisfies any) = this.multiselectWidget = this.initialValues = undefined;\n(this.configuration satisfies any) = (this.editor satisfies any) = (this\n  .editorBody satisfies any) = undefined;") ? ;
    assert_eq ! (formatted , "(a satisfies number) = 42;\n({ a: (b satisfies any) = 2000 } = x);\n(this.selectorElem satisfies any) =\n  this.multiselectWidget =\n  this.initialValues =\n    undefined;\n(this.configuration satisfies any) =\n  (this.editor satisfies any) =\n  (this.editorBody satisfies any) =\n    undefined;");
    Ok(())
}
#[test]
fn test_nested_await_and_satisfies_ts_semifalse_format_1_27d1ca63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getAccountCount = async () =>\n  (await\n    ((await (\n      await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n    ).findItem(\"My bookmarks\")) satisfies TreeItem\n  ).getChildren()\n  ).length") ? ;
    assert_eq ! (formatted , "const getAccountCount = async () =>\n  (\n    await (\n      (await (\n        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n      ).findItem(\"My bookmarks\")) satisfies TreeItem\n    ).getChildren()\n  ).length");
    Ok(())
}
#[test]
fn test_nested_await_and_satisfies_ts_format_1_27d1ca63() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const getAccountCount = async () =>\n  (await\n    ((await (\n      await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n    ).findItem(\"My bookmarks\")) satisfies TreeItem\n  ).getChildren()\n  ).length") ? ;
    assert_eq ! (formatted , "const getAccountCount = async () =>\n  (\n    await (\n      (await (\n        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)\n      ).findItem(\"My bookmarks\")) satisfies TreeItem\n    ).getChildren()\n  ).length;");
    Ok(())
}
#[test]
fn test_non_null_ts_semifalse_format_1_ebf40bd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el satisfies HTMLElement)!.style.cursor = 'pointer'") ? ;
    assert_eq ! (formatted , "// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el satisfies HTMLElement)!.style.cursor = \"pointer\"");
    Ok(())
}
#[test]
fn test_non_null_ts_format_1_ebf40bd0() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref)\n;(el satisfies HTMLElement)!.style.cursor = 'pointer'") ? ;
    assert_eq ! (formatted , "// the 2nd line needs ASI protection\nconst el = ReactDOM.findDOMNode(ref);\n(el satisfies HTMLElement)!.style.cursor = \"pointer\";");
    Ok(())
}
#[test]
fn test_satisfies_ts_semifalse_format_1_c27a32f6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("({}) satisfies {};\n({}) satisfies X;\n() => ({}) satisfies X;\nthis.isTabActionBar((e.target || e.srcElement) satisfies HTMLElement);\n\n'current' in (props.pagination satisfies Object);\n('current' in props.pagination) satisfies Object;\nstart + (yearSelectTotal satisfies number);\n(start + yearSelectTotal) satisfies number;\nscrollTop > (visibilityHeight satisfies number);\n(scrollTop > visibilityHeight) satisfies number;\n(bValue satisfies boolean) ? 0 : -1;\n\nasync function g1() {\n  const test = (await 'foo') satisfies number;\n}\n\nvar x = (v => v) satisfies (x: number) => string;\n\nfoo satisfies unknown satisfies Bar;\nfoo satisfies unknown as Bar;\nfoo as unknown satisfies Bar;") ? ;
    assert_eq ! (formatted , ";({}) satisfies {}\n;({}) satisfies X\n;() => ({}) satisfies X\nthis.isTabActionBar((e.target || e.srcElement) satisfies HTMLElement)\n\n\"current\" in (props.pagination satisfies Object)\n;(\"current\" in props.pagination) satisfies Object\nstart + (yearSelectTotal satisfies number)\n;(start + yearSelectTotal) satisfies number\nscrollTop > (visibilityHeight satisfies number)\n;(scrollTop > visibilityHeight) satisfies number\n;(bValue satisfies boolean) ? 0 : -1\n\nasync function g1() {\n  const test = (await \"foo\") satisfies number\n}\n\nvar x = ((v) => v) satisfies (x: number) => string\n\nfoo satisfies unknown satisfies Bar\nfoo satisfies unknown as Bar\nfoo as unknown satisfies Bar");
    Ok(())
}
#[test]
fn test_satisfies_ts_format_1_c27a32f6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("({}) satisfies {};\n({}) satisfies X;\n() => ({}) satisfies X;\nthis.isTabActionBar((e.target || e.srcElement) satisfies HTMLElement);\n\n'current' in (props.pagination satisfies Object);\n('current' in props.pagination) satisfies Object;\nstart + (yearSelectTotal satisfies number);\n(start + yearSelectTotal) satisfies number;\nscrollTop > (visibilityHeight satisfies number);\n(scrollTop > visibilityHeight) satisfies number;\n(bValue satisfies boolean) ? 0 : -1;\n\nasync function g1() {\n  const test = (await 'foo') satisfies number;\n}\n\nvar x = (v => v) satisfies (x: number) => string;\n\nfoo satisfies unknown satisfies Bar;\nfoo satisfies unknown as Bar;\nfoo as unknown satisfies Bar;") ? ;
    assert_eq ! (formatted , "({}) satisfies {};\n({}) satisfies X;\n() => ({}) satisfies X;\nthis.isTabActionBar((e.target || e.srcElement) satisfies HTMLElement);\n\n\"current\" in (props.pagination satisfies Object);\n(\"current\" in props.pagination) satisfies Object;\nstart + (yearSelectTotal satisfies number);\n(start + yearSelectTotal) satisfies number;\nscrollTop > (visibilityHeight satisfies number);\n(scrollTop > visibilityHeight) satisfies number;\n(bValue satisfies boolean) ? 0 : -1;\n\nasync function g1() {\n  const test = (await \"foo\") satisfies number;\n}\n\nvar x = ((v) => v) satisfies (x: number) => string;\n\nfoo satisfies unknown satisfies Bar;\nfoo satisfies unknown as Bar;\nfoo as unknown satisfies Bar;");
    Ok(())
}
#[test]
fn test_template_literal_ts_semifalse_format_1_88238fe2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = `${(foo + bar) satisfies baz}`;\nconst b = `${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) satisfies baz}`;\nconst b = `${(foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies baz}`;\nconst b = `${(foo + bar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}`;\nconst b = `${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}`;") ? ;
    assert_eq ! (formatted , "const a = `${(foo + bar) satisfies baz}`\nconst b = `${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) satisfies baz\n}`\nconst b = `${\n  (foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies baz\n}`\nconst b = `${\n  (foo + bar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}`\nconst b = `${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo +\n    veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}`");
    Ok(())
}
#[test]
fn test_template_literal_ts_format_1_88238fe2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const a = `${(foo + bar) satisfies baz}`;\nconst b = `${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) satisfies baz}`;\nconst b = `${(foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies baz}`;\nconst b = `${(foo + bar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}`;\nconst b = `${(veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz}`;") ? ;
    assert_eq ! (formatted , "const a = `${(foo + bar) satisfies baz}`;\nconst b = `${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) satisfies baz\n}`;\nconst b = `${\n  (foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies baz\n}`;\nconst b = `${\n  (foo + bar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}`;\nconst b = `${\n  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo +\n    veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) satisfies veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz\n}`;");
    Ok(())
}
#[test]
fn test_ternary_ts_semifalse_format_1_e9ecf507() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) satisfies SomeType;\n\nconst foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n  ? baaaaaaaaaaaaaaaaaaaaar\n  : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n\nfunction foo() {\n  return (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  throw (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  void ((coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);") ? ;
    assert_eq ! (formatted , "foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) satisfies Fooooooooooo\n\nfoo = (condition ? firstValue : secondValue) satisfies SomeType\n\nconst foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) satisfies Fooooooooooo\n\nfunction foo() {\n  return (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo\n}\n\nfunction foo() {\n  throw (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo\n}\n\nfunction foo() {\n  void ((\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo)\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay)\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay)");
    Ok(())
}
#[test]
fn test_ternary_ts_format_1_e9ecf507() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) satisfies SomeType;\n\nconst foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n  ? baaaaaaaaaaaaaaaaaaaaar\n  : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n\nfunction foo() {\n  return (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  throw (coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  void ((coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz) satisfies Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);") ? ;
    assert_eq ! (formatted , "foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) satisfies Fooooooooooo;\n\nfoo = (condition ? firstValue : secondValue) satisfies SomeType;\n\nconst foo = (\n  coooooooooooooooooooooooooooooooooooooooooooooooooooond\n    ? baaaaaaaaaaaaaaaaaaaaar\n    : baaaaaaaaaaaaaaaaaaaaaz\n) satisfies Fooooooooooo;\n\nfunction foo() {\n  return (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  throw (\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo;\n}\n\nfunction foo() {\n  void ((\n    coooooooooooooooooooooooooooooooooooooooooooooooooooond\n      ? baaaaaaaaaaaaaaaaaaaaar\n      : baaaaaaaaaaaaaaaaaaaaaz\n  ) satisfies Fooooooooooo);\n}\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);\n\nbifornCringerMoshedPerplexSawder =\n  askTrovenaBeenaDependsRowans +\n  ((glimseGlyphsHazardNoopsTieTie === 0 &&\n  kochabCooieGameOnOboleUnweave === Math.PI\n    ? averredBathersBoxroomBuggyNurl\n    : anodyneCondosMalateOverateRetinol) satisfies AnnularCooeedSplicesWalksWayWay);");
    Ok(())
}
#[test]
fn test_types_comments_ts_semifalse_format_1_055d9273() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .semi(false)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(() => {\n  // swallow error and fallback to using directory as path\n}) satisfies string[];") ? ;
    assert_eq ! (formatted , ";(() => {\n  // swallow error and fallback to using directory as path\n}) satisfies string[]");
    Ok(())
}
#[test]
fn test_types_comments_ts_format_1_055d9273() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(() => {\n  // swallow error and fallback to using directory as path\n}) satisfies string[];") ? ;
    assert_eq ! (formatted , "(() => {\n  // swallow error and fallback to using directory as path\n}) satisfies string[];");
    Ok(())
}
