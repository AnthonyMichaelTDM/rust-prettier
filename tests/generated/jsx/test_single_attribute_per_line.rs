use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_single_attribute_per_line_js_single_attribute_per_linetrue_format_1_06ec9d35() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .single_attribute_per_line(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\nconst Component = () => (\n  <div>\n    <div data-a=\"1\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-a=\"Lorem ipsum dolor sit amet\" data-b=\"Lorem ipsum dolor sit amet\" data-c=\"Lorem ipsum dolor sit amet\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-long-attribute-a=\"1\" data-long-attribute-b=\"2\" data-long-attribute-c=\"3\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <img src=\"/images/foo.png\" />\n\n    <img src=\"/images/foo.png\" alt=\"bar\" />\n\n    <img src=\"/images/foo.png\" alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\" />\n  </div>\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\nconst Component = () => (\n  <div>\n    <div data-a=\"1\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div\n      data-a=\"1\"\n      data-b=\"2\"\n      data-c=\"3\"\n    >\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div\n      data-a=\"Lorem ipsum dolor sit amet\"\n      data-b=\"Lorem ipsum dolor sit amet\"\n      data-c=\"Lorem ipsum dolor sit amet\"\n    >\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div\n      data-long-attribute-a=\"1\"\n      data-long-attribute-b=\"2\"\n      data-long-attribute-c=\"3\"\n    >\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <img src=\"/images/foo.png\" />\n\n    <img\n      src=\"/images/foo.png\"\n      alt=\"bar\"\n    />\n\n    <img\n      src=\"/images/foo.png\"\n      alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\"\n    />\n  </div>\n);");
}
#[test]
fn test_single_attribute_per_line_js_format_1_06ec9d35() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\nconst Component = () => (\n  <div>\n    <div data-a=\"1\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-a=\"Lorem ipsum dolor sit amet\" data-b=\"Lorem ipsum dolor sit amet\" data-c=\"Lorem ipsum dolor sit amet\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-long-attribute-a=\"1\" data-long-attribute-b=\"2\" data-long-attribute-c=\"3\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <img src=\"/images/foo.png\" />\n\n    <img src=\"/images/foo.png\" alt=\"bar\" />\n\n    <img src=\"/images/foo.png\" alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\" />\n  </div>\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\nconst Component = () => (\n  <div>\n    <div data-a=\"1\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div\n      data-a=\"Lorem ipsum dolor sit amet\"\n      data-b=\"Lorem ipsum dolor sit amet\"\n      data-c=\"Lorem ipsum dolor sit amet\"\n    >\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <div\n      data-long-attribute-a=\"1\"\n      data-long-attribute-b=\"2\"\n      data-long-attribute-c=\"3\"\n    >\n      Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n    </div>\n\n    <img src=\"/images/foo.png\" />\n\n    <img src=\"/images/foo.png\" alt=\"bar\" />\n\n    <img\n      src=\"/images/foo.png\"\n      alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\"\n    />\n  </div>\n);");
}
