#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_url_css_format_1_db4f4cb6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("css")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("div {\n  background: url(/images/bg.png);\n  background: -fb-url(/images/bg.png);\n}\n\n@font-face {\n  src: url(RobotoFlex-VariableFont_GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght.ttf);\n  src: url(foo.ttf?query=foo,bar,);\n  src: url(foo.woff2?foo=rgb\\(255,255,0\\));\n}\n\na {\n  content: url(https://example.com/\\)\\).jpg);\n  content: url(https://example.com/\\(\\(.jpg);\n  content: url(https://example.com/\\ \\ .jpg);\n  content: url(   https://example.com/\\)\\).jpg   );\n  content: url(   https://example.com/\\(\\(.jpg   );\n  content: url(   https://example.com/\\ \\ .jpg   );\n\n  background:\n    no-repeat url(https://example.com/\\)\\).jpg),\n    no-repeat url(https://example.com/\\(\\(.jpg),\n    no-repeat url(https://example.com/\\ \\ .jpg),\n    no-repeat url(   https://example.com/\\)\\).jpg   ),\n    no-repeat url(   https://example.com/\\(\\(.jpg   ),\n    no-repeat url(   https://example.com/\\ \\ .jpg   ),\n    no-repeat url(foo.ttf?query=foo,bar,),\n    no-repeat url(foo.woff2?foo=rgb\\(255,255,0\\))\n    no-repeat url(RobotoFlex-VariableFont_GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght.ttf);\n  ;\n}") ? ;
    assert_eq ! (formatted , "div {\n  background: url(/images/bg.png);\n  background: -fb-url(/images/bg.png);\n}\n\n@font-face {\n  src: url(RobotoFlex-VariableFont_GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght.ttf);\n  src: url(foo.ttf?query=foo,bar,);\n  src: url(foo.woff2?foo=rgb\\(255,255,0\\));\n}\n\na {\n  content: url(https://example.com/\\)\\).jpg);\n  content: url(https://example.com/\\(\\(.jpg);\n  content: url(https://example.com/\\ \\ .jpg);\n  content: url(   https://example.com/\\)\\).jpg   );\n  content: url(   https://example.com/\\(\\(.jpg   );\n  content: url(https://example.com/\\ \\ .jpg);\n\n  background: no-repeat url(https://example.com/\\)\\).jpg),\n    no-repeat url(https://example.com/\\(\\(.jpg),\n    no-repeat url(https://example.com/\\ \\ .jpg),\n    no-repeat url(   https://example.com/\\)\\).jpg   ),\n    no-repeat url(   https://example.com/\\(\\(.jpg   ),\n    no-repeat url(   https://example.com/\\ \\ .jpg   ),\n    no-repeat url(foo.ttf?query=foo,bar,),\n    no-repeat url(foo.woff2?foo=rgb\\(255,255,0\\))\n    no-repeat url(RobotoFlex-VariableFont_GRAD,XTRA,YOPQ,YTAS,YTDE,YTFI,YTLC,YTUC,opsz,slnt,wdth,wght.ttf);\n}");
    Ok(())
}
