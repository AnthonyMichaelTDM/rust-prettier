use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_ts_vue_format_1_12c759f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\">\nlet x = 1;\nfunction log(...args) {\n  console.log(...args);\n}\n</script>\n\n<template>\n  <div @click=\"if (x === 1 as number) { log('hello') } else { log('nonhello') };\">{{ x }}</div>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\">\nlet x = 1;\nfunction log(...args) {\n  console.log(...args);\n}\n</script>\n\n<template>\n  <div\n    @click=\"\n      if (x === (1 as number)) {\n        log('hello');\n      } else {\n        log('nonhello');\n      }\n    \"\n  >\n    {{ x }}\n  </div>\n</template>");
}
#[test]
fn test_function_expression_ts_vue_format_1_9271e00b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\"></script>\n\n<template>\n  <div @click=\" (  x :   never) =>      null\">arrow</div>\n  <div @click=\" function(  a    :   unknown[])      {\n      console.log(    'abcdefg');\n      return;\n  }\">anonymous function</div>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\"></script>\n\n<template>\n  <div @click=\"(x: never) => null\">arrow</div>\n  <div\n    @click=\"\n      function (a: unknown[]) {\n        console.log('abcdefg');\n        return;\n      }\n    \"\n  >\n    anonymous function\n  </div>\n</template>");
}
