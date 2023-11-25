#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_x_controller_uri_builder_js_format_1_9475aad2() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @generated SignedSource<<13ca42bbc7fe91f15057861e18a77d88>>\n *\n * !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n * !! This file is a check-in of a static_upstream project!      !!\n * !!                                                            !!\n * !! You should not modify this file directly. Instead:         !!\n * !! 1) Use `fjs use-upstream` to temporarily replace this with !!\n * !!    the latest version from upstream.                       !!\n * !! 2) Make your changes, test them, etc.                      !!\n * !! 3) Use `fjs push-upstream` to copy your changes back to    !!\n * !!    static_upstream.                                        !!\n * !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n *\n * @providesModule XControllerURIBuilder\n * @typechecks\n */\n\n// ...\nclass XControllerURIBuilder {\n\n  // ...\n  getURI() {\n    // ...\n    var tokenRegex = new RegExp(/^\\{(\\?)?(.+?)\\}$/);\n    // ...\n  }\n}\n\nmodule.exports = XControllerURIBuilder;") ? ;
    assert_eq ! (formatted , "/**\n * @generated SignedSource<<13ca42bbc7fe91f15057861e18a77d88>>\n *\n * !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n * !! This file is a check-in of a static_upstream project!      !!\n * !!                                                            !!\n * !! You should not modify this file directly. Instead:         !!\n * !! 1) Use `fjs use-upstream` to temporarily replace this with !!\n * !!    the latest version from upstream.                       !!\n * !! 2) Make your changes, test them, etc.                      !!\n * !! 3) Use `fjs push-upstream` to copy your changes back to    !!\n * !!    static_upstream.                                        !!\n * !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n *\n * @providesModule XControllerURIBuilder\n * @typechecks\n */\n\n// ...\nclass XControllerURIBuilder {\n  // ...\n  getURI() {\n    // ...\n    var tokenRegex = new RegExp(/^\\{(\\?)?(.+?)\\}$/);\n    // ...\n  }\n}\n\nmodule.exports = XControllerURIBuilder;");
    Ok(())
}
