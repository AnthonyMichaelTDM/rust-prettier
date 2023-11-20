#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_sfc_blocks_vue_single_attribute_per_linetrue_format_1_55e70240() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .single_attribute_per_line(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\">\nconsole.log(\"hello\")\n</script>\n\n<style scoped lang=\"scss\">\np {\n  color: red;\n}\n</style>\n\n<unknown-block foo bar=\"bar\" baz long_long_long_long_attribute></unknown-block>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_file_path.ts\">\n</script>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_long_file_path.ts\">\n</script>\n\n<script lang=\"ts\" src=\"./short\">\n</script>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template lang=\"pug\" src=\"./short\">\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\">\nconsole.log(\"hello\");\n</script>\n\n<style scoped lang=\"scss\">\np {\n  color: red;\n}\n</style>\n\n<unknown-block foo bar=\"bar\" baz long_long_long_long_attribute></unknown-block>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_file_path.ts\"></script>\n\n<script\n  lang=\"ts\"\n  src=\"./long_long_long_long_long_long_long_file_path.ts\"\n></script>\n\n<script lang=\"ts\" src=\"./short\"></script>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template\n  lang=\"pug\"\n  src=\"./long_long_long_long_long_long_long_long_file_path.pug\"\n>\n</template>\n\n<template lang=\"pug\" src=\"./short\">\n</template>");
}
#[test]
fn test_sfc_blocks_vue_format_1_55e70240() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script setup lang=\"ts\">\nconsole.log(\"hello\")\n</script>\n\n<style scoped lang=\"scss\">\np {\n  color: red;\n}\n</style>\n\n<unknown-block foo bar=\"bar\" baz long_long_long_long_attribute></unknown-block>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_file_path.ts\">\n</script>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_long_file_path.ts\">\n</script>\n\n<script lang=\"ts\" src=\"./short\">\n</script>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template lang=\"pug\" src=\"./short\">\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script setup lang=\"ts\">\nconsole.log(\"hello\");\n</script>\n\n<style scoped lang=\"scss\">\np {\n  color: red;\n}\n</style>\n\n<unknown-block foo bar=\"bar\" baz long_long_long_long_attribute></unknown-block>\n\n<script lang=\"ts\" src=\"./long_long_long_long_long_long_file_path.ts\"></script>\n\n<script\n  lang=\"ts\"\n  src=\"./long_long_long_long_long_long_long_file_path.ts\"\n></script>\n\n<script lang=\"ts\" src=\"./short\"></script>\n\n<template lang=\"pug\" src=\"./long_long_long_long_long_long_file_path.pug\">\n</template>\n\n<template\n  lang=\"pug\"\n  src=\"./long_long_long_long_long_long_long_long_file_path.pug\"\n>\n</template>\n\n<template lang=\"pug\" src=\"./short\">\n</template>");
}
#[test]
fn test_single_attribute_per_line_vue_single_attribute_per_linetrue_format_1_22b10caf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .single_attribute_per_line(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div data-a=\"1\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-a=\"Lorem ipsum dolor sit amet\" data-b=\"Lorem ipsum dolor sit amet\" data-c=\"Lorem ipsum dolor sit amet\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-long-attribute-a=\"1\" data-long-attribute-b=\"2\" data-long-attribute-c=\"3\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <img src=\"/images/foo.png\" />\n\n  <img src=\"/images/foo.png\" alt=\"bar\" />\n\n  <img src=\"/images/foo.png\" alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\" />\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div data-a=\"1\">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>\n\n  <div\n    data-a=\"1\"\n    data-b=\"2\"\n    data-c=\"3\"\n  >\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div\n    data-a=\"Lorem ipsum dolor sit amet\"\n    data-b=\"Lorem ipsum dolor sit amet\"\n    data-c=\"Lorem ipsum dolor sit amet\"\n  >\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div\n    data-long-attribute-a=\"1\"\n    data-long-attribute-b=\"2\"\n    data-long-attribute-c=\"3\"\n  >\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <img src=\"/images/foo.png\" />\n\n  <img\n    src=\"/images/foo.png\"\n    alt=\"bar\"\n  />\n\n  <img\n    src=\"/images/foo.png\"\n    alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\"\n  />\n</template>");
}
#[test]
fn test_single_attribute_per_line_vue_format_1_22b10caf() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<template>\n  <div data-a=\"1\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-a=\"Lorem ipsum dolor sit amet\" data-b=\"Lorem ipsum dolor sit amet\" data-c=\"Lorem ipsum dolor sit amet\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div data-long-attribute-a=\"1\" data-long-attribute-b=\"2\" data-long-attribute-c=\"3\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <img src=\"/images/foo.png\" />\n\n  <img src=\"/images/foo.png\" alt=\"bar\" />\n\n  <img src=\"/images/foo.png\" alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\" />\n</template>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<template>\n  <div data-a=\"1\">Lorem ipsum dolor sit amet, consectetur adipiscing elit.</div>\n\n  <div data-a=\"1\" data-b=\"2\" data-c=\"3\">\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div\n    data-a=\"Lorem ipsum dolor sit amet\"\n    data-b=\"Lorem ipsum dolor sit amet\"\n    data-c=\"Lorem ipsum dolor sit amet\"\n  >\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <div\n    data-long-attribute-a=\"1\"\n    data-long-attribute-b=\"2\"\n    data-long-attribute-c=\"3\"\n  >\n    Lorem ipsum dolor sit amet, consectetur adipiscing elit.\n  </div>\n\n  <img src=\"/images/foo.png\" />\n\n  <img src=\"/images/foo.png\" alt=\"bar\" />\n\n  <img\n    src=\"/images/foo.png\"\n    alt=\"Lorem ipsum dolor sit amet, consectetur adipiscing elit.\"\n  />\n</template>");
}
