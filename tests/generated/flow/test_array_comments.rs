#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_test_js_format_1_a8764dc1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("export type FileMetaData = [\n  /* id */ string,\n  /* mtime */ number,\n  /* visited */ 0|1,\n  /* dependencies */ Array<string>,\n];\n\nexport type ModuleMetaData = [Path, /* type */ number];") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export type FileMetaData = [\n  /* id */ string,\n  /* mtime */ number,\n  /* visited */ 0 | 1,\n  /* dependencies */ Array<string>,\n];\n\nexport type ModuleMetaData = [Path, /* type */ number];");
}
