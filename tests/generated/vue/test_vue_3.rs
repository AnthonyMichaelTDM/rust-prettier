#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_script_generic_vue_format_1_04a15e2c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\" generic=\"T\"></script>\n\n<script setup lang=\"ts\" generic=\"T extends Type1 & Type2 & (Type3 | Type4), U\"></script>\n\n<script setup lang=\"ts\" generic=\"T extends Type1 & Type2 & (Type3 | Type4), U extends string | number | boolean\"></script>\n\n<script setup lang=\"ts\" generic=\"T extends | 'loooooooooooooooooooooooooooooooooong' | 'looooooooooooooooooooooooooooooooooong', U extends LooooooooooooooooooooooooooooooooongType<AnotherLoooooooooooooooongType<NonNullable<Record<string, (Type1 & Type2 & (LoooooooooooooooooooooongType3 | LoooooooooooooooooooooongType4)) | null>[string]>>>, C\" ></script>\n\n<script setup lang=\"ts\" generic=\"\n\n\n                                        // comment 1:\nT \nextends string &\n\n'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooong',\n/**\n       * comment 2\n */\n        U extends number, // comment 3\n        \n        /** comment 4 */ C extends MyType\n\n\n        \"></script>\n\n<template>\n  <!-- should not format it here -->\n  <not-script setup lang=\"ts\" generic=\"T extends Type1 & Type2 & (Type3 | Type4), U extends string | number | boolean\"></not-script>\n</template>\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\" generic=\"T\"></script>\n\n<script\n  setup\n  lang=\"ts\"\n  generic=\"T extends Type1 & Type2 & (Type3 | Type4), U\"\n></script>\n\n<script\n  setup\n  lang=\"ts\"\n  generic=\"\n    T extends Type1 & Type2 & (Type3 | Type4),\n    U extends string | number | boolean\n  \"\n></script>\n\n<script\n  setup\n  lang=\"ts\"\n  generic=\"\n    T extends\n      | 'loooooooooooooooooooooooooooooooooong'\n      | 'looooooooooooooooooooooooooooooooooong',\n    U extends LooooooooooooooooooooooooooooooooongType<\n      AnotherLoooooooooooooooongType<\n        NonNullable<\n          Record<\n            string,\n            | (Type1 &\n                Type2 &\n                (\n                  | LoooooooooooooooooooooongType3\n                  | LoooooooooooooooooooooongType4\n                ))\n            | null\n          >[string]\n        >\n      >\n    >,\n    C\n  \"\n></script>\n\n<script\n  setup\n  lang=\"ts\"\n  generic=\"\n    // comment 1:\n    T extends string &\n      'loooooooooooooooooooooooooooooooooooooooooooooooooooooooooong',\n    /**\n     * comment 2\n     */\n    U extends number, // comment 3\n    /** comment 4 */ C extends MyType\n  \"\n></script>\n\n<template>\n  <!-- should not format it here -->\n  <not-script\n    setup\n    lang=\"ts\"\n    generic=\"T extends Type1 & Type2 & (Type3 | Type4), U extends string | number | boolean\"\n  ></not-script>\n</template>");
}
#[test]
fn test_script_setup_vue_format_1_a0ef69e0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup></script>\n<script setup=\"     foo\"></script>\n<script setup=\"     {row   }\"></script>\n<script setup=\"{destructuring:{   a:{b}}}\"></script>\n\n<!-- Not script -->\n<custom setup=\"     {row   }\">Not A script</custom>\n<style setup=\"     {row   }\"></style>\n\n<!-- Not root block -->\n<template>\n<script setup=\"     {row   }\"></script>\n</template>\n\n<!-- Not attribute -->\n<script>\n<setup>{not:{a:attribute}}</setup>\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup></script>\n<script setup=\"foo\"></script>\n<script setup=\"{ row }\"></script>\n<script\n  setup=\"{\n    destructuring: {\n      a: { b },\n    },\n  }\"\n></script>\n\n<!-- Not script -->\n<custom setup=\"     {row   }\">Not A script</custom>\n<style setup=\"     {row   }\"></style>\n\n<!-- Not root block -->\n<template>\n  <script setup=\"     {row   }\"></script>\n</template>\n\n<!-- Not attribute -->\n<script>\n<setup>{not:{a:attribute}}</setup>\n</script>");
}
#[test]
fn test_style_variables_vue_format_1_4617b51c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["vue"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<style vars></style>\n<style vars=\"     foo\"></style>\n<style vars=\"     {row   }\"></style>\n<style vars=\"{destructuring:{   a:{b}}}\"></style>\n\n<!-- Not style -->\n<custom vars=\"     {row   }\">Not A style</custom>\n<script vars=\"     {row   }\"></script>\n\n<!-- Not root block -->\n<template>\n<style vars=\"     {row   }\"></style>\n</template> \n\n<!-- Not attribute -->\n<style>\n<vars>{not:{a:attribute}}</vars>\n</style>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<style vars></style>\n<style vars=\"foo\"></style>\n<style vars=\"{ row }\"></style>\n<style\n  vars=\"{\n    destructuring: {\n      a: { b },\n    },\n  }\"\n></style>\n\n<!-- Not style -->\n<custom vars=\"     {row   }\">Not A style</custom>\n<script vars=\"     {row   }\"></script>\n\n<!-- Not root block -->\n<template>\n  <style vars=\"     {row   }\"></style>\n</template>\n\n<!-- Not attribute -->\n<style>\n<vars>{not:{a:attribute}}</vars>\n</style>");
}
