use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_template_html_single_quotetrue_format_1_91a30dc4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .single_quote(true)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script id=\"entry-template\" type=\"text/x-handlebars-template\">\n<div class=\"entry\">\n<h1>{{title}}</h1>\n<div class=\"body\">{{body}}</div></div>\n</script>\n\n<script type=\"text/x-handlebars-template\">\n  {{component arg1='hey' arg2=(helper this.arg7 this.arg4) arg3=anotherone arg6=this.arg8}}\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script id=\"entry-template\" type=\"text/x-handlebars-template\">\n  <div class='entry'>\n    <h1>{{title}}</h1>\n    <div class='body'>{{body}}</div></div>\n</script>\n\n<script type=\"text/x-handlebars-template\">\n  {{component\n    arg1='hey'\n    arg2=(helper this.arg7 this.arg4)\n    arg3=anotherone\n    arg6=this.arg8\n  }}\n</script>");
}
#[test]
fn test_template_html_format_1_91a30dc4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<script id=\"entry-template\" type=\"text/x-handlebars-template\">\n<div class=\"entry\">\n<h1>{{title}}</h1>\n<div class=\"body\">{{body}}</div></div>\n</script>\n\n<script type=\"text/x-handlebars-template\">\n  {{component arg1='hey' arg2=(helper this.arg7 this.arg4) arg3=anotherone arg6=this.arg8}}\n</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<script id=\"entry-template\" type=\"text/x-handlebars-template\">\n  <div class=\"entry\">\n    <h1>{{title}}</h1>\n    <div class=\"body\">{{body}}</div></div>\n</script>\n\n<script type=\"text/x-handlebars-template\">\n  {{component\n    arg1=\"hey\"\n    arg2=(helper this.arg7 this.arg4)\n    arg3=anotherone\n    arg6=this.arg8\n  }}\n</script>");
}
