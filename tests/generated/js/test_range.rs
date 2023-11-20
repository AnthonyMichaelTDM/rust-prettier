#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_array_js_format_1_2fed75dd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(16)
        .range_start(8)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | a = [\n  2 | ,\n> 3 | ,,,,\n    | ^^^^\n> 4 |\n    | ^\n> 5 | ,\n    | ^\n> 6 |\n    | ^\n  7 | ,\n  8 | a,\n  9 | ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a = [, , , , , , , a]");
}
#[test]
fn test_boundary_js_format_1_6cede986() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(26)
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(13)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | foo = 1.0000;bar = 1.0000;baz=1.0000;\n    |              ^^^^^^^^^^^^^\n  2 | // The range will be 13~26\n  3 | // \\`foo\\` ends at 13, should not format\n  4 | // \\`bar\\` ends at 26, should format\n  5 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = 1.0000;bar = 1.0;baz=1.0000;\n// The range will be 13~26\n// \\`foo\\` ends at 13, should not format\n// \\`bar\\` ends at 26, should format");
}
#[test]
fn test_boundary_2_js_format_1_dd21cfe9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(45)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(28)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | function a(\n   2 | ){\n   3 | a (\n   4 | );\n   5 | b (\n>  6 | );                 c (\n     |   ^^^^^^^^^^^^^^^^^\n   7 | ); d(\n   8 | );\n   9 |\n  10 | }\n  11 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "function a(\n){\n  a();\n  b();\n  c();\n  d();\n}"
    );
}
#[test]
fn test_boundary_3_js_format_1_3c250cd8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(30)
        .range_start(13)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | a (\n  2 | );\n  3 | b (\n> 4 | );                 c (\n    |   ^^^^^^^^^^^^^^^^^\n  5 | ); d(\n  6 | );\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "a (\n);\nb (\n);                 c (\n); d(\n);");
}
#[test]
fn test_class_declaration_js_format_1_c513cf3c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(26)
        .range_start(10)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 |\n  2 |\n> 3 | class    a {\n    |         ^^^^\n> 4 |   b(   ) {}\n    | ^^^^^^^^^^^\n  5 | }\n  6 |\n  7 | let    ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\n\nclass a {\n  b() {}\n}\n\nlet    ");
}
#[test]
fn test_different_levels_js_format_1_e0ec2874() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(48)
        .range_start(19)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | call(1,2,3)\n> 2 | call(1,2,3)\n    |        ^^^^\n> 3 | function f() {\n    | ^^^^^^^^^^^^^^\n> 4 |   call(1,2,3)\n    | ^^^^^^^^^\n  5 | }\n  6 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "call(1,2,3)\ncall(1, 2, 3);\nfunction f() {\n  call(1, 2, 3);\n}"
    );
}
#[test]
fn test_directive_js_format_1_fe3c76fc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(1)
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(0)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | 'aaa';\n    | ^\n  2 | 'bbb';\n  3 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "\"aaa\";\n'bbb';");
}
#[test]
fn test_function_body_js_format_1_ce0ffe04() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(20)
        .range_end(33)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "  1 | let fn =a((x ) => {\n> 2 |   quux (); //\n    | ^^^^^^^^^^^^^\n  3 | });\n  4 ",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "let fn =a((x ) => {\n  quux(); //\n});");
}
#[test]
fn test_function_declaration_js_format_1_968d9ef4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_start(16)
        .range_end(43)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | function ugly ( {a=1,     b     =   2     }      ) {}\n    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^\n  2 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "function ugly({ a = 1, b = 2 }) {}");
}
#[test]
fn test_ignore_indentation_js_format_1_47e23903() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(225)
        .range_start(222)
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | function ugly ( {a=1,     b     =   2     }      ) {\n  2 |   function ugly ( {a=1,     b     =   2     }      ) {\n  3 |     function ugly ( {a=1,     b     =   2     }      ) {\n  4 |   \t  \t     \\`multiline template string\n> 5 |               with too much indentation\\`\n    |                    ^^^\n  6 |     }\n  7 |   }\n  8 | }\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function ugly ( {a=1,     b     =   2     }      ) {\n  function ugly ( {a=1,     b     =   2     }      ) {\n    function ugly ( {a=1,     b     =   2     }      ) {\n  \t  \t     \\`multiline template string\n              with too much indentation\\`;\n    }\n  }\n}");
}
#[test]
fn test_issue_3789_1_js_format_1_6399e9d1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(17)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(31)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | export class F {\n>  2 |   reformatThis() {\n     | ^^^^^^^^^^^^^^\n   3 |     return 1;\n   4 |   }\n   5 |\n   6 |   dontTouchThis(){\n   7 |     return 2    ;\n   8 |   }\n   9 | }\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class F {\n  reformatThis() {\n    return 1;\n  }\n\n  dontTouchThis() {\n    return 2;\n  }\n}");
}
#[test]
fn test_issue_3789_2_js_format_1_e7b010fb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(18)
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(17)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | export class F {\n>  2 |   reformatThis() {\n     | ^\n   3 |     return 1;\n   4 |   }\n   5 |\n   6 |   dontTouchThis(){\n   7 |     return 2    ;\n   8 |   }\n   9 | }\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export class F {\n  reformatThis() {\n    return 1;\n  }\n\n  dontTouchThis() {\n    return 2;\n  }\n}");
}
#[test]
fn test_issue_4206_1_js_format_1_e3fc0a49() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(16)
        .range_end(30)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | export default class Foo{\n    |                 ^^^^^^^^^\n> 2 | /**/\n    | ^^^^\n  3 | }\n  4 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export default class Foo {\n  /**/\n}");
}
#[test]
fn test_issue_4206_2_js_format_1_1aa14a8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(16)
        .print_width(80)
        .range_end(30)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | export default function Foo(){\n    |                 ^^^^^^^^^^^^^^\n  2 | /**/\n  3 | }\n  4 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "export default function Foo() {\n  /**/\n}");
}
#[test]
fn test_issue_4206_3_js_format_1_f298a68d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .range_start(16)
        .range_end(28)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | /* */ function Foo(){\n    |                 ^^^^^\n> 2 | /**/\n    | ^^^^\n> 3 | }\n    | ^\n  4 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* */ function Foo() {\n  /**/\n}");
}
#[test]
fn test_issue_4206_4_js_format_1_17ed444b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(23)
        .range_start(17)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("  1 | /* */ class Foo{\n> 2 | /**/\n    | ^^^^\n> 3 | }\n    | ^\n  4 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "/* */ class Foo {\n  /**/\n}");
}
#[test]
fn test_issue_7082_js_format_1_f9250f51() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .range_start(29)
        .range_end(42)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | export const Button = styled.button\\`\n    |                              ^^^^^^^\n> 2 | color: blue;\n    | ^^^^^\n  3 | \\`;\n  4 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "export const Button = styled.button\\`\n  color: blue;\n\\`;"
    );
}
#[test]
fn test_large_dict_js_format_1_4eb247bb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_start(80)
        .range_end(155)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | function ugly() {\n  2 |   const dictWithSeveralEntries = {\n  3 |     key:          \"value\",\n> 4 |     anotherKey:   \"another value\",\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 5 |     firstNumber:  1,\n    | ^^^^^^^^^^^^^^^^^^^^\n> 6 |     secondNumber: 2\n    | ^^^^^^^^^^^^^^^^^^^\n  7 |   };\n  8 | }\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function ugly() {\n  const dictWithSeveralEntries = {\n    key: \"value\",\n    anotherKey: \"another value\",\n    firstNumber: 1,\n    secondNumber: 2,\n  };\n}");
}
#[test]
fn test_module_export_1_js_format_1_99845667() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(45)
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(56)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | import  def , {named}  from    'x'\n   2 |\n>  3 | export *  from   'd'\n     |          ^^^^^^^^^^^\n   4 |\n   5 | export    const  x\n   6 |   =  42\n   7 |\n   8 | export   default    42\n   9 |\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import  def , {named}  from    'x'\n\nexport * from \"d\";\n\nexport    const  x\n  =  42\n\nexport   default    42\n");
}
#[test]
fn test_module_export_2_js_format_1_9656c1f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(79)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(57)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | import  def , {named}  from    'x'\n   2 |\n   3 | export *  from   'd'\n>  4 |\n     | ^\n>  5 | export    const  x\n     | ^^^^^^^^^^^^^^^^^^\n>  6 |   =  42\n     | ^^\n   7 |\n   8 | export   default    42\n   9 |\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import  def , {named}  from    'x'\n\nexport *  from   'd'\n\nexport const x = 42;\n\nexport   default    42\n");
}
#[test]
fn test_module_export_3_js_format_1_b4d1e72a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .range_end(108)
        .range_start(104)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | import  def , {named}  from    'x'\n   2 |\n   3 | export *  from   'd'\n   4 |\n   5 | export    const  x\n   6 |   =  42\n   7 |\n>  8 | export   default    42\n     |                   ^^^^\n   9 |\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import  def , {named}  from    'x'\n\nexport *  from   'd'\n\nexport    const  x\n  =  42\n\nexport default 42;\n");
}
#[test]
fn test_module_import_js_format_1_0cc74eca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_start(7)
        .range_end(22)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">  1 | import  def , {named}  from    'x'\n     |        ^^^^^^^^^^^^^^^\n   2 |\n   3 | export *  from   'd'\n   4 |\n   5 | export    const  x\n   6 |   =  42\n   7 |\n   8 | export   default    42\n   9 |\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import def, { named } from \"x\";\n\nexport *  from   'd'\n\nexport    const  x\n  =  42\n\nexport   default    42\n");
}
#[test]
fn test_multiple_statements_js_format_1_799b7194() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(49)
        .range_start(30)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | call(\n   2 |   1, 2,3\n   3 | );\n   4 |\n   5 | call(\n>  6 |   1, 2,3\n     |      ^^^\n>  7 | );\n     | ^^\n>  8 |\n     | ^^\n>  9 | call(\n     | ^^\n> 10 |   1, 2,3\n     | ^^^^^\n  11 | );\n  12 |\n  13 | call(\n  14 |   1, 2,3\n  15 | );\n  16 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "call(\n  1, 2,3\n);\n\ncall(1, 2, 3);\n\ncall(1, 2, 3);\n\ncall(\n  1, 2,3\n);"
    );
}
#[test]
fn test_multiple_statements_2_js_format_1_04f9d1ea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(18)
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(56)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | call(\n   2 |   1, 2,3\n   3 | );\n>  4 |\n     | ^\n>  5 | call(\n     | ^^^^^\n>  6 |   1, 2,3\n     | ^^^^^\n>  7 | );\n     | ^^^^^\n>  8 |\n     | ^^^^^\n>  9 | call(\n     | ^^^^^\n> 10 |   1, 2,3\n     | ^^^^^\n> 11 | );\n     | ^^^^^\n> 12 |\n     | ^\n  13 | call(\n  14 |   1, 2,3\n  15 | );\n  16 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "call(\n  1, 2,3\n);\n\ncall(1, 2, 3);\n\ncall(1, 2, 3);\n\ncall(\n  1, 2,3\n);"
    );
}
#[test]
fn test_nested_js_format_1_60482964() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(0)
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(37)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | try {\n    | ^^^^^\n> 2 |   if (condition) {\n    | ^^^^^^^^^^^^^^^^^^\n> 3 |     body\n    | ^^^^^^^^^^^^^^^^^^\n> 4 |   }\n    | ^^^\n  5 | }\n  6 | catch (err) {}\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "try {\n  if (condition) {\n    body;\n  }\n} catch (err) {}"
    );
}
#[test]
fn test_nested_print_width_js_format_1_30a1b250() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(110)
        .print_width(80)
        .range_start(28)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | function f() {\n> 2 |   if (true) {\n    |              ^\n> 3 |     call(\"this line is 79 chars\", \"long\", \"it should\", \"stay as single line\");\n    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n> 4 |   }\n    | ^^\n  5 | }\n  6 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  if (true) {\n    call(\"this line is 79 chars\", \"long\", \"it should\", \"stay as single line\");\n  }\n}");
}
#[test]
fn test_nested_2_js_format_1_27512a70() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .range_end(37)
        .range_start(9)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | try {\n> 2 |   if (condition) {\n    |    ^^^^^^^^^^^^^^^\n> 3 |     body\n    | ^^^^^^^^\n> 4 |   }\n    | ^^^\n  5 | }\n  6 | catch (err) {}\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "try {\n  if (condition) {\n    body;\n  }\n}\ncatch (err) {}"
    );
}
#[test]
fn test_nested_3_js_format_1_1b8da30c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(37)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(9)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | try {\n> 2 | 1;if (condition) {\n    |    ^^^^^^^^^^^^^^^\n> 3 |     body\n    | ^^^^^^^^\n> 4 |   }\n    | ^^^\n  5 | }\n  6 | catch (err) {}\n  7 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "try {\n1;if (condition) {\n  body;\n}\n}\ncatch (err) {}"
    );
}
#[test]
fn test_object_expression_js_format_1_fad323d9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(11)
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(14)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 | const y = {a:1, b:2}\n    |            ^^^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const y = { a: 1, b: 2 };");
}
#[test]
fn test_object_expression_2_js_format_1_242abfde() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(53)
        .print_width(80)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(104)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 |\n   2 | const y =       [\n   3 |     {\n   4 |                 a: 1,\n>  5 |     },\n     |       ^\n>  6 |     {\n     | ^^^^^\n>  7 |               a: 1,\n     | ^^^^^\n>  8 |               b:2\n     | ^^^^^\n>  9 |     },\n     | ^^^^^^\n  10 | ]\n  11 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "\nconst y = [\n  {\n    a: 1,\n  },\n  {\n    a: 1,\n    b: 2,\n  },\n];"
    );
}
#[test]
fn test_range_js_format_1_bc512e0d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(224)
        .print_width(80)
        .range_end(227)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | function ugly ( {a=1,     b     =   2     }      ) {\n  2 |   function ugly ( {a=1,     b     =   2     }      ) {\n  3 |     function ugly ( {a=1,     b     =   2     }      ) {\n  4 |              \\`multiline template string\n> 5 |               with too much indentation\\`\n    |                    ^^^\n  6 |     }\n  7 |   }\n  8 | }\n  9 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function ugly ( {a=1,     b     =   2     }      ) {\n  function ugly ( {a=1,     b     =   2     }      ) {\n    function ugly ( {a=1,     b     =   2     }      ) {\n             \\`multiline template string\n              with too much indentation\\`;\n    }\n  }\n}");
}
#[test]
fn test_range_end_js_format_1_f77b464e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(50)
        .print_width(80)
        .range_start(32)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("  1 | // Unchanged\n  2 | call(\n  3 |   1, 2,3\n  4 | );\n  5 |\n> 6 |\n    | ^\n> 7 | call(\n    | ^^^^^\n> 8 |   1, 2,3\n    | ^^^^^\n> 9 | );\n    | ^") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "// Unchanged\ncall(\n  1, 2,3\n);\n\n\ncall(1, 2, 3)"
    );
}
#[test]
fn test_range_start_js_format_1_9c83562e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .range_end(18)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_start(0)
        .build()
        .unwrap();
    let formatted = pretty_printer . format (">  1 | call(\n     | ^^^^^\n>  2 |   1, 2,3\n     | ^^^^^^^^\n>  3 | );\n     | ^^^^^^^^\n>  4 |\n     | ^\n   5 |\n   6 | // Unchanged\n   7 | call(\n   8 |   1, 2,3\n   9 | );\n  10 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "call(1, 2, 3);\n\n\n// Unchanged\ncall(\n  1, 2,3\n);"
    );
}
#[test]
fn test_reversed_range_js_format_1_31e15af0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(26)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(13)
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | foo = 1.0000;bar = 1.0000;baz=1.0000;\n    |              ^^^^^^^^^^^^^ [Reversed range]\n  2 | // The range will be 26~13, should not format anything\n  3 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = 1.0000;bar = 1.0000;baz=1.0000;\n// The range will be 26~13, should not format anything");
}
#[test]
fn test_start_equals_end_js_format_1_d9cb545a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .range_start(13)
        .parsers(vec!["babel", "flow", "typescript"])
        .range_end(13)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | foo = 1.0000;bar = 1.0000;baz=1.0000;\n    |              ^\n  2 | // The range will be 13~13, should not format anything\n  3 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo = 1.0000;bar = 1.0000;baz=1.0000;\n// The range will be 13~13, should not format anything");
}
#[test]
fn test_try_catch_js_format_1_cb558dad() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_start(7)
        .range_end(8)
        .parsers(vec!["babel", "flow", "typescript"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("  1 | try {}\n> 2 | catch (err) {}\n    | ^\n  3 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "try {\n} catch (err) {}");
}
#[test]
fn test_whitespace_js_format_1_fa3b0942() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .range_end(1)
        .print_width(80)
        .range_start(0)
        .parsers(vec!["babel", "flow", "typescript"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("> 1 |  \n    | ^\n  2 ");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, " ");
}
