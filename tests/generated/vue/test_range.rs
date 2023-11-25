#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_example_vue_format_1_62a64849() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("vue")
        .print_width(80)
        .range_end(145)
        .range_start(73)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("   1 | <template>\n   2 |   <p>Templates are formatted as well...\n   3 |     </p>\n   4 | </template>\n   5 |\n>  6 | <script>\n     | ^^^^^^^^\n>  7 | let Prettier        = format => { your.js('though') }\n     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n>  8 | </script>\n     | ^^^^^^^^^\n   9 |\n  10 | <style>\n  11 | .and { css: too !important }\n  12 | </style>\n  13 ") ? ;
    assert_eq ! (formatted , "<template>\n  <p>Templates are formatted as well...</p>\n</template>\n\n<script>\nlet Prettier = (format) => {\n  your.js(\"though\");\n};\n</script>\n\n<style>\n.and {\n  css: too !important;\n}\n</style");
    Ok(())
}
