use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_html_template_literals_js_html_whitespace_sensitivityignore_format_1_90ff0b43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const nestedFun = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\`;\n  </script>\\`;\n\nconst nestedFun2 = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`\\\\\\\\n<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\\\n\\\\\\`;\n  </script>\\`;\n\nsetFoo(\n  html\\`<div>one</div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument\n);\n\nsetFoo(\n  html\\`<div>\n      <div>nested</div>\n    </div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument\n);\n\nsetFoo(\n  html\\`<div>\n    <div>nested</div>\n  </div>\\`,\n  secondArgument\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const nestedFun = /* HTML */ \\`\n  \\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`\n      <div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\n    \\\\\\`;\n  </script>\n\\`;\n\nconst nestedFun2 = /* HTML */ \\`\n  \\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`\n      <div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\n    \\\\\\`;\n  </script>\n\\`;\n\nsetFoo(\n  html\\`\n    <div>one</div>\n    <div>two</div>\n    <div>three</div>\n  \\`,\n  secondArgument,\n);\n\nsetFoo(\n  html\\`\n    <div>\n      <div>nested</div>\n    </div>\n    <div>two</div>\n    <div>three</div>\n  \\`,\n  secondArgument,\n);\n\nsetFoo(\n  html\\`\n    <div>\n      <div>nested</div>\n    </div>\n  \\`,\n  secondArgument,\n);");
}
#[test]
fn test_html_template_literals_js_format_1_90ff0b43() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const nestedFun = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\`;\n  </script>\\`;\n\nconst nestedFun2 = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`\\\\\\\\n<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\\\n\\\\\\`;\n  </script>\\`;\n\nsetFoo(\n  html\\`<div>one</div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument\n);\n\nsetFoo(\n  html\\`<div>\n      <div>nested</div>\n    </div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument\n);\n\nsetFoo(\n  html\\`<div>\n    <div>nested</div>\n  </div>\\`,\n  secondArgument\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const nestedFun = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\`;\n  </script>\\`;\n\nconst nestedFun2 = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\` <div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div> \\\\\\`;\n  </script>\\`;\n\nsetFoo(\n  html\\`<div>one</div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument,\n);\n\nsetFoo(\n  html\\`<div>\n      <div>nested</div>\n    </div>\n    <div>two</div>\n    <div>three</div>\\`,\n  secondArgument,\n);\n\nsetFoo(\n  html\\`<div>\n    <div>nested</div>\n  </div>\\`,\n  secondArgument,\n);");
}
#[test]
fn test_issue_10691_js_html_whitespace_sensitivityignore_format_1_0819a514() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default function include_photoswipe(\n\tgallery_selector = \".my-gallery\"\n) {\n\treturn /* HTML */ \\`\n\t\t<script>\n\t\t\twindow.addEventListener(\"load\", () =>\n\t\t\t\tinitPhotoSwipeFromDOM(\"\\${gallery_selector}\")\n\t\t\t);\n\t\t</script>\\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function include_photoswipe(gallery_selector = \".my-gallery\") {\n  return /* HTML */ \\`\n    <script>\n      window.addEventListener(\"load\", () =>\n        initPhotoSwipeFromDOM(\"\\${gallery_selector}\"),\n      );\n    </script>\n  \\`;\n}");
}
#[test]
fn test_issue_10691_js_format_1_0819a514() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default function include_photoswipe(\n\tgallery_selector = \".my-gallery\"\n) {\n\treturn /* HTML */ \\`\n\t\t<script>\n\t\t\twindow.addEventListener(\"load\", () =>\n\t\t\t\tinitPhotoSwipeFromDOM(\"\\${gallery_selector}\")\n\t\t\t);\n\t\t</script>\\`;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function include_photoswipe(gallery_selector = \".my-gallery\") {\n  return /* HTML */ \\` <script>\n    window.addEventListener(\"load\", () =>\n      initPhotoSwipeFromDOM(\"\\${gallery_selector}\"),\n    );\n  </script>\\`;\n}");
}
#[test]
fn test_lit_html_js_html_whitespace_sensitivityignore_format_1_53f6cb4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { LitElement, html } from '@polymer/lit-element';\n\nclass MyElement extends LitElement {\n  static get properties() {\n    return {\n      mood: { type: String }\n    };\n  }\n\n  constructor() {\n    super();\n    this.mood = 'happy';\n  }\n\n  render() {\n    return html\\`\n      <style\n      \n      \n      >\n                  .mood { color: green; }\n      </style\n      \n      \n      \n      >\n\n         Web            Components         are     <span \n      \n      \n      class=\"mood\"      >\\${\n        this.mood\n      \n      }</span\n      \n           >!\n    \\`;\n  }\n}\n\ncustomElements.define('my-element', MyElement);\n\nconst someHtml1 = html\\`<div       > hello \\${world} </div     >\\`;\nconst someHtml2 = /* HTML */ \\`<div      > hello \\${world} </div     >\\`;\n\nhtml\\`\\`\n\nhtml\\`<my-element obj=\\${obj}></my-element>\\`;\n\nhtml\\`  <\\${Footer}  >footer      content<//     >  \\`\n\nhtml\\`  <div />  \\`\n\nhtml\\`\n  <div />\n\\`\n\nhtml\\`<span>one</span><span>two</span><span>three</span>\\`;\n\nfunction HelloWorld() {\n  return html\\`\n    <h3>Bar List</h3>\n    \\${bars.map(bar => html\\`\n       <p>\\${bar}</p>\n    \\`)}\n  \\`;\n}\n\nconst trickyParens = html\\`<script> f((\\${expr}) / 2); </script>\\`;\nconst nestedFun = /* HTML */ \\`\\${outerExpr( 1 )} <script>const tpl = html\\\\\\`<div>\\\\\\${innerExpr( 1 )} \\${outerExpr( 2 )}</div>\\\\\\`</script>\\`;\n\nconst closingScriptTagShouldBeEscapedProperly = /* HTML */ \\`\n  <script>\n    const html = /* HTML */ \\\\\\`<script><\\\\\\\\/script>\\\\\\`;\n  </script>\n\\`;\n\nconst closingScriptTag2 = /* HTML */ \\`<script>const  scriptTag='<\\\\\\\\/script>'; <\\\\/script>\\`;\n\nhtml\\`\n <div style=\"\n \\${ foo}\n\"></div>\n\\`\nhtml\\`\n <div style=\\${ \n  foo\n }></div>\n\\`\n\nhtml\\`<div style=\"   color : red;\n            display    :inline \">\n  </div>\\`\n\nhtml\\`<div style=\"   color : red;\n\\${ foo}\n            display    :inline \">\n  </div>\\`\nhtml\\`<div style=\"   color : red;\n\\${ foo}:\\${bar};\n            display    :inline \">\n  </div>\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { LitElement, html } from \"@polymer/lit-element\";\n\nclass MyElement extends LitElement {\n  static get properties() {\n    return {\n      mood: { type: String },\n    };\n  }\n\n  constructor() {\n    super();\n    this.mood = \"happy\";\n  }\n\n  render() {\n    return html\\`\n      <style>\n        .mood {\n          color: green;\n        }\n      </style>\n\n      Web Components are\n      <span class=\"mood\">\\${this.mood}</span>\n      !\n    \\`;\n  }\n}\n\ncustomElements.define(\"my-element\", MyElement);\n\nconst someHtml1 = html\\`\n  <div>hello \\${world}</div>\n\\`;\nconst someHtml2 = /* HTML */ \\`\n  <div>hello \\${world}</div>\n\\`;\n\nhtml\\`\\`;\n\nhtml\\`\n  <my-element obj=\\${obj}></my-element>\n\\`;\n\nhtml\\`\n  <\\${Footer}>footer content<//>\n\\`;\n\nhtml\\`\n  <div />\n\\`;\n\nhtml\\`\n  <div />\n\\`;\n\nhtml\\`\n  <span>one</span>\n  <span>two</span>\n  <span>three</span>\n\\`;\n\nfunction HelloWorld() {\n  return html\\`\n    <h3>Bar List</h3>\n    \\${bars.map(\n      (bar) => html\\`\n        <p>\\${bar}</p>\n      \\`,\n    )}\n  \\`;\n}\n\nconst trickyParens = html\\`\n  <script>\n    f((\\${expr}) / 2);\n  </script>\n\\`;\nconst nestedFun = /* HTML */ \\`\n  \\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`\n      <div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\n    \\\\\\`;\n  </script>\n\\`;\n\nconst closingScriptTagShouldBeEscapedProperly = /* HTML */ \\`\n  <script>\n    const html = /* HTML */ \\\\\\`\n      <script><\\\\\\\\/script>\n    \\\\\\`;\n  </script>\n\\`;\n\nconst closingScriptTag2 = /* HTML */ \\`\n  <script>\n    const scriptTag = \"<\\\\\\\\/script>\";\n  </script>\n\\`;\n\nhtml\\`\n  <div\n    style=\"\n \\${foo}\n\"\n  ></div>\n\\`;\nhtml\\`\n  <div style=\\${foo}></div>\n\\`;\n\nhtml\\`\n  <div\n    style=\"   color : red;\n            display    :inline \"\n  ></div>\n\\`;\n\nhtml\\`\n  <div\n    style=\"   color : red;\n\\${foo}\n            display    :inline \"\n  ></div>\n\\`;\nhtml\\`\n  <div\n    style=\"   color : red;\n\\${foo}:\\${bar};\n            display    :inline \"\n  ></div>\n\\`;");
}
#[test]
fn test_lit_html_js_format_1_53f6cb4f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { LitElement, html } from '@polymer/lit-element';\n\nclass MyElement extends LitElement {\n  static get properties() {\n    return {\n      mood: { type: String }\n    };\n  }\n\n  constructor() {\n    super();\n    this.mood = 'happy';\n  }\n\n  render() {\n    return html\\`\n      <style\n      \n      \n      >\n                  .mood { color: green; }\n      </style\n      \n      \n      \n      >\n\n         Web            Components         are     <span \n      \n      \n      class=\"mood\"      >\\${\n        this.mood\n      \n      }</span\n      \n           >!\n    \\`;\n  }\n}\n\ncustomElements.define('my-element', MyElement);\n\nconst someHtml1 = html\\`<div       > hello \\${world} </div     >\\`;\nconst someHtml2 = /* HTML */ \\`<div      > hello \\${world} </div     >\\`;\n\nhtml\\`\\`\n\nhtml\\`<my-element obj=\\${obj}></my-element>\\`;\n\nhtml\\`  <\\${Footer}  >footer      content<//     >  \\`\n\nhtml\\`  <div />  \\`\n\nhtml\\`\n  <div />\n\\`\n\nhtml\\`<span>one</span><span>two</span><span>three</span>\\`;\n\nfunction HelloWorld() {\n  return html\\`\n    <h3>Bar List</h3>\n    \\${bars.map(bar => html\\`\n       <p>\\${bar}</p>\n    \\`)}\n  \\`;\n}\n\nconst trickyParens = html\\`<script> f((\\${expr}) / 2); </script>\\`;\nconst nestedFun = /* HTML */ \\`\\${outerExpr( 1 )} <script>const tpl = html\\\\\\`<div>\\\\\\${innerExpr( 1 )} \\${outerExpr( 2 )}</div>\\\\\\`</script>\\`;\n\nconst closingScriptTagShouldBeEscapedProperly = /* HTML */ \\`\n  <script>\n    const html = /* HTML */ \\\\\\`<script><\\\\\\\\/script>\\\\\\`;\n  </script>\n\\`;\n\nconst closingScriptTag2 = /* HTML */ \\`<script>const  scriptTag='<\\\\\\\\/script>'; <\\\\/script>\\`;\n\nhtml\\`\n <div style=\"\n \\${ foo}\n\"></div>\n\\`\nhtml\\`\n <div style=\\${ \n  foo\n }></div>\n\\`\n\nhtml\\`<div style=\"   color : red;\n            display    :inline \">\n  </div>\\`\n\nhtml\\`<div style=\"   color : red;\n\\${ foo}\n            display    :inline \">\n  </div>\\`\nhtml\\`<div style=\"   color : red;\n\\${ foo}:\\${bar};\n            display    :inline \">\n  </div>\\`") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { LitElement, html } from \"@polymer/lit-element\";\n\nclass MyElement extends LitElement {\n  static get properties() {\n    return {\n      mood: { type: String },\n    };\n  }\n\n  constructor() {\n    super();\n    this.mood = \"happy\";\n  }\n\n  render() {\n    return html\\`\n      <style>\n        .mood {\n          color: green;\n        }\n      </style>\n\n      Web Components are <span class=\"mood\">\\${this.mood}</span>!\n    \\`;\n  }\n}\n\ncustomElements.define(\"my-element\", MyElement);\n\nconst someHtml1 = html\\`<div>hello \\${world}</div>\\`;\nconst someHtml2 = /* HTML */ \\`<div>hello \\${world}</div>\\`;\n\nhtml\\`\\`;\n\nhtml\\`<my-element obj=\\${obj}></my-element>\\`;\n\nhtml\\` <\\${Footer}>footer content<//> \\`;\n\nhtml\\` <div /> \\`;\n\nhtml\\` <div /> \\`;\n\nhtml\\`<span>one</span><span>two</span><span>three</span>\\`;\n\nfunction HelloWorld() {\n  return html\\`\n    <h3>Bar List</h3>\n    \\${bars.map((bar) => html\\` <p>\\${bar}</p> \\`)}\n  \\`;\n}\n\nconst trickyParens = html\\`<script>\n  f((\\${expr}) / 2);\n</script>\\`;\nconst nestedFun = /* HTML */ \\`\\${outerExpr(1)}\n  <script>\n    const tpl = html\\\\\\`<div>\\\\\\${innerExpr(1)} \\${outerExpr(2)}</div>\\\\\\`;\n  </script>\\`;\n\nconst closingScriptTagShouldBeEscapedProperly = /* HTML */ \\`\n  <script>\n    const html = /* HTML */ \\\\\\`<script><\\\\\\\\/script>\\\\\\`;\n  </script>\n\\`;\n\nconst closingScriptTag2 = /* HTML */ \\`<script>\n  const scriptTag = \"<\\\\\\\\/script>\";\n</script>\\`;\n\nhtml\\`\n  <div\n    style=\"\n \\${foo}\n\"\n  ></div>\n\\`;\nhtml\\` <div style=\\${foo}></div> \\`;\n\nhtml\\`<div\n  style=\"   color : red;\n            display    :inline \"\n></div>\\`;\n\nhtml\\`<div\n  style=\"   color : red;\n\\${foo}\n            display    :inline \"\n></div>\\`;\nhtml\\`<div\n  style=\"   color : red;\n\\${foo}:\\${bar};\n            display    :inline \"\n></div>\\`;");
}
