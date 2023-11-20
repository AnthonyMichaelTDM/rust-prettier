#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_angularjs_html_bracket_same_linefalse_format_1_8d69a13c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(false)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\">Warning!</div>\n<span ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\">Warning!</span>\n<img ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\"/>\n<script long-attribute=\"long_long_long_long_long_long_long_long_long_long_long_long_long_value\">alert(1)</script>\n<div (event)=\"  foo == $event  \">Warning!</div>\n<span (event)=\"  foo == $event  \">Warning!</span>\n<img (event)=\"  foo == $event  \"/>\n<script>alert(1)</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\"\n>\n  Warning!\n</div>\n<span\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\"\n  >Warning!</span\n>\n<img\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\"\n/>\n<script\n  long-attribute=\"long_long_long_long_long_long_long_long_long_long_long_long_long_value\"\n>\n  alert(1);\n</script>\n<div (event)=\"(foo == $event)\">Warning!</div>\n<span (event)=\"(foo == $event)\">Warning!</span>\n<img (event)=\"(foo == $event)\" />\n<script>\n  alert(1);\n</script>");
}
#[test]
fn test_angularjs_html_bracket_same_linetrue_format_1_8d69a13c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .bracket_same_line(true)
        .parser("html")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("<div ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\">Warning!</div>\n<span ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\">Warning!</span>\n<img ng-if=\"$ctrl .shouldShowWarning&&!$ctrl.loading\"\n    bindon-target=\" a | b : c \"\n    (event)=\"  foo == $event  \"\n    *ngIf=\" something?true:false    \" [(ngModel)]=\"canSave\"/>\n<script long-attribute=\"long_long_long_long_long_long_long_long_long_long_long_long_long_value\">alert(1)</script>\n<div (event)=\"  foo == $event  \">Warning!</div>\n<span (event)=\"  foo == $event  \">Warning!</span>\n<img (event)=\"  foo == $event  \"/>\n<script>alert(1)</script>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\">\n  Warning!\n</div>\n<span\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\"\n  >Warning!</span\n>\n<img\n  ng-if=\"$ctrl.shouldShowWarning && !$ctrl.loading\"\n  bindon-target=\"a | b: c\"\n  (event)=\"(foo == $event)\"\n  *ngIf=\"something ? true : false\"\n  [(ngModel)]=\"canSave\" />\n<script\n  long-attribute=\"long_long_long_long_long_long_long_long_long_long_long_long_long_value\">\n  alert(1);\n</script>\n<div (event)=\"(foo == $event)\">Warning!</div>\n<span (event)=\"(foo == $event)\">Warning!</span>\n<img (event)=\"(foo == $event)\" />\n<script>\n  alert(1);\n</script>");
}
