#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_delegate_yield_js_format_1_c2e08408() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function *delegate_next() {\n  async function *inner() {\n    var x: void = yield; // error: number ~> void\n  }\n  yield *inner();\n}\ndelegate_next().next(0);\n\nasync function *delegate_yield() {\n  async function *inner() {\n    yield 0;\n  }\n  yield *inner();\n}\n(async () => {\n  for await (const x of delegate_yield()) {\n    (x: void); // error: number ~> void\n  }\n});\n\nasync function *delegate_return() {\n  async function *inner() {\n    return 0;\n  }\n  var x: void = yield *inner(); // error: number ~> void\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function* delegate_next() {\n  async function* inner() {\n    var x: void = yield; // error: number ~> void\n  }\n  yield* inner();\n}\ndelegate_next().next(0);\n\nasync function* delegate_yield() {\n  async function* inner() {\n    yield 0;\n  }\n  yield* inner();\n}\nasync () => {\n  for await (const x of delegate_yield()) {\n    (x: void); // error: number ~> void\n  }\n};\n\nasync function* delegate_return() {\n  async function* inner() {\n    return 0;\n  }\n  var x: void = yield* inner(); // error: number ~> void\n}");
}
#[test]
fn test_generator_js_format_1_32500f2a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare interface File {\n  readLine(): Promise<string>;\n  close(): void;\n  EOF: boolean;\n}\n\ndeclare function fileOpen(path: string): Promise<File>;\n\nasync function* readLines(path) {\n  let file: File = await fileOpen(path);\n\n  try {\n    while (!file.EOF) {\n      yield await file.readLine();\n    }\n  } finally {\n    file.close();\n  }\n}\n\nasync function f() {\n  for await (const line of readLines(\"/path/to/file\")) {\n    (line: void); // error: string ~> void\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare interface File {\n  readLine(): Promise<string>;\n  close(): void;\n  EOF: boolean;\n}\n\ndeclare function fileOpen(path: string): Promise<File>;\n\nasync function* readLines(path) {\n  let file: File = await fileOpen(path);\n\n  try {\n    while (!file.EOF) {\n      yield await file.readLine();\n    }\n  } finally {\n    file.close();\n  }\n}\n\nasync function f() {\n  for await (const line of readLines(\"/path/to/file\")) {\n    (line: void); // error: string ~> void\n  }\n}");
}
#[test]
fn test_return_js_format_1_c3e908e7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("declare var gen: AsyncGenerator<void,string,void>;\n\n// You can pass whatever you like to return, it doesn't need to be related to\n// the AsyncGenerator's return type\ngen.return(0).then(result => {\n  (result.value: void); // error: string | number ~> void\n});\n\n// However, a generator can \"refuse\" the return by catching an exception and\n// yielding or returning internally.\nasync function *refuse_return() {\n  try {\n    yield 1;\n  } finally {\n    return 0;\n  }\n}\nrefuse_return().return(\"string\").then(result => {\n  if (result.done) {\n    (result.value: string); // error: number | void ~> string\n  }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "declare var gen: AsyncGenerator<void, string, void>;\n\n// You can pass whatever you like to return, it doesn't need to be related to\n// the AsyncGenerator's return type\ngen.return(0).then((result) => {\n  (result.value: void); // error: string | number ~> void\n});\n\n// However, a generator can \"refuse\" the return by catching an exception and\n// yielding or returning internally.\nasync function* refuse_return() {\n  try {\n    yield 1;\n  } finally {\n    return 0;\n  }\n}\nrefuse_return()\n  .return(\"string\")\n  .then((result) => {\n    if (result.done) {\n      (result.value: string); // error: number | void ~> string\n    }\n  });");
}
#[test]
fn test_throw_js_format_1_05d81d64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("async function *catch_return() {\n  try {\n    yield 0;\n  } catch (e) {\n    return e;\n  }\n}\n\n(async () => {\n  catch_return().throw(\"\").then(({value}) => {\n    if (value !== undefined) {\n      (value: void); // error: number ~> void\n    }\n  });\n});\n\nasync function *yield_return() {\n  try {\n    yield 0;\n    return;\n  } catch (e) {\n    yield e;\n  }\n}\n\n(async () => {\n  yield_return().throw(\"\").then(({value}) => {\n    if (value !== undefined) {\n      (value: void); // error: number ~> void\n    }\n  });\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "async function* catch_return() {\n  try {\n    yield 0;\n  } catch (e) {\n    return e;\n  }\n}\n\nasync () => {\n  catch_return()\n    .throw(\"\")\n    .then(({ value }) => {\n      if (value !== undefined) {\n        (value: void); // error: number ~> void\n      }\n    });\n};\n\nasync function* yield_return() {\n  try {\n    yield 0;\n    return;\n  } catch (e) {\n    yield e;\n  }\n}\n\nasync () => {\n  yield_return()\n    .throw(\"\")\n    .then(({ value }) => {\n      if (value !== undefined) {\n        (value: void); // error: number ~> void\n      }\n    });\n};");
}
