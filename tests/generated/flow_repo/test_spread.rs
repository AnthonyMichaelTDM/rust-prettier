#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_0a8ad5f1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function parseTimestamp(timestamp: string): number {\n    return 0;\n}\n\nfunction parseCounter(line: string): number {\n    return 0;\n}\n\nfunction parseGroup(lines: Array<string>): {\n    counter: number;\n    begin: number;\n    end: number;\n    text: string;\n} {\n    var counter = parseCounter(lines[0]);\n    var timeframe = parseTimeframe(lines[1]);\n    return {\n        counter,\n        ...timeframe,\n        text: lines[2]\n    };\n}\n\nfunction parseTimeframe(line: string): { begin: number; end: number } {\n    var timestamps = line.split('-->');\n    return {\n        begin: parseTimestamp(timestamps[0].trim()),\n        end: parseTimestamp(timestamps[1].trim())\n    };\n}") ? ;
    assert_eq ! (formatted , "function parseTimestamp(timestamp: string): number {\n  return 0;\n}\n\nfunction parseCounter(line: string): number {\n  return 0;\n}\n\nfunction parseGroup(lines: Array<string>): {\n  counter: number,\n  begin: number,\n  end: number,\n  text: string,\n} {\n  var counter = parseCounter(lines[0]);\n  var timeframe = parseTimeframe(lines[1]);\n  return {\n    counter,\n    ...timeframe,\n    text: lines[2],\n  };\n}\n\nfunction parseTimeframe(line: string): { begin: number, end: number } {\n  var timestamps = line.split(\"-->\");\n  return {\n    begin: parseTimestamp(timestamps[0].trim()),\n    end: parseTimestamp(timestamps[1].trim()),\n  };\n}");
    Ok(())
}
#[test]
fn test_test_2_js_format_1_363e8b30() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @flow\n */\n\nfunction foo(o) {\n    bar({...o});\n}\nfunction bar(_: {foo:number}) { }\nfoo({foo: 42});") ? ;
    assert_eq ! (formatted , "/**\n * @flow\n */\n\nfunction foo(o) {\n  bar({ ...o });\n}\nfunction bar(_: { foo: number }) {}\nfoo({ foo: 42 });");
    Ok(())
}
#[test]
fn test_test_3_js_format_1_08635423() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var p = { y: \"\" };\nvar q = { z: \"\" };\nvar o = {\n  x: 5,\n  ...p,\n  ...q,\n};\nvar y: number = o.y;\nvar z: number = o.z;\n\n// test conflicting keys (they get unioned)\nvar r = { y: 123 };\nvar s = {\n  ...p,\n  ...r,\n};\nvar t: boolean = s.y; // error, string or number") ? ;
    assert_eq ! (formatted , "var p = { y: \"\" };\nvar q = { z: \"\" };\nvar o = {\n  x: 5,\n  ...p,\n  ...q,\n};\nvar y: number = o.y;\nvar z: number = o.z;\n\n// test conflicting keys (they get unioned)\nvar r = { y: 123 };\nvar s = {\n  ...p,\n  ...r,\n};\nvar t: boolean = s.y; // error, string or number");
    Ok(())
}
#[test]
fn test_test_4_js_format_1_33adaeba() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "/* @flow */\nfunction test(...nums: Array<number>) {}\n\ntest(0, ...[1, 2, 3]);",
    )?;
    assert_eq!(
        formatted,
        "/* @flow */\nfunction test(...nums: Array<number>) {}\n\ntest(0, ...[1, 2, 3]);"
    );
    Ok(())
}
#[test]
fn test_test_5_js_format_1_6196b2da() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\ndeclare function map<Tv, TNext>(\n  obj: {[key: string]: Tv},\n  iterator:((obj: Tv) => TNext),\n): Array<TNext>;\n\n/**\n * Tests overriding a property via a spread, where the value is a tvar. the\n * type of the prop from the object that is being overridden (`x.kind` in the\n * case below) should //not// feed back into the tvar (`value`), since the\n * result is a new object.\n */\nfunction test(\n  x: {kind: ?string},\n  kinds: {[key: string]: string}\n): Array<{kind: ?string}> {\n  return map(kinds, (value) => {\n    (value: string); // OK\n    return {\n      ...x,\n      kind: value,\n    };\n  });\n}") ? ;
    assert_eq ! (formatted , "/* @flow */\n\ndeclare function map<Tv, TNext>(\n  obj: { [key: string]: Tv },\n  iterator: (obj: Tv) => TNext,\n): Array<TNext>;\n\n/**\n * Tests overriding a property via a spread, where the value is a tvar. the\n * type of the prop from the object that is being overridden (`x.kind` in the\n * case below) should //not// feed back into the tvar (`value`), since the\n * result is a new object.\n */\nfunction test(\n  x: { kind: ?string },\n  kinds: { [key: string]: string },\n): Array<{ kind: ?string }> {\n  return map(kinds, (value) => {\n    (value: string); // OK\n    return {\n      ...x,\n      kind: value,\n    };\n  });\n}");
    Ok(())
}
#[test]
fn test_test_6_js_format_1_d2d008c4() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var o = {\n  foo: 'bar'\n};\no = {...o};\n(o: {foo: string});\n\nvar p = {\n  foo: 'bar'\n};\n(p: {foo: string; abc: string}); // error, p doesn't have `abc` yet\np = {...p, abc: 'def'};\n(p: {foo: string; abc: string});\n\nvar q = {\n  foo: 'bar'\n};\nfor (var i = 0; i < 10; i++) {\n  q = {...q};\n}\n(q: {foo: string});") ? ;
    assert_eq ! (formatted , "var o = {\n  foo: \"bar\",\n};\no = { ...o };\n(o: { foo: string });\n\nvar p = {\n  foo: \"bar\",\n};\n(p: { foo: string, abc: string }); // error, p doesn't have `abc` yet\np = { ...p, abc: \"def\" };\n(p: { foo: string, abc: string });\n\nvar q = {\n  foo: \"bar\",\n};\nfor (var i = 0; i < 10; i++) {\n  q = { ...q };\n}\n(q: { foo: string });");
    Ok(())
}
#[test]
fn test_test_7_js_format_1_c5f0e188() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nlet tests = [\n  function(x: Object) {\n    ({...x}: Object);\n    ({...x}: void); // error, Object\n  },\n];") ? ;
    assert_eq ! (formatted , "// @flow\n\nlet tests = [\n  function (x: Object) {\n    ({ ...x }: Object);\n    ({ ...x }: void); // error, Object\n  },\n];");
    Ok(())
}
