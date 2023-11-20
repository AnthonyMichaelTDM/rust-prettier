#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_comments_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_comments_js_format_1_e1c354de() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const m = /*A1*/ module /*A2*/ { /*A3*/\n  /*A4*/\n  export const foo = \"foo\";\n  export { foo }; /*A5*/\n  /*A6*/\n}/*A7*/;/*A8*/\n\nconst m2 = module/* B1 */{\n  /* B2 */\n};") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const m = /*A1*/ module {\n  /*A2*/ /*A3*/\n  /*A4*/\n  export const foo = \"foo\";\n  export { foo }; /*A5*/\n  /*A6*/\n}; /*A7*/ /*A8*/\n\nconst m2 = module {\n  /* B1 */\n  /* B2 */\n};");
}
#[test]
fn test_module_blocks_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_module_blocks_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_module_blocks_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_module_blocks_js_format_1_6a0c70aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("module { await 3 };\n\nclass B {\n  #p() {\n    module {\n      class C { [this.#p]; }\n    };\n  }\n}\n\nconst m = module {\n  export const foo = \"foo\";\n  export { foo };\n};\n\nmodule {\n  export { foo }\n};\n\nconst m = module       {};\n\nconst worker = new Worker(module {\n  export const foo = \"foo\";\n});\n\nlet m = module {\n  module {\n    export let foo = \"foo\";\n  };\n};\n\nconst m = module { export const foo = \"foo\" };\n\nlet moduleBlock = module { export let y = 1; };\n\nfoo(module { export let foo = \"foo\"; });\n\nlet m = module { /* foo */ };") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "module {\n  await 3;\n};\n\nclass B {\n  #p() {\n    module {\n      class C {\n        [this.#p];\n      }\n    };\n  }\n}\n\nconst m = module {\n  export const foo = \"foo\";\n  export { foo };\n};\n\nmodule {\n  export { foo };\n};\n\nconst m = module {};\n\nconst worker = new Worker(module {\n  export const foo = \"foo\";\n});\n\nlet m = module {\n  module {\n    export let foo = \"foo\";\n  };\n};\n\nconst m = module {\n  export const foo = \"foo\";\n};\n\nlet moduleBlock = module {\n  export let y = 1;\n};\n\nfoo(module {\n  export let foo = \"foo\";\n});\n\nlet m = module {\n  /* foo */\n};");
}
#[test]
fn test_non_module_blocks_js_format_1_90ab8ae5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["babel"])
        .build()
        .unwrap();
    let formatted = pretty_printer.format("const m = module\n{}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "const m = module;\n{\n}");
}
#[test]
fn test_range_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_range_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_range_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_range_js_format_1_f5f4bf48() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .range_start(28)
        .range_end(52)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("> 1 | let moduleBlock = module {  export let y = 1;\n    |                             ^^^^^^^^^^^^^^^^^\n> 2 | };\n    | ^^\n> 3 |\n    | ^^\n> 4 | foo(module { export let foo = \"foo\"; })\n    | ^^\n  5 ") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let moduleBlock = module {\n  export let y = 1;\n};\n\nfoo(module {\n  export let foo = \"foo\";\n});");
}
#[test]
fn test_worker_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_worker_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_worker_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_worker_js_format_1_94bfe618() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["babel"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("let worker = new Worker(module {\n  onmessage = function({data}) {\n    let mod = import(data);\n    postMessage(mod.fn());\n  }\n}, {type: \"module\"});\n\nlet worker = new Worker(module {\n  onmessage = function({data}) {\n    let mod = import(data);\n    postMessage(mod.fn());\n  }\n}, {type: \"module\", foo: \"bar\" });\n\nworker.postMessage(module { export function fn() { return \"hello!\" } });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "let worker = new Worker(module {\n  onmessage = function ({ data }) {\n    let mod = import(data);\n    postMessage(mod.fn());\n  };\n}, { type: \"module\" });\n\nlet worker = new Worker(\n  module {\n    onmessage = function ({ data }) {\n      let mod = import(data);\n      postMessage(mod.fn());\n    };\n  },\n  { type: \"module\", foo: \"bar\" },\n);\n\nworker.postMessage(module {\n  export function fn() {\n    return \"hello!\";\n  }\n});");
}
