#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_break_ts_format_1_ccf2bb37() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class MyContractSelectionWidget extends React.Component<void,  MyContractSelectionWidgetPropsType, void> implements SomethingLarge {\n  method() {}\n}\n\nclass DisplayObject1\n  extends utils.EventEmitter\n  implements interaction_InteractiveTarget {\n}\n\nclass DisplayObject2 extends utils.EventEmitter\n  implements interaction_InteractiveTarget {\n}\n\nclass DisplayObject3 extends utils.EventEmitter\n  implements interaction_InteractiveTarget,\n    somethingElse_SomeOtherThing,\n    somethingElseAgain_RunningOutOfNames {\n}\n\nclass DisplayObject4 extends utils.EventEmitter implements interaction_InteractiveTarget {}\nclass Readable extends events.EventEmitter implements NodeJS_ReadableStream {}\nclass InMemoryAppender extends log4javascript.Appender implements ICachedLogMessageProvider {}\n\nclass Foo extends Immutable.Record({\n  ipaddress: '',\n}) {\n  ipaddress: string;\n}\n\nexport class VisTimelineComponent\n\timplements AfterViewInit, OnChanges, OnDestroy {\n}\nexport class VisTimelineComponent2\n\timplements AfterViewInit, OnChanges, OnDestroy, AndSomethingReallyReallyLong {\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class MyContractSelectionWidget\n  extends React.Component<void, MyContractSelectionWidgetPropsType, void>\n  implements SomethingLarge\n{\n  method() {}\n}\n\nclass DisplayObject1\n  extends utils.EventEmitter\n  implements interaction_InteractiveTarget {}\n\nclass DisplayObject2\n  extends utils.EventEmitter\n  implements interaction_InteractiveTarget {}\n\nclass DisplayObject3\n  extends utils.EventEmitter\n  implements\n    interaction_InteractiveTarget,\n    somethingElse_SomeOtherThing,\n    somethingElseAgain_RunningOutOfNames {}\n\nclass DisplayObject4\n  extends utils.EventEmitter\n  implements interaction_InteractiveTarget {}\nclass Readable extends events.EventEmitter implements NodeJS_ReadableStream {}\nclass InMemoryAppender\n  extends log4javascript.Appender\n  implements ICachedLogMessageProvider {}\n\nclass Foo extends Immutable.Record({\n  ipaddress: \"\",\n}) {\n  ipaddress: string;\n}\n\nexport class VisTimelineComponent\n  implements AfterViewInit, OnChanges, OnDestroy {}\nexport class VisTimelineComponent2\n  implements\n    AfterViewInit,\n    OnChanges,\n    OnDestroy,\n    AndSomethingReallyReallyLong {}");
}
#[test]
fn test_break_heritage_ts_format_1_9dbcf9e4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("class loooooooooooooooooooong extends looooooooooooooooooong implements loooooooooooooooooooong {\n  // leading comment\n  property: string;\n}\n\nclass loooooooooooooooooooong extends looooooooooooooooooong implements loooooooooooooooooooong {\n  property: string;\n}\n\nclass loooooooooooooooooooong extends looooooooooooooooooong implements loooooooooooooooooooong {\n\n  property: string;\n}\n\nclass loooooooooooooooooooong extends looooooooooooooooooong implements loooooooooooooooooooong, loooooooooooooooooooong, loooooooooooooooooooong {\n  property: string;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "class loooooooooooooooooooong\n  extends looooooooooooooooooong\n  implements loooooooooooooooooooong\n{\n  // leading comment\n  property: string;\n}\n\nclass loooooooooooooooooooong\n  extends looooooooooooooooooong\n  implements loooooooooooooooooooong\n{\n  property: string;\n}\n\nclass loooooooooooooooooooong\n  extends looooooooooooooooooong\n  implements loooooooooooooooooooong\n{\n  property: string;\n}\n\nclass loooooooooooooooooooong\n  extends looooooooooooooooooong\n  implements\n    loooooooooooooooooooong,\n    loooooooooooooooooooong,\n    loooooooooooooooooooong\n{\n  property: string;\n}");
}
