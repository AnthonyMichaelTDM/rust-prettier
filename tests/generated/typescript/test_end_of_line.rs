#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
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
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};<LF>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<LF>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<LF>\n  (_0: IAmIncredibleLongParameterType) => {<LF>\n    setTimeout(() => {<LF>\n      /*<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n      */<LF>\n      console.log(<LF>\n        'Multiline string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string'<LF>\n      );<LF>\n      console.log(<LF>\n        \\`Multiline \\\\n string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string\\`<LF>\n      );<LF>\n    });<LF>\n  }<LF>\n);<LF>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};<CR>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<CR>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<CR>\n  (_0: IAmIncredibleLongParameterType) => {<CR>\n    setTimeout(() => {<CR>\n      /*<CR>\n          Multiline comment<CR>\n          Multiline comment<CR>\n          Multiline comment<CR>\n      */<CR>\n      console.log(<CR>\n        \"Multiline string\\\\<CR>\n         Multiline string\\\\<CR>\n         Multiline string\",<CR>\n      );<CR>\n      console.log(<CR>\n        \\`Multiline \\\\n string\\\\<CR>\n         Multiline string\\\\<CR>\n         Multiline string\\`,<CR>\n      );<CR>\n    });<CR>\n  },<CR>\n);<CR>");
}
#[test]
fn test_multiline_ts_end_of_linecrlf_format_1_6f3fdc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("crlf")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};<LF>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<LF>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<LF>\n  (_0: IAmIncredibleLongParameterType) => {<LF>\n    setTimeout(() => {<LF>\n      /*<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n      */<LF>\n      console.log(<LF>\n        'Multiline string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string'<LF>\n      );<LF>\n      console.log(<LF>\n        \\`Multiline \\\\n string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string\\`<LF>\n      );<LF>\n    });<LF>\n  }<LF>\n);<LF>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};<CRLF>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<CRLF>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<CRLF>\n  (_0: IAmIncredibleLongParameterType) => {<CRLF>\n    setTimeout(() => {<CRLF>\n      /*<CRLF>\n          Multiline comment<CRLF>\n          Multiline comment<CRLF>\n          Multiline comment<CRLF>\n      */<CRLF>\n      console.log(<CRLF>\n        \"Multiline string\\\\<CRLF>\n         Multiline string\\\\<CRLF>\n         Multiline string\",<CRLF>\n      );<CRLF>\n      console.log(<CRLF>\n        \\`Multiline \\\\n string\\\\<CRLF>\n         Multiline string\\\\<CRLF>\n         Multiline string\\`,<CRLF>\n      );<CRLF>\n    });<CRLF>\n  },<CRLF>\n);<CRLF>");
}
#[test]
fn test_multiline_ts_end_of_linelf_format_1_6f3fdc10() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .end_of_line("lf")
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type IAmIncredibleLongParameterType = {};<LF>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<LF>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<LF>\n  (_0: IAmIncredibleLongParameterType) => {<LF>\n    setTimeout(() => {<LF>\n      /*<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n      */<LF>\n      console.log(<LF>\n        'Multiline string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string'<LF>\n      );<LF>\n      console.log(<LF>\n        \\`Multiline \\\\n string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string\\`<LF>\n      );<LF>\n    });<LF>\n  }<LF>\n);<LF>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type IAmIncredibleLongParameterType = {};<LF>\nconst IAmAnotherFunctionName = (_0: IAmIncredibleLongParameterType) => {};<LF>\nexport const IAmIncredibleLongFunctionName = IAmAnotherFunctionName(<LF>\n  (_0: IAmIncredibleLongParameterType) => {<LF>\n    setTimeout(() => {<LF>\n      /*<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n          Multiline comment<LF>\n      */<LF>\n      console.log(<LF>\n        \"Multiline string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string\",<LF>\n      );<LF>\n      console.log(<LF>\n        \\`Multiline \\\\n string\\\\<LF>\n         Multiline string\\\\<LF>\n         Multiline string\\`,<LF>\n      );<LF>\n    });<LF>\n  },<LF>\n);<LF>");
}
