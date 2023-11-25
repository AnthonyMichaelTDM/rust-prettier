use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_test_js_format_1_c571753e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type T5 = \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\" | \"10\" | \"11\" | \"12\" | \"13\";\n\ntype T6 = \"a-long-string\" | \"another-long-string\" | \"yet-another-long-string\" | \"one-more-for-good-measure\";\n\ntype T7 =\n  { eventName: \"these\", a: number } |\n  { eventName: \"will\", b: number } |\n  { eventName: \"not\", c: number } |\n  { eventName: \"fit\", d: number } |\n  { eventName: \"on\", e: number } |\n  { eventName: \"one\", f: number } |\n  { eventName: \"line\", g: number };\n\ntype Comment = {\n  type: 'CommentLine';\n  _CommentLine: void;\n  value: string;\n  end: number;\n  loc: {\n    end: {column: number, line: number},\n    start: {column: number, line: number},\n  };\n  start: number;\n} | {\n  type: 'CommentBlock';\n  _CommentBlock: void;\n  value: string;\n  end: number;\n  loc: {\n    end: {column: number, line: number},\n    start: {column: number, line: number},\n  };\n  start: number;\n};\n\ntype Props = {\n  onChange: (\n    | {\n        name: string\n      }\n    | {\n        title: string\n      }\n    | {\n        year: year\n      }\n  ) => void\n};\n\ndeclare class FormData {\n  append(\n    options?:\n      | string\n      | {\n          filepath?: string,\n          filename?: string\n        }\n  ): void;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type T5 =\n  | \"1\"\n  | \"2\"\n  | \"3\"\n  | \"4\"\n  | \"5\"\n  | \"6\"\n  | \"7\"\n  | \"8\"\n  | \"9\"\n  | \"10\"\n  | \"11\"\n  | \"12\"\n  | \"13\";\n\ntype T6 =\n  | \"a-long-string\"\n  | \"another-long-string\"\n  | \"yet-another-long-string\"\n  | \"one-more-for-good-measure\";\n\ntype T7 =\n  | { eventName: \"these\", a: number }\n  | { eventName: \"will\", b: number }\n  | { eventName: \"not\", c: number }\n  | { eventName: \"fit\", d: number }\n  | { eventName: \"on\", e: number }\n  | { eventName: \"one\", f: number }\n  | { eventName: \"line\", g: number };\n\ntype Comment =\n  | {\n      type: \"CommentLine\",\n      _CommentLine: void,\n      value: string,\n      end: number,\n      loc: {\n        end: { column: number, line: number },\n        start: { column: number, line: number },\n      },\n      start: number,\n    }\n  | {\n      type: \"CommentBlock\",\n      _CommentBlock: void,\n      value: string,\n      end: number,\n      loc: {\n        end: { column: number, line: number },\n        start: { column: number, line: number },\n      },\n      start: number,\n    };\n\ntype Props = {\n  onChange: (\n    | {\n        name: string,\n      }\n    | {\n        title: string,\n      }\n    | {\n        year: year,\n      },\n  ) => void,\n};\n\ndeclare class FormData {\n  append(\n    options?:\n      | string\n      | {\n          filepath?: string,\n          filename?: string,\n        },\n  ): void;\n}");
}
