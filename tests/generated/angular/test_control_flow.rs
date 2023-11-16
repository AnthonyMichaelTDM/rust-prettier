#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[test]
fn test_after_close_tag_html_format_1_5d6c7ee1() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<ion-list class=\"timeline\" appScrollItem>\n  @for (let item of items; index as i; trackBy: trackByFn) {\n    <app-tweet-item></app-tweet-item>@if (\n      areaId &&\n      canTweet\n    ) {\n      <ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n    }\n  }\n</ion-list>\n\n<ng-container *ngIf=\"\n      areaId &&\n      canTweet\n\">\n<ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n</ng-container>\n\n\n@if (\n      areaId &&\n      canTweet\n    ) {\n      <ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n    }") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ion-list class=\"timeline\" appScrollItem>\n  @for (let item of items; index as i; trackBy: trackByFn) {\n    <app-tweet-item></app-tweet-item>\n    @if (areaId && canTweet) {\n      <ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n    }\n  }\n</ion-list>\n\n<ng-container *ngIf=\"areaId && canTweet\">\n  <ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n</ng-container>\n\n@if (areaId && canTweet) {\n  <ion-item class=\"recommend-trust-up\" lines=\"full\"></ion-item>\n}");
}
#[test]
fn test_chid_html_format_1_7485573e() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted =
        pretty_printer.format("<span>@if (test) {text}</span>\n<div>@if (test) {text}</div>");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<span>\n  @if (test) {\n    text\n  }\n</span>\n<div>\n  @if (test) {\n    text\n  }\n</div>");
}
#[test]
fn test_defer_html_format_1_41bedaba() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@defer (when isCheckedDefer()) {\n<app-c1/>\n}\n@placeholder {\n<span>Placeholder</span>\n}\n@error {\n<span>Error</span>\n}\n@loading(minimum 1s) {\n<span>Loading...</span>\n}\n\n\n@defer (on interaction) {\n<span>Clicked</span>\n}\n@placeholder {\n<span>Placeholder (click on it!)</span>\n}\n\n\n@defer (on hover) {\n<span>Hovered</span>\n}\n@placeholder {\n<span>Placeholder (hover it!)</span>\n}\n\n\n@defer (on idle) {\n<span>Browser has reached an idle state</span>\n}\n@placeholder {\n<span>Placeholder</span>\n}\n\n\n@defer (on timer(5s)) {\n<span>Visible after 5s</span>\n}\n@placeholder {\n<span>Placeholder</span>\n}\n\n\n@defer (on viewport) {\n<app-c2 text=\"The block entered the viewport\"/>\n}\n@placeholder {\n<span>Placeholder</span>\n}\n\n\n@defer (on interaction; prefetch on hover) {\n<app-c3/>\n}\n@placeholder {\n<span>Placeholder (hover it, then click on it!)</span>\n}\n\n\n@defer (on interaction; prefetch on hover) {\n<app-c3/>\n}\n@placeholder {\n<span>Placeholder (hover it, then click on it!)</span>\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@defer (when isCheckedDefer()) {\n  <app-c1 />\n} @placeholder {\n  <span>Placeholder</span>\n} @error {\n  <span>Error</span>\n} @loading (minimum 1s) {\n  <span>Loading...</span>\n}\n\n@defer (on interaction) {\n  <span>Clicked</span>\n} @placeholder {\n  <span>Placeholder (click on it!)</span>\n}\n\n@defer (on hover) {\n  <span>Hovered</span>\n} @placeholder {\n  <span>Placeholder (hover it!)</span>\n}\n\n@defer (on idle) {\n  <span>Browser has reached an idle state</span>\n} @placeholder {\n  <span>Placeholder</span>\n}\n\n@defer (on timer(5s)) {\n  <span>Visible after 5s</span>\n} @placeholder {\n  <span>Placeholder</span>\n}\n\n@defer (on viewport) {\n  <app-c2 text=\"The block entered the viewport\" />\n} @placeholder {\n  <span>Placeholder</span>\n}\n\n@defer (on interaction; prefetch on hover) {\n  <app-c3 />\n} @placeholder {\n  <span>Placeholder (hover it, then click on it!)</span>\n}\n\n@defer (on interaction; prefetch on hover) {\n  <app-c3 />\n} @placeholder {\n  <span>Placeholder (hover it, then click on it!)</span>\n}");
}
#[test]
fn test_element_tags_html_format_1_5abe714a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("@if (true) {<img />}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@if (true) {\n  <img />\n}");
}
#[test]
fn test_for_html_format_1_9bc1cdc8() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<ul>\n@for (\n            let\n\n            item of items;index as\ni;\n            trackBy: trackByFn\n) {\n<li><strong>{{item.name}}</strong></li>\n}\n@empty {\n<span>The collection is empty</span>\n}\n</ul>\n\n\n<li *ngFor=\"\n            let\n\n            item of items;index as\ni;\n            trackBy: trackByFn\n\"></li>\n\n\n\n<ul>\n@for (item of collection; track item.id; let index = $index, first = $first; let last = $last, even = $even, odd = $odd; let count = $count) {\n\n<li><strong>{{item.name}}</strong> index={{index}} first={{first}} last={{last}} even={{even}} odd={{odd}} count={{count}}</li>\n\n}\n</ul>\n\n<ul>\n@for (item of\n emptyCollection; track item.id;) {\n<li><strong>{{item.name}}</strong></li>\n}\n@empty {\n  <span>The collection is empty</span>\n}\n</ul>\n<li *ngFor=\"\nitem of\n emptyCollection; track item.id;\n\"></li>\n\n<div>\n  @for ( item of items; track item){\n  }\n\n  <div *ngFor=\"item of items; track item\"></div>\n</div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<ul>\n  @for (let item of items; index as i; trackBy: trackByFn) {\n    <li>\n      <strong>{{ item.name }}</strong>\n    </li>\n  } @empty {\n    <span>The collection is empty</span>\n  }\n</ul>\n\n<li *ngFor=\"let item of items; index as i; trackBy: trackByFn\"></li>\n\n<ul>\n  @for (\n    item of collection;\n    track item.id;\n    let index = $index, first = $first;\n    let last = $last, even = $even, odd = $odd;\n    let count = $count\n  ) {\n    <li>\n      <strong>{{ item.name }}</strong> index={{ index }} first={{\n        first\n      }}\n      last={{ last }} even={{ even }} odd={{ odd }} count={{ count }}\n    </li>\n  }\n</ul>\n\n<ul>\n  @for (item of emptyCollection; track item.id) {\n    <li>\n      <strong>{{ item.name }}</strong>\n    </li>\n  } @empty {\n    <span>The collection is empty</span>\n  }\n</ul>\n<li *ngFor=\"item of emptyCollection; track item.id\"></li>\n\n<div>\n  @for (item of items; track item) {}\n\n  <div *ngFor=\"item of items; track item\"></div>\n</div>");
}
#[test]
fn test_if_html_format_1_0c8bf85f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<h3>&#64;if and &#64;else</h3>\n<div>\n<input #checkbox type=\"checkbox\" [checked]=\"isChecked()\" (change)=\"isChecked.set(checkbox.checked)\" id=\"checkbox\"/>\n</div>\n<div>\n@if (isChecked()) {\n<span>Checked</span>\n}\n@else {\n<span>Not checked</span>\n}\n</div>\n\n@if (users$   |   async;   as   users) { {{   users.length   }} }\n@else if (users$   |   async;   as   users) { {{   users.length   }} }\n\n<div *ngIf=\"users$   |   async;   as   users\">{{   users.length   }}</div>\n\n@else {}\n\n\n@if (isDev) {}\n@else             if (test) {}\n\n@if      (foo(     \"quotes\")) {}\n@else if (foo(     'quotes')) {}\n\n@if ( widthCategory !== undefined && widthCategory >\ntopStartToSideStartMaxSize) {\n<app-component />\n}\n\n@if ( widthCategory; as item) {\n}\n\n<div *ngIf=\"widthCategory; as item\"></div>") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<h3>&#64;if and &#64;else</h3>\n<div>\n  <input\n    #checkbox\n    type=\"checkbox\"\n    [checked]=\"isChecked()\"\n    (change)=\"isChecked.set(checkbox.checked)\"\n    id=\"checkbox\"\n  />\n</div>\n<div>\n  @if (isChecked()) {\n    <span>Checked</span>\n  } @else {\n    <span>Not checked</span>\n  }\n</div>\n\n@if (users$ | async; as users) {\n  {{ users.length }}\n} @else if (users$ | async; as users) {\n  {{ users.length }}\n}\n\n<div *ngIf=\"users$ | async; as users\">{{ users.length }}</div>\n\n@else {}\n\n@if (isDev) {\n} @else if (test) {}\n\n@if (foo(\"quotes\")) {\n} @else if (foo(\"quotes\")) {}\n\n@if (\n  widthCategory !== undefined && widthCategory > topStartToSideStartMaxSize\n) {\n  <app-component />\n}\n\n@if (widthCategory; as item) {}\n\n<div *ngIf=\"widthCategory; as item\"></div>");
}
#[test]
fn test_mix_html_format_1_8ba93dc5() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@if (user.isHuman) {\n<human-profile [data]=\"user\" />\n} @else if (user.isRobot) {\n<!-- robot users are rare, so load their profiles lazily -->\n@defer {\n<robot-profile [data]=\"user\" />\n}\n} @else {\n<p>The profile is unknown!</p>\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@if (user.isHuman) {\n  <human-profile [data]=\"user\" />\n} @else if (user.isRobot) {\n  <!-- robot users are rare, so load their profiles lazily -->\n  @defer {\n    <robot-profile [data]=\"user\" />\n  }\n} @else {\n  <p>The profile is unknown!</p>\n}");
}
#[test]
fn test_switch_html_format_1_bda03d4f() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("@switch (radioValue()) {\n@case (1) {\n<span>Case 1</span>\n}\n@case (2) {\n<span>Case 2</span>\n}\n@default {\n<span>Default case (Not 1 or 2)</span>\n}\n}\n\n\n@switch(\n  should\n.be.formatted\n) {\n  @case(\n  should\n.be.formatted) {}\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "@switch (radioValue()) {\n  @case (1) {\n    <span>Case 1</span>\n  }\n  @case (2) {\n    <span>Case 2</span>\n  }\n  @default {\n    <span>Default case (Not 1 or 2)</span>\n  }\n}\n\n@switch (should.be.formatted) {\n  @case (should.be.formatted) {}\n}");
}
#[test]
fn test_unclosed_html_format_1_fb198443() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("@empty{}\n\n@else {}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@empty {}\n\n@else {}");
}
#[test]
fn test_unknown_block_html_format_1_b3bdde1a() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer.format("@unknown (foo) {\n} @unknown (foo) {\n}");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "@unknown (foo) {}\n@unknown (foo) {}");
}
#[test]
fn test_with_comment_html_format_1_39506734() {
    let pretty_printer = PrettyPrinterBuilder::default().build().unwrap();
    let formatted = pretty_printer . format ("<div>\n  <!-- comment -->\n  <span>Not checked</span>\n</div>\n\n\n<!-- comment -->\n@if (isChecked()) {\n<span>\n  Checked\n  <!-- comment -->\n</span>\n} <!-- comment -->\n@else {\n<span>Not checked</span>\n}\n<!-- comment -->") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "<div>\n  <!-- comment -->\n  <span>Not checked</span>\n</div>\n\n<!-- comment -->\n@if (isChecked()) {\n  <span>\n    Checked\n    <!-- comment -->\n  </span>\n}\n<!-- comment -->\n@else {\n  <span>Not checked</span>\n}\n<!-- comment -->");
}
