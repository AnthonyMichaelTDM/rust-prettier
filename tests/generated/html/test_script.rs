#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_babel_html_format_1_90dae268() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"text/babel\" data-presets=\"react\" data-type=\"module\">\nimport { h, \n         render } from 'https://unpkg.com/preact?module';\nrender(\n<h1>Hello World!</h1>,\n         document.body\n);\n</script>\n\n<script type=\"text/babel\">\n<!--\nalert(1)\n-->\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"text/babel\" data-presets=\"react\" data-type=\"module\">\n  import { h, render } from \"https://unpkg.com/preact?module\";\n  render(<h1>Hello World!</h1>, document.body);\n</script>\n\n<script type=\"text/babel\">\n  <!--\n  alert(1);\n  -->\n</script>");
}
#[test]
fn test_legacy_html_format_1_aa734793() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "\n<script>\n<!--\nalert(1)\n-->\n</script>\n\n<script>\n<!--\nalert(2)\n//-->\n</script>",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script>\n  <!--\n  alert(1);\n  -->\n</script>\n\n<script>\n  <!--\n  alert(2);\n  //-->\n</script>");
}
#[test]
fn test_module_html_format_1_7d0a01df() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .print_width(80)
        .parsers(vec!["html"])
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"module\">\nimport prettier from \"prettier/standalone\";\nimport parserGraphql from \"prettier/parser-graphql\";\n\nprettier.format(\"query { }\", {\n                      parser: \"graphql\",\n  plugins: [\nparserGraphql],\n});\n</script>\n\n<script type=\"module\">\nasync function foo() {\n  let x=10;while(x-->0)console.log(x)\n  await(import('mod'))\n}\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"module\">\n  import prettier from \"prettier/standalone\";\n  import parserGraphql from \"prettier/parser-graphql\";\n\n  prettier.format(\"query { }\", {\n    parser: \"graphql\",\n    plugins: [parserGraphql],\n  });\n</script>\n\n<script type=\"module\">\n  async function foo() {\n    let x = 10;\n    while (x-- > 0) console.log(x);\n    await import(\"mod\");\n  }\n</script>");
}
#[test]
fn test_module_attributes_html_format_1_b8d97a53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<script src=\"foo.wasm\" type=\"module\" withtype=\"webassembly\"></script>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<script src=\"foo.wasm\" type=\"module\" withtype=\"webassembly\"></script>"
    );
}
#[test]
fn test_script_html_format_1_1a0381ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["html"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script type=\"application/ld+json\">\n  {   \"json\": true }\n</script>\n<script type=\"application/json\">\n  {   \"json\":true  }\n</script>\n<script type=\"importmap\">\n  {   \"json\":true  }\n</script>\n<script type=\"systemjs-importmap\">\n  {   \"json\":true  }\n</script><script type=\"invalid\">\n  {   \"json\":false  }\n</script>\n<script type=\"text/html\">\n  <div>\n  <p>foo</p>\n  </div>\n</script>\n\n<script\n  async=\"\"\n  id=\"\"\n  src=\"/_next/static/development/pages/_app.js?ts=1565732195968\"\n></script><script></script>\n\n<!-- #8147 -->\n<script lang=\"vbscript\">\nFunction hello()\nEnd Function\n</script>\n\n<script lang=\"unknown\">\n</script>\n\n<script type=\"speculationrules\">\n  {\n   \"prerender\": [\n  {\"source\": \"list\", \"urls\": [\"https://a.test/foo\"]}\n  ]\n  }\n  </script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script type=\"application/ld+json\">\n  { \"json\": true }\n</script>\n<script type=\"application/json\">\n  { \"json\": true }\n</script>\n<script type=\"importmap\">\n  { \"json\": true }\n</script>\n<script type=\"systemjs-importmap\">\n  { \"json\": true }\n</script>\n<script type=\"invalid\">\n  {   \"json\":false  }\n</script>\n<script type=\"text/html\">\n  <div>\n    <p>foo</p>\n  </div>\n</script>\n\n<script\n  async=\"\"\n  id=\"\"\n  src=\"/_next/static/development/pages/_app.js?ts=1565732195968\"\n></script>\n<script></script>\n\n<!-- #8147 -->\n<script lang=\"vbscript\">\n  Function hello()\n  End Function\n</script>\n\n<script lang=\"unknown\"></script>\n\n<script type=\"speculationrules\">\n  {\n    \"prerender\": [{ \"source\": \"list\", \"urls\": [\"https://a.test/foo\"] }]\n  }\n</script>");
}
