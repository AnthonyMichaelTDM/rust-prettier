#[allow(unused_imports)]
use anyhow::Result;
#[allow(unused_imports)]
use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_case_sensitive_html_html_whitespace_sensitivityignore_format_1_72ff2414() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>")?;
    assert_eq!(
        formatted,
        "<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>"
    );
    Ok(())
}
#[test]
fn test_case_sensitive_html_html_whitespace_sensitivitystrict_format_1_72ff2414() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>")?;
    assert_eq!(
        formatted,
        "<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>"
    );
    Ok(())
}
#[test]
fn test_case_sensitive_html_print_width_infinity_format_1_72ff2414() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>")?;
    assert_eq!(
        formatted,
        "<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>"
    );
    Ok(())
}
#[test]
fn test_case_sensitive_html_print_width_1_format_1_72ff2414() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>")?;
    assert_eq!(
        formatted,
        "<CaseSensitive\n  CaseSensitive=\"true\"\n  >hello\n  world</CaseSensitive\n>"
    );
    Ok(())
}
#[test]
fn test_case_sensitive_html_format_1_72ff2414() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>")?;
    assert_eq!(
        formatted,
        "<CaseSensitive CaseSensitive=\"true\">hello world</CaseSensitive>"
    );
    Ok(())
}
#[test]
fn test_closing_at_start_html_html_whitespace_sensitivityignore_format_1_176e521b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >\n    cccccccccc\n</div>\n<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >cccccccccc\n</div>") ? ;
    assert_eq ! (formatted , "<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n  >\n    bbbbbbbbbb\n  </a>\n  cccccccccc\n</div>\n<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n  >\n    bbbbbbbbbb\n  </a>\n  cccccccccc\n</div>");
    Ok(())
}
#[test]
fn test_closing_at_start_html_html_whitespace_sensitivitystrict_format_1_176e521b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >\n    cccccccccc\n</div>\n<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >cccccccccc\n</div>") ? ;
    assert_eq ! (formatted , "<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >\n  cccccccccc\n</div>\n<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >cccccccccc\n</div>");
    Ok(())
}
#[test]
fn test_closing_at_start_html_print_width_infinity_format_1_176e521b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >\n    cccccccccc\n</div>\n<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >cccccccccc\n</div>") ? ;
    assert_eq ! (formatted , "<div>\n  aaaaaaaaaa\n  <a href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\">bbbbbbbbbb</a>\n  cccccccccc\n</div>\n<div>\n  aaaaaaaaaa\n  <a href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\">bbbbbbbbbb</a>cccccccccc\n</div>");
    Ok(())
}
#[test]
fn test_closing_at_start_html_print_width_1_format_1_176e521b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >\n    cccccccccc\n</div>\n<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >cccccccccc\n</div>") ? ;
    assert_eq ! (formatted , "<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >\n  cccccccccc\n</div>\n<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >cccccccccc\n</div>");
    Ok(())
}
#[test]
fn test_closing_at_start_html_format_1_176e521b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >\n    cccccccccc\n</div>\n<div>\n    aaaaaaaaaa\n    <a\n      href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n      >bbbbbbbbbb</a\n    >cccccccccc\n</div>") ? ;
    assert_eq ! (formatted , "<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >\n  cccccccccc\n</div>\n<div>\n  aaaaaaaaaa\n  <a\n    href=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n    >bbbbbbbbbb</a\n  >cccccccccc\n</div>");
    Ok(())
}
#[test]
fn test_custom_element_html_html_whitespace_sensitivityignore_format_1_e5bb73dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<app-foo></app-foo>\n<app-bar></app-bar>")?;
    assert_eq!(formatted, "<app-foo></app-foo>\n<app-bar></app-bar>");
    Ok(())
}
#[test]
fn test_custom_element_html_html_whitespace_sensitivitystrict_format_1_e5bb73dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<app-foo></app-foo>\n<app-bar></app-bar>")?;
    assert_eq!(formatted, "<app-foo></app-foo>\n<app-bar></app-bar>");
    Ok(())
}
#[test]
fn test_custom_element_html_print_width_infinity_format_1_e5bb73dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<app-foo></app-foo>\n<app-bar></app-bar>")?;
    assert_eq!(formatted, "<app-foo></app-foo>\n<app-bar></app-bar>");
    Ok(())
}
#[test]
fn test_custom_element_html_print_width_1_format_1_e5bb73dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<app-foo></app-foo>\n<app-bar></app-bar>")?;
    assert_eq!(formatted, "<app-foo></app-foo>\n<app-bar></app-bar>");
    Ok(())
}
#[test]
fn test_custom_element_html_format_1_e5bb73dd() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<app-foo></app-foo>\n<app-bar></app-bar>")?;
    assert_eq!(formatted, "<app-foo></app-foo>\n<app-bar></app-bar>");
    Ok(())
}
#[test]
fn test_marquee_html_html_whitespace_sensitivityignore_format_1_d74f564c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border:solid\">\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>") ? ;
    assert_eq ! (formatted , "<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border: solid\"\n>\n  <marquee behavior=\"alternate\">This text will bounce</marquee>\n</marquee>");
    Ok(())
}
#[test]
fn test_marquee_html_html_whitespace_sensitivitystrict_format_1_d74f564c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border:solid\">\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>") ? ;
    assert_eq ! (formatted , "<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border: solid\"\n>\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>");
    Ok(())
}
#[test]
fn test_marquee_html_print_width_infinity_format_1_d74f564c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border:solid\">\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>") ? ;
    assert_eq ! (formatted , "<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee direction=\"down\" width=\"250\" height=\"200\" behavior=\"alternate\" style=\"border: solid\">\n  <marquee behavior=\"alternate\">This text will bounce</marquee>\n</marquee>");
    Ok(())
}
#[test]
fn test_marquee_html_print_width_1_format_1_d74f564c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border:solid\">\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>") ? ;
    assert_eq ! (formatted , "<marquee>\n  This\n  text\n  will\n  scroll\n  from\n  right\n  to\n  left\n</marquee>\n\n<marquee\n  direction=\"up\"\n>\n  This\n  text\n  will\n  scroll\n  from\n  bottom\n  to\n  top\n</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"\n    border: solid;\n  \"\n>\n  <marquee\n    behavior=\"alternate\"\n  >\n    This\n    text\n    will\n    bounce\n  </marquee>\n</marquee>");
    Ok(())
}
#[test]
fn test_marquee_html_format_1_d74f564c() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border:solid\">\n  <marquee behavior=\"alternate\"> This text will bounce </marquee>\n</marquee>") ? ;
    assert_eq ! (formatted , "<marquee>This text will scroll from right to left</marquee>\n\n<marquee direction=\"up\">This text will scroll from bottom to top</marquee>\n\n<marquee\n  direction=\"down\"\n  width=\"250\"\n  height=\"200\"\n  behavior=\"alternate\"\n  style=\"border: solid\"\n>\n  <marquee behavior=\"alternate\">This text will bounce</marquee>\n</marquee>");
    Ok(())
}
#[test]
fn test_menu_html_html_whitespace_sensitivityignore_format_1_edfa239a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>") ? ;
    assert_eq ! (formatted , "<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>");
    Ok(())
}
#[test]
fn test_menu_html_html_whitespace_sensitivitystrict_format_1_edfa239a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>") ? ;
    assert_eq ! (formatted , "<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>");
    Ok(())
}
#[test]
fn test_menu_html_print_width_infinity_format_1_edfa239a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>") ? ;
    assert_eq ! (formatted , "<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>");
    Ok(())
}
#[test]
fn test_menu_html_print_width_1_format_1_edfa239a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>") ? ;
    assert_eq ! (formatted , "<menu>\n  <li>\n    <button\n      onclick=\"copy()\"\n    >\n      Copy\n    </button>\n  </li>\n  <li>\n    <button\n      onclick=\"cut()\"\n    >\n      Cut\n    </button>\n  </li>\n  <li>\n    <button\n      onclick=\"paste()\"\n    >\n      Paste\n    </button>\n  </li>\n</menu>");
    Ok(())
}
#[test]
fn test_menu_html_format_1_edfa239a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>") ? ;
    assert_eq ! (formatted , "<menu>\n  <li><button onclick=\"copy()\">Copy</button></li>\n  <li><button onclick=\"cut()\">Cut</button></li>\n  <li><button onclick=\"paste()\">Paste</button></li>\n</menu>");
    Ok(())
}
#[test]
fn test_openging_at_end_html_html_whitespace_sensitivityignore_format_1_b8eccaf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p\n  >Want to write us a letter? Use our<a\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n  href1=\"contacts.html#Mailing_address\"\n  href2=\"contacts.html#Mailing_address\"\n  href3=\"contacts.html#Mailing_address\"\n  href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>") ? ;
    assert_eq ! (formatted , "<p>\n  Want to write us a letter? Use our\n  <a>\n    <b><a>mailing address</a></b>\n  </a>\n  .\n</p>\n\n<p>\n  Want to write us a letter? Use our\n  <a href=\"contacts.html#Mailing_address\">\n    <b><a>mailing address</a></b>\n  </a>\n  .\n</p>\n\n<p>\n  Want to write us a letter? Use our\n  <a\n    href=\"contacts.html#Mailing_address\"\n    href1=\"contacts.html#Mailing_address\"\n    href2=\"contacts.html#Mailing_address\"\n    href3=\"contacts.html#Mailing_address\"\n    href4=\"contacts.html#Mailing_address\"\n  >\n    <b><a>mailing address</a></b>\n  </a>\n  .\n</p>");
    Ok(())
}
#[test]
fn test_openging_at_end_html_html_whitespace_sensitivitystrict_format_1_b8eccaf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p\n  >Want to write us a letter? Use our<a\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n  href1=\"contacts.html#Mailing_address\"\n  href2=\"contacts.html#Mailing_address\"\n  href3=\"contacts.html#Mailing_address\"\n  href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>") ? ;
    assert_eq ! (formatted , "<p\n  >Want to write us a letter? Use our<a\n    ><b><a>mailing address</a></b></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a href=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n    href=\"contacts.html#Mailing_address\"\n    href1=\"contacts.html#Mailing_address\"\n    href2=\"contacts.html#Mailing_address\"\n    href3=\"contacts.html#Mailing_address\"\n    href4=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.</p\n>");
    Ok(())
}
#[test]
fn test_openging_at_end_html_print_width_infinity_format_1_b8eccaf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p\n  >Want to write us a letter? Use our<a\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n  href1=\"contacts.html#Mailing_address\"\n  href2=\"contacts.html#Mailing_address\"\n  href3=\"contacts.html#Mailing_address\"\n  href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>") ? ;
    assert_eq ! (formatted , "<p>\n  Want to write us a letter? Use our<a\n    ><b><a>mailing address</a></b></a\n  >.\n</p>\n\n<p>\n  Want to write us a letter? Use our<a href=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.\n</p>\n\n<p>\n  Want to write us a letter? Use our<a href=\"contacts.html#Mailing_address\" href1=\"contacts.html#Mailing_address\" href2=\"contacts.html#Mailing_address\" href3=\"contacts.html#Mailing_address\" href4=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.\n</p>");
    Ok(())
}
#[test]
fn test_openging_at_end_html_print_width_1_format_1_b8eccaf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p\n  >Want to write us a letter? Use our<a\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n  href1=\"contacts.html#Mailing_address\"\n  href2=\"contacts.html#Mailing_address\"\n  href3=\"contacts.html#Mailing_address\"\n  href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>") ? ;
    assert_eq ! (formatted , "<p>\n  Want\n  to\n  write\n  us\n  a\n  letter?\n  Use\n  our<a\n    ><b\n      ><a\n        >mailing\n        address</a\n      ></b\n    ></a\n  >.\n</p>\n\n<p>\n  Want\n  to\n  write\n  us\n  a\n  letter?\n  Use\n  our<a\n    href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a\n        >mailing\n        address</a\n      ></b\n    ></a\n  >.\n</p>\n\n<p>\n  Want\n  to\n  write\n  us\n  a\n  letter?\n  Use\n  our<a\n    href=\"contacts.html#Mailing_address\"\n    href1=\"contacts.html#Mailing_address\"\n    href2=\"contacts.html#Mailing_address\"\n    href3=\"contacts.html#Mailing_address\"\n    href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a\n        >mailing\n        address</a\n      ></b\n    ></a\n  >.\n</p>");
    Ok(())
}
#[test]
fn test_openging_at_end_html_format_1_b8eccaf8() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<p\n  >Want to write us a letter? Use our<a\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>\n\n<p\n  >Want to write us a letter? Use our<a\n  href=\"contacts.html#Mailing_address\"\n  href1=\"contacts.html#Mailing_address\"\n  href2=\"contacts.html#Mailing_address\"\n  href3=\"contacts.html#Mailing_address\"\n  href4=\"contacts.html#Mailing_address\"\n    ><b\n      ><a>mailing address</a></b\n    ></a\n  >.</p\n>") ? ;
    assert_eq ! (formatted , "<p>\n  Want to write us a letter? Use our<a\n    ><b><a>mailing address</a></b></a\n  >.\n</p>\n\n<p>\n  Want to write us a letter? Use our<a href=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.\n</p>\n\n<p>\n  Want to write us a letter? Use our<a\n    href=\"contacts.html#Mailing_address\"\n    href1=\"contacts.html#Mailing_address\"\n    href2=\"contacts.html#Mailing_address\"\n    href3=\"contacts.html#Mailing_address\"\n    href4=\"contacts.html#Mailing_address\"\n    ><b><a>mailing address</a></b></a\n  >.\n</p>");
    Ok(())
}
#[test]
fn test_option_html_html_whitespace_sensitivityignore_format_1_ac49371e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<select><option>Blue</option><option>Green</option><optgroup label=\"Darker\"><option>Dark Blue</option><option>Dark Green</option></optgroup></select>\n<input list=colors>\n<datalist id=colors><option>Blue</option><option>Green</option></datalist>") ? ;
    assert_eq ! (formatted , "<select>\n  <option>Blue</option>\n  <option>Green</option>\n  <optgroup label=\"Darker\">\n    <option>Dark Blue</option>\n    <option>Dark Green</option>\n  </optgroup>\n</select>\n<input list=\"colors\" />\n<datalist id=\"colors\">\n  <option>Blue</option>\n  <option>Green</option>\n</datalist>");
    Ok(())
}
#[test]
fn test_option_html_html_whitespace_sensitivitystrict_format_1_ac49371e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<select><option>Blue</option><option>Green</option><optgroup label=\"Darker\"><option>Dark Blue</option><option>Dark Green</option></optgroup></select>\n<input list=colors>\n<datalist id=colors><option>Blue</option><option>Green</option></datalist>") ? ;
    assert_eq ! (formatted , "<select\n  ><option>Blue</option\n  ><option>Green</option\n  ><optgroup label=\"Darker\"\n    ><option>Dark Blue</option><option>Dark Green</option></optgroup\n  ></select\n>\n<input list=\"colors\" />\n<datalist id=\"colors\"><option>Blue</option><option>Green</option></datalist>");
    Ok(())
}
#[test]
fn test_option_html_print_width_infinity_format_1_ac49371e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<select><option>Blue</option><option>Green</option><optgroup label=\"Darker\"><option>Dark Blue</option><option>Dark Green</option></optgroup></select>\n<input list=colors>\n<datalist id=colors><option>Blue</option><option>Green</option></datalist>") ? ;
    assert_eq ! (formatted , "<select>\n  <option>Blue</option>\n  <option>Green</option>\n  <optgroup label=\"Darker\">\n    <option>Dark Blue</option>\n    <option>Dark Green</option>\n  </optgroup>\n</select>\n<input list=\"colors\" />\n<datalist id=\"colors\">\n  <option>Blue</option>\n  <option>Green</option>\n</datalist>");
    Ok(())
}
#[test]
fn test_option_html_print_width_1_format_1_ac49371e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<select><option>Blue</option><option>Green</option><optgroup label=\"Darker\"><option>Dark Blue</option><option>Dark Green</option></optgroup></select>\n<input list=colors>\n<datalist id=colors><option>Blue</option><option>Green</option></datalist>") ? ;
    assert_eq ! (formatted , "<select>\n  <option>\n    Blue\n  </option>\n  <option>\n    Green\n  </option>\n  <optgroup\n    label=\"Darker\"\n  >\n    <option>\n      Dark\n      Blue\n    </option>\n    <option>\n      Dark\n      Green\n    </option>\n  </optgroup>\n</select>\n<input\n  list=\"colors\"\n/>\n<datalist\n  id=\"colors\"\n>\n  <option>\n    Blue\n  </option>\n  <option>\n    Green\n  </option>\n</datalist>");
    Ok(())
}
#[test]
fn test_option_html_format_1_ac49371e() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<select><option>Blue</option><option>Green</option><optgroup label=\"Darker\"><option>Dark Blue</option><option>Dark Green</option></optgroup></select>\n<input list=colors>\n<datalist id=colors><option>Blue</option><option>Green</option></datalist>") ? ;
    assert_eq ! (formatted , "<select>\n  <option>Blue</option>\n  <option>Green</option>\n  <optgroup label=\"Darker\">\n    <option>Dark Blue</option>\n    <option>Dark Green</option>\n  </optgroup>\n</select>\n<input list=\"colors\" />\n<datalist id=\"colors\">\n  <option>Blue</option>\n  <option>Green</option>\n</datalist>");
    Ok(())
}
#[test]
fn test_pre_html_html_whitespace_sensitivityignore_format_1_9f800772() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>Foo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.\n  </figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre></details>\n\n<details><pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<PRE><HR></PRE>\n<pre><br/></pre>\n<PRE><HR/></PRE>\n<pre><br /></pre>\n<PRE><HR /></PRE>\n<pre><span></span></pre>\n<PRE><DIV></DIV></PRE>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>") ? ;
    assert_eq ! (formatted , "<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>\nFoo     Bar\n</pre>\n<pre>     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using\n    preformatted text characters.\n  </figcaption>\n</figure>\n<pre\n  data-attr-1=\"foo\"\n  data-attr-2=\"foo\"\n  data-attr-3=\"foo\"\n  data-attr-4=\"foo\"\n  data-attr-5=\"foo\"\n  data-attr-6=\"foo\"\n>\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre>\n</details>\n\n<details>\n  <pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<pre><HR></pre>\n<pre><br/></pre>\n<pre><HR/></pre>\n<pre><br /></pre>\n<pre><HR /></pre>\n<pre><span></span></pre>\n<pre><DIV></DIV></pre>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>");
    Ok(())
}
#[test]
fn test_pre_html_html_whitespace_sensitivitystrict_format_1_9f800772() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>Foo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.\n  </figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre></details>\n\n<details><pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<PRE><HR></PRE>\n<pre><br/></pre>\n<PRE><HR/></PRE>\n<pre><br /></pre>\n<PRE><HR /></PRE>\n<pre><span></span></pre>\n<PRE><DIV></DIV></PRE>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>") ? ;
    assert_eq ! (formatted , "<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>\nFoo     Bar\n</pre>\n<pre>     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using\n    preformatted text characters.\n  </figcaption>\n</figure>\n<pre\n  data-attr-1=\"foo\"\n  data-attr-2=\"foo\"\n  data-attr-3=\"foo\"\n  data-attr-4=\"foo\"\n  data-attr-5=\"foo\"\n  data-attr-6=\"foo\"\n>\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre>\n</details>\n\n<details>\n  <pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<pre><HR></pre>\n<pre><br/></pre>\n<pre><HR/></pre>\n<pre><br /></pre>\n<pre><HR /></pre>\n<pre><span></span></pre>\n<pre><DIV></DIV></pre>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>");
    Ok(())
}
#[test]
fn test_pre_html_print_width_infinity_format_1_9f800772() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>Foo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.\n  </figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre></details>\n\n<details><pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<PRE><HR></PRE>\n<pre><br/></pre>\n<PRE><HR/></PRE>\n<pre><br /></pre>\n<PRE><HR /></PRE>\n<pre><span></span></pre>\n<PRE><DIV></DIV></PRE>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>") ? ;
    assert_eq ! (formatted , "<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>\nFoo     Bar\n</pre>\n<pre>     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.</figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre>\n</details>\n\n<details>\n  <pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<pre><HR></pre>\n<pre><br/></pre>\n<pre><HR/></pre>\n<pre><br /></pre>\n<pre><HR /></pre>\n<pre><span></span></pre>\n<pre><DIV></DIV></pre>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>");
    Ok(())
}
#[test]
fn test_pre_html_print_width_1_format_1_9f800772() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>Foo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.\n  </figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre></details>\n\n<details><pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<PRE><HR></PRE>\n<pre><br/></pre>\n<PRE><HR/></PRE>\n<pre><br /></pre>\n<PRE><HR /></PRE>\n<pre><span></span></pre>\n<PRE><DIV></DIV></PRE>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>") ? ;
    assert_eq ! (formatted , "<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>\n     Foo     Bar     </pre\n>\n<pre>\n     Foo     Bar\n</pre>\n<pre>\nFoo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre\n>\n<figure\n  role=\"img\"\n  aria-labelledby=\"cow-caption\"\n>\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption\n    id=\"cow-caption\"\n  >\n    A\n    cow\n    saying,\n    \"I'm\n    an\n    expert\n    in\n    my\n    field.\"\n    The\n    cow\n    is\n    illustrated\n    using\n    preformatted\n    text\n    characters.\n  </figcaption>\n</figure>\n<pre\n  data-attr-1=\"foo\"\n  data-attr-2=\"foo\"\n  data-attr-3=\"foo\"\n  data-attr-4=\"foo\"\n  data-attr-5=\"foo\"\n  data-attr-6=\"foo\"\n>\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre>\n</details>\n\n<details>\n  <pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<pre><HR></pre>\n<pre><br/></pre>\n<pre><HR/></pre>\n<pre><br /></pre>\n<pre><HR /></pre>\n<pre><span></span></pre>\n<pre><DIV></DIV></pre>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>");
    Ok(())
}
#[test]
fn test_pre_html_format_1_9f800772() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>Foo     Bar\n</pre>\n<pre>\n     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using preformatted text characters.\n  </figcaption>\n</figure>\n<pre data-attr-1=\"foo\" data-attr-2=\"foo\" data-attr-3=\"foo\" data-attr-4=\"foo\" data-attr-5=\"foo\" data-attr-6=\"foo\">\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre></details>\n\n<details><pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<PRE><HR></PRE>\n<pre><br/></pre>\n<PRE><HR/></PRE>\n<pre><br /></pre>\n<PRE><HR /></PRE>\n<pre><span></span></pre>\n<PRE><DIV></DIV></PRE>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>") ? ;
    assert_eq ! (formatted , "<pre>\n--------------------------------------------------------------------------------\n\n\n                                      *         *       *\n                                     **        **      ***\n                                     **        **       *\n   ****    ***  ****               ********  ********                   ***  ****\n  * ***  *  **** **** *    ***    ********  ********  ***        ***     **** **** *\n *   ****    **   ****    * ***      **        **      ***      * ***     **   ****\n**    **     **          *   ***     **        **       **     *   ***    **\n**    **     **         **    ***    **        **       **    **    ***   **\n**    **     **         ********     **        **       **    ********    **\n**    **     **         *******      **        **       **    *******     **\n**    **     **         **           **        **       **    **          **\n*******      ***        ****    *    **        **       **    ****    *   ***\n******        ***        *******      **        **      *** *  *******     ***\n**                        *****                          ***    *****\n**\n**\n **\n\n--------------------------------------------------------------------------------\n</pre>\n<pre>\n\n        Text in a pre element\n\n    is displayed in a fixed-width\n\n   font, and it preserves\n\n   both             spaces and\n\n   line breaks\n\n</pre>\n<pre>     Foo     Bar     </pre>\n<pre>\n     Foo     Bar\n</pre>\n<pre>\nFoo     Bar\n</pre>\n<pre>     Foo     Bar</pre>\n<figure role=\"img\" aria-labelledby=\"cow-caption\">\n  <pre>\n___________________________\n< I'm an expert in my field. >\n---------------------------\n     \\   ^__^\n      \\  (oo)\\_______\n         (__)\\       )\\/\\\n             ||----w |\n             ||     ||\n___________________________\n  </pre>\n  <figcaption id=\"cow-caption\">\n    A cow saying, \"I'm an expert in my field.\" The cow is illustrated using\n    preformatted text characters.\n  </figcaption>\n</figure>\n<pre\n  data-attr-1=\"foo\"\n  data-attr-2=\"foo\"\n  data-attr-3=\"foo\"\n  data-attr-4=\"foo\"\n  data-attr-5=\"foo\"\n  data-attr-6=\"foo\"\n>\n     Foo     Bar\n</pre>\n<div>\n  <div>\n    <div>\n      <div>\n        <pre>\n          ______\n          STRING\n          ______\n        </pre>\n      </div>\n    </div>\n  </div>\n</div>\n<pre></pre>\n\n<pre><code #foo></code></pre>\n\n<details>\n  <pre><!--Comments-->\n  </pre>\n</details>\n\n<details>\n  <pre>\n  <!--Comments-->\n</pre>\n</details>\n\n<!-- #6028 -->\n<pre><br></pre>\n<pre><HR></pre>\n<pre><br/></pre>\n<pre><HR/></pre>\n<pre><br /></pre>\n<pre><HR /></pre>\n<pre><span></span></pre>\n<pre><DIV></DIV></pre>\n<pre><br/>long long long text long long long text long long long text long long long text <br></pre>\n<pre><br>long long long text long long long text long long long text long long long text <BR/></pre>");
    Ok(())
}
#[test]
fn test_seach_html_html_whitespace_sensitivityignore_format_1_f566638a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<header>\n  <h1><a href=\"/\">My fancy blog</a></h1>\n  ...\n  <search>\n    <form action=\"search.php\">\n      <label for=\"query\">Find an article</label>\n      <input id=\"query\" name=\"q\" type=\"search\">\n      <button type=\"submit\">Go!</button>\n    </form>\n  </search>\n\n  <SEARCH></SEARCH>\n</header>\n</body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <header>\n      <h1><a href=\"/\">My fancy blog</a></h1>\n      ...\n      <search>\n        <form action=\"search.php\">\n          <label for=\"query\">Find an article</label>\n          <input id=\"query\" name=\"q\" type=\"search\" />\n          <button type=\"submit\">Go!</button>\n        </form>\n      </search>\n\n      <search></search>\n    </header>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_seach_html_html_whitespace_sensitivitystrict_format_1_f566638a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<header>\n  <h1><a href=\"/\">My fancy blog</a></h1>\n  ...\n  <search>\n    <form action=\"search.php\">\n      <label for=\"query\">Find an article</label>\n      <input id=\"query\" name=\"q\" type=\"search\">\n      <button type=\"submit\">Go!</button>\n    </form>\n  </search>\n\n  <SEARCH></SEARCH>\n</header>\n</body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <header>\n      <h1><a href=\"/\">My fancy blog</a></h1>\n      ...\n      <search>\n        <form action=\"search.php\">\n          <label for=\"query\">Find an article</label>\n          <input id=\"query\" name=\"q\" type=\"search\" />\n          <button type=\"submit\">Go!</button>\n        </form>\n      </search>\n\n      <search></search>\n    </header>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_seach_html_print_width_infinity_format_1_f566638a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<header>\n  <h1><a href=\"/\">My fancy blog</a></h1>\n  ...\n  <search>\n    <form action=\"search.php\">\n      <label for=\"query\">Find an article</label>\n      <input id=\"query\" name=\"q\" type=\"search\">\n      <button type=\"submit\">Go!</button>\n    </form>\n  </search>\n\n  <SEARCH></SEARCH>\n</header>\n</body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <header>\n      <h1><a href=\"/\">My fancy blog</a></h1>\n      ...\n      <search>\n        <form action=\"search.php\">\n          <label for=\"query\">Find an article</label>\n          <input id=\"query\" name=\"q\" type=\"search\" />\n          <button type=\"submit\">Go!</button>\n        </form>\n      </search>\n\n      <search></search>\n    </header>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_seach_html_print_width_1_format_1_f566638a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<header>\n  <h1><a href=\"/\">My fancy blog</a></h1>\n  ...\n  <search>\n    <form action=\"search.php\">\n      <label for=\"query\">Find an article</label>\n      <input id=\"query\" name=\"q\" type=\"search\">\n      <button type=\"submit\">Go!</button>\n    </form>\n  </search>\n\n  <SEARCH></SEARCH>\n</header>\n</body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html\n  lang=\"en\"\n>\n  <head>\n    <meta\n      charset=\"UTF-8\"\n    />\n    <meta\n      http-equiv=\"X-UA-Compatible\"\n      content=\"IE=edge\"\n    />\n    <meta\n      name=\"viewport\"\n      content=\"width=device-width, initial-scale=1.0\"\n    />\n    <title>\n      Document\n    </title>\n  </head>\n  <body>\n    <header>\n      <h1>\n        <a\n          href=\"/\"\n          >My\n          fancy\n          blog</a\n        >\n      </h1>\n      ...\n      <search>\n        <form\n          action=\"search.php\"\n        >\n          <label\n            for=\"query\"\n            >Find\n            an\n            article</label\n          >\n          <input\n            id=\"query\"\n            name=\"q\"\n            type=\"search\"\n          />\n          <button\n            type=\"submit\"\n          >\n            Go!\n          </button>\n        </form>\n      </search>\n\n      <search></search>\n    </header>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_seach_html_format_1_f566638a() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n  <meta charset=\"UTF-8\">\n  <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n  <title>Document</title>\n</head>\n<body>\n<header>\n  <h1><a href=\"/\">My fancy blog</a></h1>\n  ...\n  <search>\n    <form action=\"search.php\">\n      <label for=\"query\">Find an article</label>\n      <input id=\"query\" name=\"q\" type=\"search\">\n      <button type=\"submit\">Go!</button>\n    </form>\n  </search>\n\n  <SEARCH></SEARCH>\n</header>\n</body>\n</html>") ? ;
    assert_eq ! (formatted , "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Document</title>\n  </head>\n  <body>\n    <header>\n      <h1><a href=\"/\">My fancy blog</a></h1>\n      ...\n      <search>\n        <form action=\"search.php\">\n          <label for=\"query\">Find an article</label>\n          <input id=\"query\" name=\"q\" type=\"search\" />\n          <button type=\"submit\">Go!</button>\n        </form>\n      </search>\n\n      <search></search>\n    </header>\n  </body>\n</html>");
    Ok(())
}
#[test]
fn test_tags_html_html_whitespace_sensitivityignore_format_1_3dd5858b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<br/>\n<br />\n<br  />\n<br\n/>\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br\n  attribute-a=\"value\"\n/>\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\">\n  <source src=\"movie.ogg\" type=\"video/ogg\">\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div><div><div>string</div></div><div>string</div></div>\n<div><div>string</div><div><div>string</div></div></div>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div><div><div><div><div><div><div>string</div></div></div></div></div></div></div>\n<div>\n  <div>string</div>\n</div>\n<div>\n\n  <div>string</div>\n\n</div>\n<div>\n\n  <div>string</div>\n\n  <div>string</div>\n\n</div>\n<ul\n  >123<li\n    class=\"foo\"\n    id=\"bar\"\n  >First</li\n  >456<li\n    class=\"baz\"\n  >Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr/>456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a></p>\n<p>x<meta></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n<app-nav></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-nav [input]=\"something\"></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-primary-navigation></app-primary-navigation>\n<router-outlet></router-outlet>\n<app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><SPAN>tag name in other namespace should also lower cased</SPAN></x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr>\n</tr>\n</table><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\"/> </span>\n<span><span><input type=\"checkbox\"/></span></span>\n<span><input type=\"checkbox\"/></span>") ? ;
    assert_eq ! (formatted , "<br />\n<br />\n<br />\n<br />\n<br attribute-a />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n/>\n<br attribute-a=\"value\" />\n<br attribute-a=\"value\" />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n/>\n<br\n  attribute-a=\"value\"\n  attribute-b=\"value\"\n  attribute-c=\"value\"\n  attribute-d=\"value\"\n  attribute-e=\"value\"\n  attribute-f=\"value\"\n/>\n<div>string</div>\n<div>\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n>\n  string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n>\n  string\n</div>\n<div attribute=\"value\">\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\" />\n  <source src=\"movie.ogg\" type=\"video/ogg\" />\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div>\n  <div>string</div>\n  <div>string</div>\n</div>\n<div>\n  <div><div>string</div></div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n  <div><div>string</div></div>\n</div>\n<div><div></div></div>\n<div>\n  <div></div>\n  <div></div>\n</div>\n<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div><div>string</div></div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n\n  <div>string</div>\n</div>\n<ul>\n  123\n  <li class=\"foo\" id=\"bar\">First</li>\n  456\n  <li class=\"baz\">Second</li>\n  789\n</ul>\n<span>\n  *\n  <b>200</b>\n</span>\n<img\n  src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n/>\n123\n<div>\n  123\n  <meta attr />\n  456\n</div>\n<p>\n  x\n  <span a=\"b\"></span>\n</p>\n<p>\n  x\n  <meta a />\n</p>\n<p>\n  x\n  <meta />\n</p>\n<span></span>\n\n<label\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n></label>\n|\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br />\n<br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">\n  Disabled Cancel\n</button>\n<br />\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br />\n<br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">\n  Disabled Cancel\n</button>\n<br />\n<br />\n<p>\n  \"\n  <span [innerHTML]=\"title\"></span>\n  \" is the\n  <i>property bound</i>\n  title.\n</p>\n<li>\n  12345678901234567890123456789012345678901234567890123456789012345678901234567890\n</li>\n<div>\n  <app-nav></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-nav [input]=\"something\"></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-primary-navigation></app-primary-navigation>\n  <router-outlet></router-outlet>\n  <app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root>\n  <span>tag name in other namespace should also lower cased</span>\n</x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit, \"\n  <strong>seddoeiusmod</strong>\n  \".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>\n  .\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr></tr></table>\n<div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\" /></span>\n<span>\n  <span><input type=\"checkbox\" /></span>\n</span>\n<span><input type=\"checkbox\" /></span>");
    Ok(())
}
#[test]
fn test_tags_html_html_whitespace_sensitivitystrict_format_1_3dd5858b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<br/>\n<br />\n<br  />\n<br\n/>\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br\n  attribute-a=\"value\"\n/>\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\">\n  <source src=\"movie.ogg\" type=\"video/ogg\">\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div><div><div>string</div></div><div>string</div></div>\n<div><div>string</div><div><div>string</div></div></div>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div><div><div><div><div><div><div>string</div></div></div></div></div></div></div>\n<div>\n  <div>string</div>\n</div>\n<div>\n\n  <div>string</div>\n\n</div>\n<div>\n\n  <div>string</div>\n\n  <div>string</div>\n\n</div>\n<ul\n  >123<li\n    class=\"foo\"\n    id=\"bar\"\n  >First</li\n  >456<li\n    class=\"baz\"\n  >Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr/>456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a></p>\n<p>x<meta></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n<app-nav></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-nav [input]=\"something\"></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-primary-navigation></app-primary-navigation>\n<router-outlet></router-outlet>\n<app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><SPAN>tag name in other namespace should also lower cased</SPAN></x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr>\n</tr>\n</table><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\"/> </span>\n<span><span><input type=\"checkbox\"/></span></span>\n<span><input type=\"checkbox\"/></span>") ? ;
    assert_eq ! (formatted , "<br />\n<br />\n<br />\n<br />\n<br attribute-a />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n/>\n<br attribute-a=\"value\" />\n<br attribute-a=\"value\" />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n/>\n<br\n  attribute-a=\"value\"\n  attribute-b=\"value\"\n  attribute-c=\"value\"\n  attribute-d=\"value\"\n  attribute-e=\"value\"\n  attribute-f=\"value\"\n/>\n<div>string</div>\n<div\n  >very very very very very very very very very very very very very very very\n  very long string</div\n>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n  >string</div\n>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n  >string</div\n>\n<div\n  attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n  >string</div\n>\n<div attribute=\"value\"\n  >very very very very very very very very very very very very very very very\n  very long string</div\n>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n  >string</div\n>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n  >very very very very very very very very very very very very very very very\n  very long string</div\n>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\" />\n  <source src=\"movie.ogg\" type=\"video/ogg\" />\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div\n  ><div><div>string</div></div\n  ><div>string</div></div\n>\n<div\n  ><div>string</div><div><div>string</div></div></div\n>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div\n  ><div\n    ><div\n      ><div\n        ><div\n          ><div><div>string</div></div></div\n        ></div\n      ></div\n    ></div\n  ></div\n>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n\n  <div>string</div>\n</div>\n<ul\n  >123<li class=\"foo\" id=\"bar\">First</li\n  >456<li class=\"baz\">Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img\n  src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n/>123\n<div>123<meta attr />456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a /></p>\n<p>x<meta /></p>\n<span></span>\n\n<label\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n></label>\n|\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li\n  >12345678901234567890123456789012345678901234567890123456789012345678901234567890</li\n>\n<div>\n  <app-nav></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-nav [input]=\"something\"></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-primary-navigation></app-primary-navigation>\n  <router-outlet></router-outlet>\n  <app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root\n  ><span>tag name in other namespace should also lower cased</span></x:root\n>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr> </tr> </table\n><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\" /> </span>\n<span\n  ><span><input type=\"checkbox\" /></span\n></span>\n<span><input type=\"checkbox\" /></span>");
    Ok(())
}
#[test]
fn test_tags_html_print_width_infinity_format_1_3dd5858b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<br/>\n<br />\n<br  />\n<br\n/>\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br\n  attribute-a=\"value\"\n/>\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\">\n  <source src=\"movie.ogg\" type=\"video/ogg\">\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div><div><div>string</div></div><div>string</div></div>\n<div><div>string</div><div><div>string</div></div></div>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div><div><div><div><div><div><div>string</div></div></div></div></div></div></div>\n<div>\n  <div>string</div>\n</div>\n<div>\n\n  <div>string</div>\n\n</div>\n<div>\n\n  <div>string</div>\n\n  <div>string</div>\n\n</div>\n<ul\n  >123<li\n    class=\"foo\"\n    id=\"bar\"\n  >First</li\n  >456<li\n    class=\"baz\"\n  >Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr/>456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a></p>\n<p>x<meta></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n<app-nav></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-nav [input]=\"something\"></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-primary-navigation></app-primary-navigation>\n<router-outlet></router-outlet>\n<app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><SPAN>tag name in other namespace should also lower cased</SPAN></x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr>\n</tr>\n</table><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\"/> </span>\n<span><span><input type=\"checkbox\"/></span></span>\n<span><input type=\"checkbox\"/></span>") ? ;
    assert_eq ! (formatted , "<br />\n<br />\n<br />\n<br />\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br attribute-a=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\" />\n  <source src=\"movie.ogg\" type=\"video/ogg\" />\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div>\n  <div>string</div>\n  <div>string</div>\n</div>\n<div>\n  <div><div>string</div></div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n  <div><div>string</div></div>\n</div>\n<div><div></div></div>\n<div>\n  <div></div>\n  <div></div>\n</div>\n<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div><div>string</div></div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n\n  <div>string</div>\n</div>\n<ul>\n  123\n  <li class=\"foo\" id=\"bar\">First</li>\n  456\n  <li class=\"baz\">Second</li>\n  789\n</ul>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr />456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a /></p>\n<p>x<meta /></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">Disabled Cancel</button>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">Disabled Cancel</button>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n  <app-nav></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-nav [input]=\"something\"></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-primary-navigation></app-primary-navigation>\n  <router-outlet></router-outlet>\n  <app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><span>tag name in other namespace should also lower cased</span></x:root>\n<div>Lorem ipsum dolor sit amet, consectetur adipiscing elit, \"<strong>seddoeiusmod</strong>\".</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table>\n  <tr></tr>\n</table>\n<div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\" /> </span>\n<span\n  ><span><input type=\"checkbox\" /></span\n></span>\n<span><input type=\"checkbox\" /></span>");
    Ok(())
}
#[test]
fn test_tags_html_print_width_1_format_1_3dd5858b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<br/>\n<br />\n<br  />\n<br\n/>\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br\n  attribute-a=\"value\"\n/>\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\">\n  <source src=\"movie.ogg\" type=\"video/ogg\">\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div><div><div>string</div></div><div>string</div></div>\n<div><div>string</div><div><div>string</div></div></div>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div><div><div><div><div><div><div>string</div></div></div></div></div></div></div>\n<div>\n  <div>string</div>\n</div>\n<div>\n\n  <div>string</div>\n\n</div>\n<div>\n\n  <div>string</div>\n\n  <div>string</div>\n\n</div>\n<ul\n  >123<li\n    class=\"foo\"\n    id=\"bar\"\n  >First</li\n  >456<li\n    class=\"baz\"\n  >Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr/>456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a></p>\n<p>x<meta></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n<app-nav></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-nav [input]=\"something\"></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-primary-navigation></app-primary-navigation>\n<router-outlet></router-outlet>\n<app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><SPAN>tag name in other namespace should also lower cased</SPAN></x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr>\n</tr>\n</table><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\"/> </span>\n<span><span><input type=\"checkbox\"/></span></span>\n<span><input type=\"checkbox\"/></span>") ? ;
    assert_eq ! (formatted , "<br />\n<br />\n<br />\n<br />\n<br\n  attribute-a\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n/>\n<br\n  attribute-a=\"value\"\n/>\n<br\n  attribute-a=\"value\"\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n/>\n<br\n  attribute-a=\"value\"\n  attribute-b=\"value\"\n  attribute-c=\"value\"\n  attribute-d=\"value\"\n  attribute-e=\"value\"\n  attribute-f=\"value\"\n/>\n<div>\n  string\n</div>\n<div>\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  long\n  string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n>\n  string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n>\n  string\n</div>\n<div\n  attribute=\"value\"\n>\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  long\n  string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  very\n  long\n  string\n</div>\n<video\n  width=\"320\"\n  height=\"240\"\n  controls\n>\n  <source\n    src=\"movie.mp4\"\n    type=\"video/mp4\"\n  />\n  <source\n    src=\"movie.ogg\"\n    type=\"video/ogg\"\n  />\n  Your\n  browser\n  does\n  not\n  support\n  the\n  video\n  tag.\n</video>\n<div>\n  <div>\n    string\n  </div>\n</div>\n<div>\n  <div>\n    string\n  </div>\n  <div>\n    string\n  </div>\n</div>\n<div>\n  <div>\n    <div>\n      string\n    </div>\n  </div>\n  <div>\n    string\n  </div>\n</div>\n<div>\n  <div>\n    string\n  </div>\n  <div>\n    <div>\n      string\n    </div>\n  </div>\n</div>\n<div>\n  <div></div>\n</div>\n<div>\n  <div></div>\n  <div></div>\n</div>\n<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              string\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<div>\n  <div>\n    string\n  </div>\n</div>\n<div>\n  <div>\n    string\n  </div>\n</div>\n<div>\n  <div>\n    string\n  </div>\n\n  <div>\n    string\n  </div>\n</div>\n<ul>\n  123\n  <li\n    class=\"foo\"\n    id=\"bar\"\n  >\n    First\n  </li>\n  456\n  <li\n    class=\"baz\"\n  >\n    Second\n  </li>\n  789\n</ul>\n<span\n  >*<b\n    >200</b\n  ></span\n>\n<img\n  src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n/>123\n<div>\n  123<meta\n    attr\n  />456\n</div>\n<p>\n  x<span\n    a=\"b\"\n  ></span>\n</p>\n<p>\n  x<meta\n    a\n  />\n</p>\n<p>\n  x<meta />\n</p>\n<span></span>\n\n<label\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n></label>\n|\n<span></span>\n<br />\n<button\n  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n>\n  12345678901234567890\n</button>\n<br /><br />\n\n<button\n  bind-disabled=\"isUnchanged\"\n  on-click=\"onSave($event)\"\n>\n  Disabled\n  Cancel\n</button>\n<br /><br />\n<button\n  xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n>\n  12345678901234567890\n</button>\n<br /><br />\n\n<button\n  bind-disabled=\"isUnchanged\"\n  on-click=\"onSave($event)\"\n>\n  Disabled\n  Cancel\n</button>\n<br /><br />\n<p>\n  \"<span\n    [innerHTML]=\"title\"\n  ></span\n  >\"\n  is\n  the\n  <i\n    >property\n    bound</i\n  >\n  title.\n</p>\n<li>\n  12345678901234567890123456789012345678901234567890123456789012345678901234567890\n</li>\n<div>\n  <app-nav></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-nav\n    [input]=\"something\"\n  ></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-primary-navigation></app-primary-navigation>\n  <router-outlet></router-outlet>\n  <app-footer\n    [input]=\"something\"\n  ></app-footer>\n</div>\n<x:root\n  ><span\n    >tag\n    name\n    in\n    other\n    namespace\n    should\n    also\n    lower\n    cased</span\n  ></x:root\n>\n<div>\n  Lorem\n  ipsum\n  dolor\n  sit\n  amet,\n  consectetur\n  adipiscing\n  elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem\n  ipsum\n  dolor\n  sit\n  amet,\n  consectetur\n  adipiscing\n  elit,\n  <strong\n    >seddoeiusmod</strong\n  >.\n</div>\n<span>\n  <i\n    class=\"fa fa-refresh fa-spin\"\n  />\n  <i\n    class=\"fa fa-refresh fa-spin\"\n  />\n  <i\n    class=\"fa fa-refresh fa-spin\"\n  />\n</span>\n\n<!-- #5810 -->\n<table>\n  <tr></tr>\n</table>\n<div>\n  Should\n  not\n  insert\n  empty\n  line\n  before\n  this\n  div\n</div>\n\n<!-- self-closing -->\n<span\n  ><input\n    type=\"checkbox\"\n  />\n</span>\n<span\n  ><span\n    ><input\n      type=\"checkbox\" /></span\n></span>\n<span\n  ><input\n    type=\"checkbox\"\n/></span>");
    Ok(())
}
#[test]
fn test_tags_html_format_1_3dd5858b() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<br/>\n<br />\n<br  />\n<br\n/>\n<br attribute-a />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute />\n<br attribute-a=\"value\" />\n<br\n  attribute-a=\"value\"\n/>\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\" />\n<br very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\" />\n<br attribute-a=\"value\" attribute-b=\"value\" attribute-c=\"value\" attribute-d=\"value\" attribute-e=\"value\" attribute-f=\"value\" />\n<div>string</div>\n<div>very very very very very very very very very very very very very very very very long string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute>string</div>\n<div very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\">string</div>\n<div attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\">string</div>\n<div attribute=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">string</div>\n<div attribute=\"value\" attributea=\"value\" attributeb=\"value\" attributec=\"value\" attributed=\"value\" attributef=\"value\">very very very very very very very very very very very very very very very very long string</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\">\n  <source src=\"movie.ogg\" type=\"video/ogg\">\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div><div>string</div><div>string</div></div>\n<div><div><div>string</div></div><div>string</div></div>\n<div><div>string</div><div><div>string</div></div></div>\n<div><div></div></div>\n<div><div></div><div></div></div>\n<div><div><div><div><div><div><div>string</div></div></div></div></div></div></div>\n<div>\n  <div>string</div>\n</div>\n<div>\n\n  <div>string</div>\n\n</div>\n<div>\n\n  <div>string</div>\n\n  <div>string</div>\n\n</div>\n<ul\n  >123<li\n    class=\"foo\"\n    id=\"bar\"\n  >First</li\n  >456<li\n    class=\"baz\"\n  >Second</li\n  >789</ul\n>\n<span>*<b>200</b></span>\n<img src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\" />123\n<div>123<meta attr/>456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a></p>\n<p>x<meta></p>\n<span></span>\n\n<label aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa></label> |\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\n  >12345678901234567890</button\n> <br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\"\n  >Disabled Cancel</button\n>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>12345678901234567890123456789012345678901234567890123456789012345678901234567890</li>\n<div>\n<app-nav></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-nav [input]=\"something\"></app-nav>\n<router-outlet></router-outlet>\n<app-footer></app-footer>\n\n<app-primary-navigation></app-primary-navigation>\n<router-outlet></router-outlet>\n<app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root><SPAN>tag name in other namespace should also lower cased</SPAN></x:root>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table><tr>\n</tr>\n</table><div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\"/> </span>\n<span><span><input type=\"checkbox\"/></span></span>\n<span><input type=\"checkbox\"/></span>") ? ;
    assert_eq ! (formatted , "<br />\n<br />\n<br />\n<br />\n<br attribute-a />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n/>\n<br attribute-a=\"value\" />\n<br attribute-a=\"value\" />\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n/>\n<br\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n/>\n<br\n  attribute-a=\"value\"\n  attribute-b=\"value\"\n  attribute-c=\"value\"\n  attribute-d=\"value\"\n  attribute-e=\"value\"\n  attribute-f=\"value\"\n/>\n<div>string</div>\n<div>\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute\n>\n  string\n</div>\n<div\n  very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-attribute=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-very-long-value\"\n>\n  string\n</div>\n<div attribute=\"value\">\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  string\n</div>\n<div\n  attribute=\"value\"\n  attributea=\"value\"\n  attributeb=\"value\"\n  attributec=\"value\"\n  attributed=\"value\"\n  attributef=\"value\"\n>\n  very very very very very very very very very very very very very very very\n  very long string\n</div>\n<video width=\"320\" height=\"240\" controls>\n  <source src=\"movie.mp4\" type=\"video/mp4\" />\n  <source src=\"movie.ogg\" type=\"video/ogg\" />\n  Your browser does not support the video tag.\n</video>\n<div><div>string</div></div>\n<div>\n  <div>string</div>\n  <div>string</div>\n</div>\n<div>\n  <div><div>string</div></div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n  <div><div>string</div></div>\n</div>\n<div><div></div></div>\n<div>\n  <div></div>\n  <div></div>\n</div>\n<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div><div>string</div></div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n</div>\n<div>\n  <div>string</div>\n\n  <div>string</div>\n</div>\n<ul>\n  123\n  <li class=\"foo\" id=\"bar\">First</li>\n  456\n  <li class=\"baz\">Second</li>\n  789\n</ul>\n<span>*<b>200</b></span>\n<img\n  src=\"longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong\"\n/>123\n<div>123<meta attr />456</div>\n<p>x<span a=\"b\"></span></p>\n<p>x<meta a /></p>\n<p>x<meta /></p>\n<span></span>\n\n<label\n  aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\n></label>\n|\n<span></span>\n<br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">\n  Disabled Cancel\n</button>\n<br /><br />\n<button xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>12345678901234567890</button>\n<br /><br />\n\n<button bind-disabled=\"isUnchanged\" on-click=\"onSave($event)\">\n  Disabled Cancel\n</button>\n<br /><br />\n<p>\"<span [innerHTML]=\"title\"></span>\" is the <i>property bound</i> title.</p>\n<li>\n  12345678901234567890123456789012345678901234567890123456789012345678901234567890\n</li>\n<div>\n  <app-nav></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-nav [input]=\"something\"></app-nav>\n  <router-outlet></router-outlet>\n  <app-footer></app-footer>\n\n  <app-primary-navigation></app-primary-navigation>\n  <router-outlet></router-outlet>\n  <app-footer [input]=\"something\"></app-footer>\n</div>\n<x:root\n  ><span>tag name in other namespace should also lower cased</span></x:root\n>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  \"<strong>seddoeiusmod</strong>\".\n</div>\n<div>\n  Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n  <strong>seddoeiusmod</strong>.\n</div>\n<span>\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n  <i class=\"fa fa-refresh fa-spin\" />\n</span>\n\n<!-- #5810 -->\n<table>\n  <tr></tr>\n</table>\n<div>Should not insert empty line before this div</div>\n\n<!-- self-closing -->\n<span><input type=\"checkbox\" /> </span>\n<span\n  ><span><input type=\"checkbox\" /></span\n></span>\n<span><input type=\"checkbox\" /></span>");
    Ok(())
}
#[test]
fn test_tags_2_html_html_whitespace_sensitivityignore_format_1_a136d9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>before<details><summary>summary long long long long </summary>details</details>after</div>\n\n<div>before<dialog open>dialog long long long long  long long long long </dialog>after</div>\n\n<div>before<object data=\"horse.wav\"><param name=\"autoplay\" value=\"true\"/><param name=\"autoplay\" value=\"true\"/></object>after</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>") ? ;
    assert_eq ! (formatted , "<div>\n  before\n  <noscript>noscript long long long long long long long long</noscript>\n  after\n</div>\n\n<div>\n  before\n  <details>\n    <summary>summary long long long long</summary>\n    details\n  </details>\n  after\n</div>\n\n<div>\n  before\n  <dialog open>dialog long long long long long long long long</dialog>\n  after\n</div>\n\n<div>\n  before\n  <object data=\"horse.wav\">\n    <param name=\"autoplay\" value=\"true\" />\n    <param name=\"autoplay\" value=\"true\" />\n  </object>\n  after\n</div>\n\n<div>\n  before\n  <meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>\n  after\n</div>\n\n<div>\n  before\n  <progress value=\".5\" max=\"1\"></progress>\n  after\n</div>");
    Ok(())
}
#[test]
fn test_tags_2_html_html_whitespace_sensitivitystrict_format_1_a136d9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>before<details><summary>summary long long long long </summary>details</details>after</div>\n\n<div>before<dialog open>dialog long long long long  long long long long </dialog>after</div>\n\n<div>before<object data=\"horse.wav\"><param name=\"autoplay\" value=\"true\"/><param name=\"autoplay\" value=\"true\"/></object>after</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>") ? ;
    assert_eq ! (formatted , "<div\n  >before<noscript>noscript long long long long long long long long</noscript\n  >after</div\n>\n\n<div\n  >before<details\n    ><summary>summary long long long long </summary>details</details\n  >after</div\n>\n\n<div\n  >before<dialog open>dialog long long long long long long long long </dialog\n  >after</div\n>\n\n<div\n  >before<object data=\"horse.wav\"\n    ><param name=\"autoplay\" value=\"true\" /><param\n      name=\"autoplay\"\n      value=\"true\" /></object\n  >after</div\n>\n\n<div\n  >before<meter\n    min=\"0\"\n    max=\"1\"\n    low=\".4\"\n    high=\".7\"\n    optimum=\".5\"\n    value=\".2\"\n  ></meter\n  >after</div\n>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>");
    Ok(())
}
#[test]
fn test_tags_2_html_print_width_infinity_format_1_a136d9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>before<details><summary>summary long long long long </summary>details</details>after</div>\n\n<div>before<dialog open>dialog long long long long  long long long long </dialog>after</div>\n\n<div>before<object data=\"horse.wav\"><param name=\"autoplay\" value=\"true\"/><param name=\"autoplay\" value=\"true\"/></object>after</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>") ? ;
    assert_eq ! (formatted , "<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>\n  before\n  <details>\n    <summary>summary long long long long</summary>\n    details\n  </details>\n  after\n</div>\n\n<div>\n  before\n  <dialog open>dialog long long long long long long long long</dialog>\n  after\n</div>\n\n<div>\n  before<object data=\"horse.wav\">\n    <param name=\"autoplay\" value=\"true\" />\n    <param name=\"autoplay\" value=\"true\" /></object\n  >after\n</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>");
    Ok(())
}
#[test]
fn test_tags_2_html_print_width_1_format_1_a136d9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>before<details><summary>summary long long long long </summary>details</details>after</div>\n\n<div>before<dialog open>dialog long long long long  long long long long </dialog>after</div>\n\n<div>before<object data=\"horse.wav\"><param name=\"autoplay\" value=\"true\"/><param name=\"autoplay\" value=\"true\"/></object>after</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>") ? ;
    assert_eq ! (formatted , "<div>\n  before<noscript\n    >noscript\n    long\n    long\n    long\n    long\n    long\n    long\n    long\n    long</noscript\n  >after\n</div>\n\n<div>\n  before\n  <details>\n    <summary>\n      summary\n      long\n      long\n      long\n      long\n    </summary>\n    details\n  </details>\n  after\n</div>\n\n<div>\n  before\n  <dialog\n    open\n  >\n    dialog\n    long\n    long\n    long\n    long\n    long\n    long\n    long\n    long\n  </dialog>\n  after\n</div>\n\n<div>\n  before<object\n    data=\"horse.wav\"\n  >\n    <param\n      name=\"autoplay\"\n      value=\"true\"\n    />\n    <param\n      name=\"autoplay\"\n      value=\"true\"\n    /></object\n  >after\n</div>\n\n<div>\n  before<meter\n    min=\"0\"\n    max=\"1\"\n    low=\".4\"\n    high=\".7\"\n    optimum=\".5\"\n    value=\".2\"\n  ></meter\n  >after\n</div>\n\n<div>\n  before<progress\n    value=\".5\"\n    max=\"1\"\n  ></progress\n  >after\n</div>");
    Ok(())
}
#[test]
fn test_tags_2_html_format_1_a136d9fa() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>before<noscript>noscript long long long long long long long long</noscript>after</div>\n\n<div>before<details><summary>summary long long long long </summary>details</details>after</div>\n\n<div>before<dialog open>dialog long long long long  long long long long </dialog>after</div>\n\n<div>before<object data=\"horse.wav\"><param name=\"autoplay\" value=\"true\"/><param name=\"autoplay\" value=\"true\"/></object>after</div>\n\n<div>before<meter min=\"0\" max=\"1\" low=\".4\" high=\".7\" optimum=\".5\" value=\".2\"></meter>after</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>") ? ;
    assert_eq ! (formatted , "<div>\n  before<noscript>noscript long long long long long long long long</noscript\n  >after\n</div>\n\n<div>\n  before\n  <details>\n    <summary>summary long long long long</summary>\n    details\n  </details>\n  after\n</div>\n\n<div>\n  before\n  <dialog open>dialog long long long long long long long long</dialog>\n  after\n</div>\n\n<div>\n  before<object data=\"horse.wav\">\n    <param name=\"autoplay\" value=\"true\" />\n    <param name=\"autoplay\" value=\"true\" /></object\n  >after\n</div>\n\n<div>\n  before<meter\n    min=\"0\"\n    max=\"1\"\n    low=\".4\"\n    high=\".7\"\n    optimum=\".5\"\n    value=\".2\"\n  ></meter\n  >after\n</div>\n\n<div>before<progress value=\".5\" max=\"1\"></progress>after</div>");
    Ok(())
}
#[test]
fn test_textarea_html_html_whitespace_sensitivityignore_format_1_8f29dac6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>") ? ;
    assert_eq ! (formatted , "<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>");
    Ok(())
}
#[test]
fn test_textarea_html_html_whitespace_sensitivitystrict_format_1_8f29dac6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>") ? ;
    assert_eq ! (formatted , "<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>");
    Ok(())
}
#[test]
fn test_textarea_html_print_width_infinity_format_1_8f29dac6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>") ? ;
    assert_eq ! (formatted , "<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>");
    Ok(())
}
#[test]
fn test_textarea_html_print_width_1_format_1_8f29dac6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>") ? ;
    assert_eq ! (formatted , "<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea\n                          rows=\"10\"\n                          cols=\"45\"\n                          name=\"text\"\n                        >\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div>\n  <textarea>\nlorem ipsum</textarea\n  >\n</div>");
    Ok(())
}
#[test]
fn test_textarea_html_format_1_8f29dac6() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>") ? ;
    assert_eq ! (formatted , "<div>\n  <div>\n    <div>\n      <div>\n        <div>\n          <div>\n            <div>\n              <div>\n                <div>\n                  <div>\n                    <div>\n                      <div>\n                        <textarea rows=\"10\" cols=\"45\" name=\"text\">\n                        String\n                        </textarea>\n                      </div>\n                    </div>\n                  </div>\n                </div>\n              </div>\n            </div>\n          </div>\n        </div>\n      </div>\n    </div>\n  </div>\n</div>\n<textarea></textarea>\n\n<div><textarea>lorem ipsum</textarea></div>");
    Ok(())
}
#[test]
fn test_unsupported_html_html_whitespace_sensitivityignore_format_1_7c2c4310() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("ignore")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<center></center>")?;
    assert_eq!(formatted, "<center></center>");
    Ok(())
}
#[test]
fn test_unsupported_html_html_whitespace_sensitivitystrict_format_1_7c2c4310() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .html_whitespace_sensitivity("strict")
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<center></center>")?;
    assert_eq!(formatted, "<center></center>");
    Ok(())
}
#[test]
fn test_unsupported_html_print_width_infinity_format_1_7c2c4310() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(INFINITY)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<center></center>")?;
    assert_eq!(formatted, "<center></center>");
    Ok(())
}
#[test]
fn test_unsupported_html_print_width_1_format_1_7c2c4310() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(1)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<center></center>")?;
    assert_eq!(formatted, "<center></center>");
    Ok(())
}
#[test]
fn test_unsupported_html_format_1_7c2c4310() -> Result<()> {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format("<center></center>")?;
    assert_eq!(formatted, "<center></center>");
    Ok(())
}
