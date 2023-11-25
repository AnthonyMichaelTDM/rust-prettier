use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_multiline_ts_end_of_linecr_format_1_6f3fdc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("cr")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\n  (_0: IAmIncredibleLongParameterType) => {\n    setTimeout(() => {\n      /*\n          Multiline comment\n          Multiline comment\n          Multiline comment\n      */\n      console.log(\n        'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'\n      );\n      console.log(\n        \\`Multiline \\\\n string\\\\\n         Multiline string\\\\\n         Multiline string\\`\n      );\n    });\n  }\n);\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};\rconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\rexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\r  (_0: IAmIncredibleLongParameterType) => {\r    setTimeout(() => {\r      /*\r          Multiline comment\r          Multiline comment\r          Multiline comment\r      */\r      console.log(\r        \"Multiline string\\\\\r         Multiline string\\\\\r         Multiline string\",\r      );\r      console.log(\r        \\`Multiline \\\\n string\\\\\r         Multiline string\\\\\r         Multiline string\\`,\r      );\r    });\r  },\r);\r");
}
#[test]
fn test_multiline_ts_end_of_linecrlf_format_1_6f3fdc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("crlf")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\n  (_0: IAmIncredibleLongParameterType) => {\n    setTimeout(() => {\n      /*\n          Multiline comment\n          Multiline comment\n          Multiline comment\n      */\n      console.log(\n        'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'\n      );\n      console.log(\n        \\`Multiline \\\\n string\\\\\n         Multiline string\\\\\n         Multiline string\\`\n      );\n    });\n  }\n);\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};\r\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\r\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\r\n  (_0: IAmIncredibleLongParameterType) => {\r\n    setTimeout(() => {\r\n      /*\r\n          Multiline comment\r\n          Multiline comment\r\n          Multiline comment\r\n      */\r\n      console.log(\r\n        \"Multiline string\\\\\r\n         Multiline string\\\\\r\n         Multiline string\",\r\n      );\r\n      console.log(\r\n        \\`Multiline \\\\n string\\\\\r\n         Multiline string\\\\\r\n         Multiline string\\`,\r\n      );\r\n    });\r\n  },\r\n);\r\n");
}
#[test]
fn test_multiline_ts_end_of_linelf_format_1_6f3fdc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("lf")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\n  (_0: IAmIncredibleLongParameterType) => {\n    setTimeout(() => {\n      /*\n          Multiline comment\n          Multiline comment\n          Multiline comment\n      */\n      console.log(\n        'Multiline string\\\\\n         Multiline string\\\\\n         Multiline string'\n      );\n      console.log(\n        \\`Multiline \\\\n string\\\\\n         Multiline string\\\\\n         Multiline string\\`\n      );\n    });\n  }\n);\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(\n  (_0: IAmIncredibleLongParameterType) => {\n    setTimeout(() => {\n      /*\n          Multiline comment\n          Multiline comment\n          Multiline comment\n      */\n      console.log(\n        \"Multiline string\\\\\n         Multiline string\\\\\n         Multiline string\",\n      );\n      console.log(\n        \\`Multiline \\\\n string\\\\\n         Multiline string\\\\\n         Multiline string\\`,\n      );\n    });\n  },\n);\n");
}
