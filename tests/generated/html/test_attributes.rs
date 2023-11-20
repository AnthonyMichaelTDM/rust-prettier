#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_attributes_html_format_1_95a3b9f4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<input name=address maxlength=200>\n<input name='address' maxlength='200'>\n<input name=\"address\" maxlength=\"200\">\n<div class=\"foo\"></div>\n<div   class=\"foo\"   ></div>\n<div class=\"foo bar\"></div>\n<div class=\"foo bar\" id=\"header\"></div>\n<div   class=\"foo bar\"   id=\"header\"   ></div>\n<div data-prettier></div>\n<div data-prettier=\"true\"></div>\n<meta property=\"og:description\" content=\"The Mozilla Developer Network (MDN) provides\ninformation about Open Web technologies including HTML, CSS, and APIs for both Web sites\nand HTML5 Apps. It also documents Mozilla products, like Firefox OS.\">\n<div attribute>String</div>\n<div attribute=\"\">String</div>\n<div attribute=''>String</div>\n<div attribute >String</div>\n<div attribute = \"\" >String</div>\n<div attribute = '' >String</div>\n<div  attribute  >String</div>\n<div  attribute  =  \"\"  >String</div>\n<div  attribute  =  ''  >String</div>\n<div attribute=\"attribute = attribute\"></div>\n<div ATTRIBUTE>String</div>\n<div ATTRIBUTE=\"\">String</div>\n<div ATTRIBUTE=''>String</div>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\">\n</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\">...</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\">\n  ...\n</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\">\n\n</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\">\n\n\n\n</article>\n<X>\n</X>\n<X a=\"1\">\n</X>\n<X a=\"1\" b=\"2\">\n</X>\n<X a=\"1\" b=\"2\" c=\"3\">\n</X>\n<p \n  class=\"\n    foo\n    bar\n    baz\n  \"\n>\n</p>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<input name=\"address\" maxlength=\"200\" />\n<input name=\"address\" maxlength=\"200\" />\n<input name=\"address\" maxlength=\"200\" />\n<div class=\"foo\"></div>\n<div class=\"foo\"></div>\n<div class=\"foo bar\"></div>\n<div class=\"foo bar\" id=\"header\"></div>\n<div class=\"foo bar\" id=\"header\"></div>\n<div data-prettier></div>\n<div data-prettier=\"true\"></div>\n<meta\n  property=\"og:description\"\n  content=\"The Mozilla Developer Network (MDN) provides\ninformation about Open Web technologies including HTML, CSS, and APIs for both Web sites\nand HTML5 Apps. It also documents Mozilla products, like Firefox OS.\"\n/>\n<div attribute>String</div>\n<div attribute=\"\">String</div>\n<div attribute=\"\">String</div>\n<div attribute>String</div>\n<div attribute=\"\">String</div>\n<div attribute=\"\">String</div>\n<div attribute>String</div>\n<div attribute=\"\">String</div>\n<div attribute=\"\">String</div>\n<div attribute=\"attribute = attribute\"></div>\n<div ATTRIBUTE>String</div>\n<div ATTRIBUTE=\"\">String</div>\n<div ATTRIBUTE=\"\">String</div>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\"\n></article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\"\n>\n  ...\n</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\"\n>\n  ...\n</article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\"\n></article>\n<article\n  id=\"electriccars\"\n  data-columns=\"3\"\n  data-index-number=\"12314\"\n  data-parent=\"cars\"\n></article>\n<X> </X>\n<X a=\"1\"> </X>\n<X a=\"1\" b=\"2\"> </X>\n<X a=\"1\" b=\"2\" c=\"3\"> </X>\n<p class=\"foo bar baz\"></p>");
}
#[test]
fn test_boolean_html_format_1_41b1294d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<button type=\"submit\">This is valid.</button>\n<button type=\"submit\" disabled>This is valid.</button>\n<button type=\"submit\" disabled=\"\">This is valid.</button>\n<button type=\"submit\" disabled=\"disabled\">This is valid.</button>\n<button type=\"submit\" disabled=true>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled='true'>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled=\"true\">This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled=false>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled=\"false\">This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled='false'>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled=hahah>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled='hahah'>This is valid. This will be disabled.</button>\n<button type=\"submit\" disabled=\"hahah\">This is valid. This will be disabled.</button>\n<input type=\"checkbox\" checked disabled name=\"cheese\">\n<input type=\"checkbox\" checked=\"checked\" disabled=\"disabled\" name=\"cheese\">\n<input type='checkbox' checked=\"\" disabled=\"\" name=cheese >\n<div lang=\"\"></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<button type=\"submit\">This is valid.</button>\n<button type=\"submit\" disabled>This is valid.</button>\n<button type=\"submit\" disabled=\"\">This is valid.</button>\n<button type=\"submit\" disabled=\"disabled\">This is valid.</button>\n<button type=\"submit\" disabled=\"true\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"true\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"true\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"false\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"false\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"false\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"hahah\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"hahah\">\n  This is valid. This will be disabled.\n</button>\n<button type=\"submit\" disabled=\"hahah\">\n  This is valid. This will be disabled.\n</button>\n<input type=\"checkbox\" checked disabled name=\"cheese\" />\n<input type=\"checkbox\" checked=\"checked\" disabled=\"disabled\" name=\"cheese\" />\n<input type=\"checkbox\" checked=\"\" disabled=\"\" name=\"cheese\" />\n<div lang=\"\"></div>");
}
#[test]
fn test_case_sensitive_html_format_1_8f27dd9b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<div CaseSensitive></div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<div CaseSensitive></div>");
}
#[test]
fn test_class_bem_1_html_format_1_31f63510() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div class=\"ProviderMeasuresContainer__heading-row\n  d-flex\n  flex-column flex-lg-row\n  justify-content-start justify-content-lg-between\n  align-items-start align-items-lg-center\">Foo</div>\n\n<div  class=\"a-bem-block a-bem-block--with-modifer \">\n<div  class=\"a-bem-block__element a-bem-block__element--with-modifer also-another-block\" >\n<div  class=\"a-bem-block__element a-bem-block__element--with-modifer also-another-block__element\">\n</div></div> </div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  class=\"ProviderMeasuresContainer__heading-row d-flex flex-column flex-lg-row justify-content-start justify-content-lg-between align-items-start align-items-lg-center\"\n>\n  Foo\n</div>\n\n<div class=\"a-bem-block a-bem-block--with-modifer\">\n  <div\n    class=\"a-bem-block__element a-bem-block__element--with-modifer also-another-block\"\n  >\n    <div\n      class=\"a-bem-block__element a-bem-block__element--with-modifer also-another-block__element\"\n    ></div>\n  </div>\n</div>");
}
#[test]
fn test_class_bem_2_html_format_1_be67a09d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div class=\"news__header widget__content\">\n  <div class=\"news__tabs\">\n    <h1 class=\"news__tab-wrapper news__head-item\">\n      <a\n        class=\"home-link home-link_blue_yes news__tab news__tab_selected_yes mix-tabber__tab mix-tabber__tab_selected_yes\"\n        tabindex=\"0\"\n        aria-selected=\"true\"\n        aria-controls=\"news_panel_news\"\n        data-key=\"news\"\n        id=\"news_tab_news\"\n        data-stat-link=\"news.tab.link.news\"\n        data-stat-select=\"news.tab.select.news\"\n        target=\"_blank\"\n        role=\"tab\"\n        href=\"https://yandex.ru/news?msid=1581089780.29024.161826.172442&mlid=1581088893.glob_225\"\n        rel=\"noopener\"\n        >...</a\n      >\n    </h1>\n  </div>\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div class=\"news__header widget__content\">\n  <div class=\"news__tabs\">\n    <h1 class=\"news__tab-wrapper news__head-item\">\n      <a\n        class=\"home-link home-link_blue_yes news__tab news__tab_selected_yes mix-tabber__tab mix-tabber__tab_selected_yes\"\n        tabindex=\"0\"\n        aria-selected=\"true\"\n        aria-controls=\"news_panel_news\"\n        data-key=\"news\"\n        id=\"news_tab_news\"\n        data-stat-link=\"news.tab.link.news\"\n        data-stat-select=\"news.tab.select.news\"\n        target=\"_blank\"\n        role=\"tab\"\n        href=\"https://yandex.ru/news?msid=1581089780.29024.161826.172442&mlid=1581088893.glob_225\"\n        rel=\"noopener\"\n        >...</a\n      >\n    </h1>\n  </div>\n</div>");
}
#[test]
fn test_class_colon_html_format_1_0885e4f8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<my-tag class=\"md:foo-bg md:foo-color md:foo--sub-bg md:foo--sub-color xl:foo xl:prefix2 --prefix2--something-else unrelated_class_to_fill_80_chars\"></my-tag>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<my-tag\n  class=\"md:foo-bg md:foo-color md:foo--sub-bg md:foo--sub-color xl:foo xl:prefix2 --prefix2--something-else unrelated_class_to_fill_80_chars\"\n></my-tag>");
}
#[test]
fn test_class_leading_dashes_html_format_1_e7ed1b15() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<my-tag class=\"__prefix1__foo __prefix1__bar __prefix2__foo prefix2 prefix2--something --prefix2--something-else\"></my-tag>\n\n<my-tag class=\"--prefix1--foo --prefix1--bar --prefix2--foo prefix2 prefix2__something __prefix2__something_else\"></my-tag>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<my-tag\n  class=\"__prefix1__foo __prefix1__bar __prefix2__foo prefix2 prefix2--something --prefix2--something-else\"\n></my-tag>\n\n<my-tag\n  class=\"--prefix1--foo --prefix1--bar --prefix2--foo prefix2 prefix2__something __prefix2__something_else\"\n></my-tag>");
}
#[test]
fn test_class_many_short_names_html_format_1_622552cd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\">\n  Jump to\n  <span class=\"d-inline-block ml-1 v-align-middle\">x</span>\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  aria-hidden=\"true\"\n  class=\"border rounded-1 flex-shrink-0 bg-gray px-1 text-gray-light ml-1 f6 d-none d-on-nav-focus js-jump-to-badge-jump\"\n>\n  Jump to\n  <span class=\"d-inline-block ml-1 v-align-middle\">x</span>\n</div>");
}
#[test]
fn test_class_names_html_format_1_eb416ff6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<img class=\"\n                     foo\nbar\n\">\n\n<img class=\"  \">\n<img class>\n\n<img class=\"\nlooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong\na-long-long-long-long-long-class-name\nanother-long-long-long-class-name\n                     foo bar\nfoo bar\n                     foo bar\nfoo bar\n                     foo bar\nfoo bar\n                     foo bar\nfoo bar\n                     foo bar\nfoo bar\n                     foo bar\nfoo bar\n                     foo bar\n\">\n\n<img\nclass=\"{{ ...classes }}\">\n<img\nclass=\"foo bar {{ otherClass }}\">\n\n<!-- escaped -->\n<!-- from: https://developer.mozilla.org/en-US/docs/Web/API/CSS/escape#Basic_results -->\n<img class=\"\n\\\\.foo\\\\#bar\n\\\\(\\\\)\\\\[\\\\]\\\\{\\\\}\n--a\n\\\\30\n\\\\ufffd\n\">\n\n<!-- from yahoo website -->\n<div id=\"header-wrapper\" class=\"Bgc(#fff) Bdbc(t) Bdbs(s) Bdbw(1px) D(tb) Pos(f) Tbl(f) W(100%) Z(4)\nhas-scrolled_Bdc($c-fuji-grey-d) Scrolling_Bdc($c-fuji-grey-d) has-scrolled_Bxsh($headerShadow)\nScrolling_Bxsh($headerShadow) \">\n<div class=\"Bgc(#fff) M(a) Maw(1301px) Miw(1000px) Pb(12px) Pt(22px) Pos(r) TranslateZ(0) Z(6)\"\n><h1 class=\"Fz(0) Pstart(15px) Pos(a)\"><a id=\"header-logo\"\nhref=\"https://www.yahoo.com/\" class=\"D(b) Pos(r)\" data-ylk=\"elm:img;elmt:logo;sec:hd;slk:logo\">\n<img class=\"H(27px)!--sm1024 Mt(9px)!--sm1024 W(90px)!--sm1024\"\nsrc=\"https://s.yimg.com/rz/p/yahoo_frontpage_en-US_s_f_p_205x58_frontpage_2x.png\" height=\"58px\"\nwidth=\"205px\" alt=\"Yahoo\"/></a></h1></div></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<img class=\"foo bar\" />\n\n<img class=\"  \" />\n<img class />\n\n<img\n  class=\"looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong a-long-long-long-long-long-class-name another-long-long-long-class-name foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar foo bar\"\n/>\n\n<img class=\"{{ ...classes }}\" />\n<img class=\"foo bar {{ otherClass }}\" />\n\n<!-- escaped -->\n<!-- from: https://developer.mozilla.org/en-US/docs/Web/API/CSS/escape#Basic_results -->\n<img class=\"\\\\.foo\\\\#bar \\\\(\\\\)\\\\[\\\\]\\\\{\\\\} --a \\\\30 \\\\ufffd\" />\n\n<!-- from yahoo website -->\n<div\n  id=\"header-wrapper\"\n  class=\"Bgc(#fff) Bdbc(t) Bdbs(s) Bdbw(1px) D(tb) Pos(f) Tbl(f) W(100%) Z(4) has-scrolled_Bdc($c-fuji-grey-d) Scrolling_Bdc($c-fuji-grey-d) has-scrolled_Bxsh($headerShadow) Scrolling_Bxsh($headerShadow)\"\n>\n  <div\n    class=\"Bgc(#fff) M(a) Maw(1301px) Miw(1000px) Pb(12px) Pt(22px) Pos(r) TranslateZ(0) Z(6)\"\n  >\n    <h1 class=\"Fz(0) Pstart(15px) Pos(a)\">\n      <a\n        id=\"header-logo\"\n        href=\"https://www.yahoo.com/\"\n        class=\"D(b) Pos(r)\"\n        data-ylk=\"elm:img;elmt:logo;sec:hd;slk:logo\"\n      >\n        <img\n          class=\"H(27px)!--sm1024 Mt(9px)!--sm1024 W(90px)!--sm1024\"\n          src=\"https://s.yimg.com/rz/p/yahoo_frontpage_en-US_s_f_p_205x58_frontpage_2x.png\"\n          height=\"58px\"\n          width=\"205px\"\n          alt=\"Yahoo\"\n      /></a>\n    </h1>\n  </div>\n</div>");
}
#[test]
fn test_class_print_width_edge_html_format_1_c9e3ac68() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div aria-hidden=\"true\" class=\"border rounded-1 flex-shrink-0 bg-gray px-1 loooooooooooooooooooooooong\">\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  aria-hidden=\"true\"\n  class=\"border rounded-1 flex-shrink-0 bg-gray px-1 loooooooooooooooooooooooong\"\n></div>");
}
#[test]
fn test_dobule_quotes_html_format_1_354b3c8b() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<img src=\"test.png\" alt=\"John 'ShotGun' Nelson\">");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<img src=\"test.png\" alt=\"John 'ShotGun' Nelson\" />"
    );
}
#[test]
fn test_duplicate_html_format_1_67b111f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<a href=\"1\" href=\"2\">123</a>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<a href=\"1\" href=\"2\">123</a>");
}
#[test]
fn test_single_quotes_html_format_1_598fc7f6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<img src=\"test.png\" alt='John \"ShotGun\" Nelson'>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "<img src=\"test.png\" alt='John \"ShotGun\" Nelson' />"
    );
}
#[test]
fn test_smart_quotes_html_format_1_fe3821bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div \n    smart-quotes='123 \" 456'\n    smart-quotes=\"123 ' 456\"\n    smart-quotes='123 &apos;&quot; 456'\n></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  smart-quotes='123 \" 456'\n  smart-quotes=\"123 ' 456\"\n  smart-quotes=\"123 '&quot; 456\"\n></div>");
}
#[test]
fn test_srcset_html_format_1_4c4d6c08() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<img src=\"/assets/visual.png\" \nsrcset=\"/assets/visual@0.5.png  400w, /assets/visual.png      805w\" \nsizes=\"(max-width: 66rem) 100vw, 66rem\" alt=\"\"/>\n<img src=\"/assets/visual.png\" \nsrcset=\"/assets/visual@0.5.png  400w, /assets/visual.png      805w,\t/assets/visual@2x.png   1610w,\t/assets/visual@3x.png   2415w\" \nsizes=\"(max-width: 66rem) 100vw, 66rem\" alt=\"\"/>\n<img src=\"/assets/visual.png\" \nsrcset=\"/assets/visual@0.5.png  0.5x, /assets/visual.png      1111x,\t/assets/visual@2x.png   2x,\t/assets/visual@3x.png   3.3333x\" \nsizes=\"(max-width: 66rem) 100vw, 66rem\" alt=\"\"/>\n<img\n\nsrcset=\"\n             /media/examples/surfer-240-200.jpg\n\">\n<!-- #8150 -->\n<img\nsizes=\"(max-width: 1400px) 100vw, 1400px\"\nsrcset=\"\n_20200401_145009_szrhju_c_scale,w_200.jpg 200w,\n_20200401_145009_szrhju_c_scale,w_379.jpg 379w,\n_20200401_145009_szrhju_c_scale,w_515.jpg 515w,\n_20200401_145009_szrhju_c_scale,w_630.jpg 630w,\n_20200401_145009_szrhju_c_scale,w_731.jpg 731w,\n_20200401_145009_szrhju_c_scale,w_828.jpg 828w,\n_20200401_145009_szrhju_c_scale,w_921.jpg 921w,\n_20200401_145009_szrhju_c_scale,w_995.jpg 995w,\n_20200401_145009_szrhju_c_scale,w_1072.jpg 1072w,\n_20200401_145009_szrhju_c_scale,w_1145.jpg 1145w,\n_20200401_145009_szrhju_c_scale,w_1216.jpg 1216w,\n_20200401_145009_szrhju_c_scale,w_1284.jpg 1284w,\n_20200401_145009_szrhju_c_scale,w_1350.jpg 1350w,\n_20200401_145009_szrhju_c_scale,w_1398.jpg 1398w,\n_20200401_145009_szrhju_c_scale,w_1400.jpg 1400w\"\nsrc=\"_20200401_145009_szrhju_c_scale,w_1400.jpg\"\nalt=\"\">") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<img\n  src=\"/assets/visual.png\"\n  srcset=\"/assets/visual@0.5.png 400w, /assets/visual.png 805w\"\n  sizes=\"(max-width: 66rem) 100vw, 66rem\"\n  alt=\"\"\n/>\n<img\n  src=\"/assets/visual.png\"\n  srcset=\"\n    /assets/visual@0.5.png  400w,\n    /assets/visual.png      805w,\n    /assets/visual@2x.png  1610w,\n    /assets/visual@3x.png  2415w\n  \"\n  sizes=\"(max-width: 66rem) 100vw, 66rem\"\n  alt=\"\"\n/>\n<img\n  src=\"/assets/visual.png\"\n  srcset=\"\n    /assets/visual@0.5.png    0.5x,\n    /assets/visual.png     1111x,\n    /assets/visual@2x.png     2x,\n    /assets/visual@3x.png     3.3333x\n  \"\n  sizes=\"(max-width: 66rem) 100vw, 66rem\"\n  alt=\"\"\n/>\n<img srcset=\"/media/examples/surfer-240-200.jpg\" />\n<!-- #8150 -->\n<img\n  sizes=\"(max-width: 1400px) 100vw, 1400px\"\n  srcset=\"\n    _20200401_145009_szrhju_c_scale,w_200.jpg   200w,\n    _20200401_145009_szrhju_c_scale,w_379.jpg   379w,\n    _20200401_145009_szrhju_c_scale,w_515.jpg   515w,\n    _20200401_145009_szrhju_c_scale,w_630.jpg   630w,\n    _20200401_145009_szrhju_c_scale,w_731.jpg   731w,\n    _20200401_145009_szrhju_c_scale,w_828.jpg   828w,\n    _20200401_145009_szrhju_c_scale,w_921.jpg   921w,\n    _20200401_145009_szrhju_c_scale,w_995.jpg   995w,\n    _20200401_145009_szrhju_c_scale,w_1072.jpg 1072w,\n    _20200401_145009_szrhju_c_scale,w_1145.jpg 1145w,\n    _20200401_145009_szrhju_c_scale,w_1216.jpg 1216w,\n    _20200401_145009_szrhju_c_scale,w_1284.jpg 1284w,\n    _20200401_145009_szrhju_c_scale,w_1350.jpg 1350w,\n    _20200401_145009_szrhju_c_scale,w_1398.jpg 1398w,\n    _20200401_145009_szrhju_c_scale,w_1400.jpg 1400w\n  \"\n  src=\"_20200401_145009_szrhju_c_scale,w_1400.jpg\"\n  alt=\"\"\n/>");
}
#[test]
fn test_style_html_format_1_024bffce() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div style=\"\n\ncolor:\n#fFf\n\n\"></div>\n\n<div style=\" \"></div>\n<div style></div>\n\n<div style=\"\n\nall: initial;display: block;\ncontain: content;text-align: center;\n\n\n\nbackground: linear-gradient(to left, hotpink, #FFF00F, #ccc, hsla(240, 100%, 50%, .05), transparent);\nbackground: linear-gradient(to left, hsla(240, 100%, 50%, .05), red);\nmax-width: 500px;margin: 0 auto;\nborder-radius: 8px;transition: transform .2s ease-out;\n\n\"></div>\n\n\n\n<div style=\"\nbackground: linear-gradient(to left, hotpink, hsla(240, 100%, 50%, .05), transparent);\n\"></div>\n\n<div style=\"   color : red;\n            display    :inline \">\n  </div>\n\n\n<div style=\"  \n\ncolor: green;\n\ndisplay: inline \n\n\">\n  </div>\n\n<div attribute-1 attribute-2 attribute-3 attribute-4 attribute-5 attribute-6 attribute-7 \nstyle=\"css-prop-1: css-value;css-prop-2: css-value;css-prop-3: css-value;css-prop-4: css-value;\"\n attribute-1 attribute-2 attribute-3 attribute-4 attribute-5 attribute-6 attribute-7 >\n  </div>\n\n<div style=\"{{ ...styles }}\" \n></div>\n\n<div style=\"color: red; {{ otherStyles }}\"\n></div") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div style=\"color: #fff\"></div>\n\n<div style=\"\"></div>\n<div style></div>\n\n<div\n  style=\"\n    all: initial;\n    display: block;\n    contain: content;\n    text-align: center;\n\n    background: linear-gradient(\n      to left,\n      hotpink,\n      #fff00f,\n      #ccc,\n      hsla(240, 100%, 50%, 0.05),\n      transparent\n    );\n    background: linear-gradient(to left, hsla(240, 100%, 50%, 0.05), red);\n    max-width: 500px;\n    margin: 0 auto;\n    border-radius: 8px;\n    transition: transform 0.2s ease-out;\n  \"\n></div>\n\n<div\n  style=\"\n    background: linear-gradient(\n      to left,\n      hotpink,\n      hsla(240, 100%, 50%, 0.05),\n      transparent\n    );\n  \"\n></div>\n\n<div style=\"color: red; display: inline\"></div>\n\n<div\n  style=\"\n    color: green;\n\n    display: inline;\n  \"\n></div>\n\n<div\n  attribute-1\n  attribute-2\n  attribute-3\n  attribute-4\n  attribute-5\n  attribute-6\n  attribute-7\n  style=\"\n    css-prop-1: css-value;\n    css-prop-2: css-value;\n    css-prop-3: css-value;\n    css-prop-4: css-value;\n  \"\n  attribute-1\n  attribute-2\n  attribute-3\n  attribute-4\n  attribute-5\n  attribute-6\n  attribute-7\n></div>\n\n<div style=\"{{ ...styles }}\"></div>\n\n<div style=\"color: red; {{ otherStyles }}\"></div>");
}
#[test]
fn test_without_quotes_html_format_1_a9d3fa07() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<p title=Title>String</p>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "<p title=\"Title\">String</p>");
}
