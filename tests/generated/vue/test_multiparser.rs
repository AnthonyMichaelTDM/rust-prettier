#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_lang_empty_vue_format_1_78886a2b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<script lang=\"\">\nshould.\nbe.   well.formatted(\n\n);</script>")?;
    assert_eq!(
        formatted,
        "<script lang=\"\">\nshould.be.well.formatted();\n</script>"
    );
    Ok(())
}
#[test]
fn test_lang_js_vue_format_1_62aff037() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<script lang=\"js\">\nshould.\nbe.   well.formatted(\n\n);</script>")?;
    assert_eq!(
        formatted,
        "<script lang=\"js\">\nshould.be.well.formatted();\n</script>"
    );
    Ok(())
}
#[test]
fn test_lang_jsx_vue_format_1_e300bb5a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"jsx\">\nexport default {\n  data: () => ({\n    message: 'hello with jsx'\n  }),\n  render(h) {\n\n\n\n    return <div>{this.message}</div>\n  }\n}\n</script>") ? ;
    assert_eq ! (formatted , "<script lang=\"jsx\">\nexport default {\n  data: () => ({\n    message: \"hello with jsx\",\n  }),\n  render(h) {\n    return <div>{this.message}</div>;\n  },\n};\n</script>");
    Ok(())
}
#[test]
fn test_lang_none_vue_format_1_fa40e13b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted =
        pretty_printer.format("<script>\nshould.\nbe.   well.formatted(\n\n);</script>")?;
    assert_eq!(
        formatted,
        "<script>\nshould.be.well.formatted();\n</script>"
    );
    Ok(())
}
#[test]
fn test_lang_ts_vue_format_1_410c3e0b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div>{{foo}}</div>\n</template>\n\n<script lang=\"ts\">\nexport default {\n  computed: {  foo( ): string {  return \"foo\";  },  },\n}\n</script>\n") ? ;
    assert_eq ! (formatted , "<template>\n  <div>{{ foo }}</div>\n</template>\n\n<script lang=\"ts\">\nexport default {\n  computed: {\n    foo(): string {\n      return \"foo\";\n    },\n  },\n};\n</script>");
    Ok(())
}
#[test]
fn test_lang_ts_multiple_script_tags_vue_format_1_c7af8c72() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script setup lang=\"ts\">\nlet x:      string |         number = 1\n</script>\n<script></script>\n<script></script>\n<script></script>\n<template>\n  <span\n  v-if=\" (x    as string).length  >     0\"\n  v-for=\"a in [1,2,  3,4,5].map(   (x  :    number) => x *   x)\"\n  :foo=\"  (x    as number).toFixed(   2) \"\n  >\n  {{ (x      as      number).toFixed(2) }}\n  </span>\n</template>") ? ;
    assert_eq ! (formatted , "<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script></script>\n<script setup lang=\"ts\">\nlet x: string | number = 1;\n</script>\n<script></script>\n<script></script>\n<script></script>\n<template>\n  <span\n    v-if=\"(x as string).length > 0\"\n    v-for=\"a in [1, 2, 3, 4, 5].map((x: number) => x * x)\"\n    :foo=\"(x as number).toFixed(2)\"\n  >\n    {{ (x as number).toFixed(2) }}\n  </span>\n</template>");
    Ok(())
}
#[test]
fn test_lang_tsx_vue_format_1_f95b77c2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script lang=\"tsx\">\nimport {VNode} from \"vue\"\nexport default {\n  computed: {  foo( ):string { return \"foo\" }, },\n  render(h):VNode {  return <div>{ this.foo }</div> },\n}\n</script>") ? ;
    assert_eq ! (formatted , "<script lang=\"tsx\">\nimport { VNode } from \"vue\";\nexport default {\n  computed: {\n    foo(): string {\n      return \"foo\";\n    },\n  },\n  render(h): VNode {\n    return <div>{this.foo}</div>;\n  },\n};\n</script>");
    Ok(())
}
#[test]
fn test_snippet_empty_format_1_ee0f4918() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\"></custom")?;
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
    Ok(())
}
#[test]
fn test_snippet_new_line_format_1_8b347162() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">\n \n</custom")?;
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
    Ok(())
}
#[test]
fn test_snippet_non_space_format_1_3ce2f35a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">\n \u{2005} \n</custom")?;
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
    Ok(())
}
#[test]
fn test_snippet_spaces_format_1_5ec811f1() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\">   </custom")?;
    assert_eq!(formatted, "<custom lang=\"markdown\"></custom>");
    Ok(())
}
#[test]
fn test_template_bind_vue_format_1_3deb1282() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n    <div v-bind:id=\" 'list-'   +  id \"></div>\n    <div v-bind:id=\" &quot;list-&quot;   +  id \"></div>\n    <div v-bind:id=\" &apos;list-&apos;   +  id \"></div>\n    <div v-bind:id=\" &apos;&quot;&apos;   +  id \"></div>\n    <div v-bind:id=\"  rawId | formatId \"></div>\n    <div v-bind:id=\" ok ? 'YES' : 'NO' \"></div>\n    <button @click=\" foo ( arg, 'string' ) \"></button>\n</template>\n") ? ;
    assert_eq ! (formatted , "<template>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'list-' + id\"></div>\n  <div v-bind:id=\"'&quot;' + id\"></div>\n  <div v-bind:id=\"rawId | formatId\"></div>\n  <div v-bind:id=\"ok ? 'YES' : 'NO'\"></div>\n  <button @click=\"foo(arg, 'string')\"></button>\n</template>");
    Ok(())
}
#[test]
fn test_template_class_vue_format_1_51fe0dcc() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <h2\n    class=\"title\"\n    :class=\"{ 'issue-realtime-pre-pulse': preAnimation,\n 'issue-realtime-trigger-pulse': pulseAnimation}\"\n    v-html=\"titleHtml\"\n  >\n  </h2>\n</template>") ? ;
    assert_eq ! (formatted , "<template>\n  <h2\n    class=\"title\"\n    :class=\"{\n      'issue-realtime-pre-pulse': preAnimation,\n      'issue-realtime-trigger-pulse': pulseAnimation,\n    }\"\n    v-html=\"titleHtml\"\n  ></h2>\n</template>");
    Ok(())
}
#[test]
fn test_void_element_vue_format_1_be9c29bd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<custom lang=\"markdown\" />")?;
    assert_eq!(formatted, "<custom lang=\"markdown\" />");
    Ok(())
}
#[test]
fn test_vue_component_vue_format_1_8700e9f8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template >\n  <h1 >{{greeting}}     world</h1 >\n  <script>kikoo ( ) </script>\n</template >\n\n<script>\nmodule  .  exports  =\n{data : function () {return {\n\tgreeting: \"Hello\"\n}}\n}\n</script>\n\n<style   scoped >\np { font-size : 2em ; text-align : center ; }\n\n  </style >\n\n<style   lang=\"postcss\" >\np { font-size : 2em ; text-align : center ; }\n\n  </style >") ? ;
    assert_eq ! (formatted , "<template>\n  <h1>{{ greeting }} world</h1>\n  <script>\n    kikoo();\n  </script>\n</template>\n\n<script>\nmodule.exports = {\n  data: function () {\n    return {\n      greeting: \"Hello\",\n    };\n  },\n};\n</script>\n\n<style scoped>\np {\n  font-size: 2em;\n  text-align: center;\n}\n</style>\n\n<style lang=\"postcss\">\np {\n  font-size: 2em;\n  text-align: center;\n}\n</style>");
    Ok(())
}
