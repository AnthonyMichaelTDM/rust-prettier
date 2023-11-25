#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lang_empty_vue_format_1_78886a2b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script lang=\"\">\nshould.\nbe.   well.formatted(\n\n);</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script lang=\"\">\nshould.be.well.formatted();\n</script>"
    );
}
#[test]
fn test_lang_js_vue_format_1_62aff037() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<script lang=\"js\">\nshould.\nbe.   well.formatted(\n\n);</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script lang=\"js\">\nshould.be.well.formatted();\n</script>"
    );
}
#[test]
fn test_lang_jsx_vue_format_1_e300bb5a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"jsx\">\nexport default {\n  data: () => ({\n    message: 'hello with jsx'\n  }),\n  render(h) {\n\n\n\n    return <div>{this.message}</div>\n  }\n}\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script lang=\"jsx\">\nexport default {\n  data: () => ({\n    message: \"hello with jsx\",\n  }),\n  render(h) {\n    return <div>{this.message}</div>;\n  },\n};\n</script>");
}
#[test]
fn test_lang_none_vue_format_1_fa40e13b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script>\nshould.\nbe.   well.formatted(\n\n);</script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script>\nshould.be.well.formatted();\n</script>"
    );
}
#[test]
fn test_lang_ts_vue_format_1_410c3e0b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>{{foo}}</div>\n</template>\n\n<script lang=\"ts\">\nexport default {\n  computed: {  foo( ): string {  return \"foo\";  },  },\n}\n</script>\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div>{{ foo }}</div>\n</template>\n\n<script lang=\"ts\">\nexport default {\n  computed: {\n    foo(): string {\n      return \"foo\";\n    },\n  },\n};\n</script>");
}
#[test]
fn test_lang_ts_multiple_script_tags_vue_format_1_c7af8c72() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script setup lang=\"ts\">\nlet x:      string |         number = 1\n</script>\n<script></script>\n<script></script>\n<script></script>\n<template>\n  <span\n  v-if=\" (x    as string).length  >     0\"\n  v-for=\"a in [1,2,  3,4,5].map(   (x  :    number) => x *   x)\"\n  :foo=\"  (x    as number).toFixed(   2) \"\n  >\n  {{ (x      as      number).toFixed(2) }}\n  </span>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script setup lang=\"ts\">\nlet x: string | number = 1;\n</script>\n<script></script>\n<script></script>\n<script></script>\n<template>\n  <span\n    v-if=\"(x as string).length > 0\"\n    v-for=\"a in [1, 2, 3, 4, 5].map((x: number) => x * x)\"\n    :foo=\"(x as number).toFixed(2)\"\n  >\n    {{ (x as number).toFixed(2) }}\n  </span>\n</template>");
}
#[test]
fn test_lang_tsx_vue_format_1_f95b77c2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"tsx\">\nimport {VNode} from \"vue\"\nexport default {\n  computed: {  foo( ):string { return \"foo\" }, },\n  render(h):VNode {  return <div>{ this.foo }</div> },\n}\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script lang=\"tsx\">\nimport { VNode } from \"vue\";\nexport default {\n  computed: {\n    foo(): string {\n      return \"foo\";\n    },\n  },\n  render(h): VNode {\n    return <div>{this.foo}</div>;\n  },\n};\n</script>");
}
#[test]
fn test_snippet_empty_format_1_ee0f4918() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\"></custom");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
}
#[test]
fn test_snippet_new_line_format_1_8b347162() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">\n \n</custom");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
}
#[test]
fn test_snippet_non_space_format_1_3ce2f35a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">\n \u{2005} \n</custom");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
}
#[test]
fn test_snippet_spaces_format_1_5ec811f1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">   </custom");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
}
#[test]
fn test_template_bind_vue_format_1_3deb1282() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n    <div v-bind:id=\" 'list-'   +  id \"></div>\n    <div v-bind:id=\" &quot;list-&quot;   +  id \"></div>\n    <div v-bind:id=\" &apos;list-&apos;   +  id \"></div>\n    <div v-bind:id=\" &apos;&quot;&apos;   +  id \"></div>\n    <div v-bind:id=\"  rawId | formatId \"></div>\n    <div v-bind:id=\" ok ? 'YES' : 'NO' \"></div>\n    <button @click=\" foo ( arg, 'string' ) \"></button>\n</template>\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'&quot;' + id\"></div>\n  <div v-bind:id=\"rawId | formatId\"></div>\n  <div v-bind:id=\"ok ? 'YES' : 'NO'\"></div>\n  <button @click=\"foo(arg, 'string')\"></button>\n</template>");
}
#[test]
fn test_template_class_vue_format_1_51fe0dcc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <h2\n    class=\"title\"\n    :class=\"{ 'issue-realtime-pre-pulse': preAnimation,\n 'issue-realtime-trigger-pulse': pulseAnimation}\"\n    v-html=\"titleHtml\"\n  >\n  </h2>\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <h2\n    class=\"title\"\n    :class=\"{\n      'issue-realtime-pre-pulse': preAnimation,\n      'issue-realtime-trigger-pulse': pulseAnimation,\n    }\"\n    v-html=\"titleHtml\"\n  ></h2>\n</template>");
}
#[test]
fn test_void_element_vue_format_1_be9c29bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\" />");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<custom lang=\"markdown\" />");
}
#[test]
fn test_vue_component_vue_format_1_8700e9f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template >\n  <h1 >{{greeting}}     world</h1 >\n  <script>kikoo ( ) </script>\n</template >\n\n<script>\nmodule  .  exports  =\n{data : function () {return {\n\tgreeting: \"Hello\"\n}}\n}\n</script>\n\n<style   scoped >\np { font-size : 2em ; text-align : center ; }\n\n  </style >\n\n<style   lang=\"postcss\" >\np { font-size : 2em ; text-align : center ; }\n\n  </style >") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <h1>{{ greeting }} world</h1>\n  <script>\n    kikoo();\n  </script>\n</template>\n\n<script>\nmodule.exports = {\n  data: function () {\n    return {\n      greeting: \"Hello\",\n    };\n  },\n};\n</script>\n\n<style scoped>\np {\n  font-size: 2em;\n  text-align: center;\n}\n</style>\n\n<style lang=\"postcss\">\np {\n  font-size: 2em;\n  text-align: center;\n}\n</style>");
}
