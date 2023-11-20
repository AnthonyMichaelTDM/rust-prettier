#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attribute_expr_vue_format_1_c444b4e2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"ts\"></script>\n\n<template>\n  <comp :foo=\"   (a:string)=>1\"/>\n  <comp :foo=\" <X extends Something & AnothoerOne, Y    extends unknown[]>(x:X,y:Y)=>y.length + x.foobar.abcdefg\"/>\n  <comp :foo=\"(myFunction<T   |U>(qwerty,qwerty.qwerty?.qwerty)as any) + x.filter(abcdefg as never).join(xxx)\"/>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script lang=\"ts\"></script>\n\n<template>\n  <comp :foo=\"(a: string) => 1\" />\n  <comp\n    :foo=\"\n      <X extends Something & AnothoerOne, Y extends unknown[]>(x: X, y: Y) =>\n        y.length + x.foobar.abcdefg\n    \"\n  />\n  <comp\n    :foo=\"\n      (myFunction<T | U>(qwerty, qwerty.qwerty?.qwerty) as any) +\n      x.filter(abcdefg as never).join(xxx)\n    \"\n  />\n</template>");
}
#[test]
fn test_basic_vue_format_1_5e122296() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <p v-if=\"isFolder(file)\">{{ (   file as   mymodule.Folder    ).deadline }}</p>\n  <prettier :format=\" myFunc(  o as unknown )\" />\n</template>\n\n<script lang=\"ts\"></script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <p v-if=\"isFolder(file)\">{{ (file as mymodule.Folder).deadline }}</p>\n  <prettier :format=\"myFunc(o as unknown)\" />\n</template>\n\n<script lang=\"ts\"></script>");
}
#[test]
fn test_comment_vue_format_1_b5d45a9e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div v-if=\"\n            // leading comment\n    isFolder(   /* NOTE: I like pizza */file)\n    // trailing comment\n    \"\n  :format=\"   /* leading comment */ myFunc( /* NOTE: I like banana */ o as unknown ) /* trailing comment */\"\n  >{{ /* leading comment */ (   file as /* NOTE: I like sushi */   mymodule.Folder   ).deadline /* trailing comment */    }}</div>\n</template>\n\n<script lang=\"ts\"></script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div\n    v-if=\"\n      // leading comment\n      isFolder(/* NOTE: I like pizza */ file)\n      // trailing comment\n    \"\n    :format=\"\n      /* leading comment */ myFunc(\n        /* NOTE: I like banana */ o as unknown,\n      ) /* trailing comment */\n    \"\n  >\n    {{\n      /* leading comment */ (file as /* NOTE: I like sushi */ mymodule.Folder)\n        .deadline /* trailing comment */\n    }}\n  </div>\n</template>\n\n<script lang=\"ts\"></script>");
}
#[test]
fn test_filter_vue_format_1_856c9210() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\"></script>\n\n<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div>\n    <div class=\"allowed\">{{\n      value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2 as unknown) | anotherPipeLongJustForFun | pipeTheThird\n    }}</div>\n    <div class=\"allowed\" v-bind:something='\n      value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2 as unknown) | anotherPipeLongJustForFun | pipeTheThird\n    '></div>\n    <div class=\"allowed\" :class='\n      value | thisIsARealSuperLongFilterPipe(\"arg1\", arg2 as unknown) | anotherPipeLongJustForFun | pipeTheThird'\n    ></div>\n    <div class=\"not-allowed\" v-if='\n      value | thisIsARealSuperLongBitwiseOr(\"arg1\", arg2 as unknown) | anotherBitwiseOrLongJustForFun | bitwiseOrTheThird\n    '></div>\n  </div>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\"></script>\n\n<!-- vue filters are only allowed in v-bind and interpolation -->\n<template>\n  <div>\n    <div class=\"allowed\">\n      {{\n        value\n          | thisIsARealSuperLongFilterPipe(\"arg1\", arg2 as unknown)\n          | anotherPipeLongJustForFun\n          | pipeTheThird\n      }}\n    </div>\n    <div\n      class=\"allowed\"\n      v-bind:something=\"\n        value\n          | thisIsARealSuperLongFilterPipe('arg1', arg2 as unknown)\n          | anotherPipeLongJustForFun\n          | pipeTheThird\n      \"\n    ></div>\n    <div\n      class=\"allowed\"\n      :class=\"\n        value\n          | thisIsARealSuperLongFilterPipe('arg1', arg2 as unknown)\n          | anotherPipeLongJustForFun\n          | pipeTheThird\n      \"\n    ></div>\n    <div\n      class=\"not-allowed\"\n      v-if=\"\n        value |\n          thisIsARealSuperLongBitwiseOr('arg1', arg2 as unknown) |\n          anotherBitwiseOrLongJustForFun |\n          bitwiseOrTheThird\n      \"\n    ></div>\n  </div>\n</template>");
}
#[test]
fn test_not_working_with_non_ts_script_vue_format_1_a2121d7d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <p v-if=\"isFolder(file)\">{{ (   file as   mymodule.Folder    ).deadline }}</p>\n  <prettier :format=\" myFunc(  o as unknown )\" />\n</template>\n\n<script></script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <p v-if=\"isFolder(file)\">{{ (   file as   mymodule.Folder    ).deadline }}</p>\n  <prettier :format=\" myFunc(  o as unknown )\" />\n</template>\n\n<script></script>");
}
