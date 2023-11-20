#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_basic_ts_format_1_ddaf7059() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Covariant<out T> = {\n    x: T;\n}\ntype Contravariant<in T> = {\n    f: (x: T) => void;\n}\ntype Invariant<in out T> = {\n    f: (x: T) => T;\n}\ntype T10<out T> = T;\ntype T11<in T> = keyof T;\ntype T12<out T, out K extends keyof T> = T[K];\ntype T13<in out T> = T[keyof T];\n\ntype Covariant1<in T> = {\n    x: T;\n}\n\ntype Contravariant1<out T> = keyof T;\n\ntype Contravariant2<out T> = {\n    f: (x: T) => void;\n}\n\ntype Invariant1<in T> = {\n    f: (x: T) => T;\n}\n\ntype Invariant2<out T> = {\n    f: (x: T) => T;\n}\ntype Foo1<in T> = {\n    x: T;\n    f: FooFn1<T>;\n}\n\ntype Foo2<out T> = {\n    x: T;\n    f: FooFn2<T>;\n}\n\ntype Foo3<in out T> = {\n    x: T;\n    f: FooFn3<T>;\n}\n\ntype T21<in out T> = T;\n\ninterface Baz<out T> {}\ninterface Baz<in T> {}\n\ninterface Parent<out A> {\n    child: Child<A> | null;\n    parent: Parent<A> | null;\n}\n\ndeclare class StateNode<TContext, in out TEvent extends { type: string }> {\n    _storedEvent: TEvent;\n    _action: ActionObject<TEvent>;\n    _state: StateNode<TContext, any>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Covariant<out T> = {\n  x: T;\n};\ntype Contravariant<in T> = {\n  f: (x: T) => void;\n};\ntype Invariant<in out T> = {\n  f: (x: T) => T;\n};\ntype T10<out T> = T;\ntype T11<in T> = keyof T;\ntype T12<out T, out K extends keyof T> = T[K];\ntype T13<in out T> = T[keyof T];\n\ntype Covariant1<in T> = {\n  x: T;\n};\n\ntype Contravariant1<out T> = keyof T;\n\ntype Contravariant2<out T> = {\n  f: (x: T) => void;\n};\n\ntype Invariant1<in T> = {\n  f: (x: T) => T;\n};\n\ntype Invariant2<out T> = {\n  f: (x: T) => T;\n};\ntype Foo1<in T> = {\n  x: T;\n  f: FooFn1<T>;\n};\n\ntype Foo2<out T> = {\n  x: T;\n  f: FooFn2<T>;\n};\n\ntype Foo3<in out T> = {\n  x: T;\n  f: FooFn3<T>;\n};\n\ntype T21<in out T> = T;\n\ninterface Baz<out T> {}\ninterface Baz<in T> {}\n\ninterface Parent<out A> {\n  child: Child<A> | null;\n  parent: Parent<A> | null;\n}\n\ndeclare class StateNode<TContext, in out TEvent extends { type: string }> {\n  _storedEvent: TEvent;\n  _action: ActionObject<TEvent>;\n  _state: StateNode<TContext, any>;\n}");
}
#[test]
fn test_with_jsx_tsx_format_1_ddaf7059() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("tsx")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("type Covariant<out T> = {\n    x: T;\n}\ntype Contravariant<in T> = {\n    f: (x: T) => void;\n}\ntype Invariant<in out T> = {\n    f: (x: T) => T;\n}\ntype T10<out T> = T;\ntype T11<in T> = keyof T;\ntype T12<out T, out K extends keyof T> = T[K];\ntype T13<in out T> = T[keyof T];\n\ntype Covariant1<in T> = {\n    x: T;\n}\n\ntype Contravariant1<out T> = keyof T;\n\ntype Contravariant2<out T> = {\n    f: (x: T) => void;\n}\n\ntype Invariant1<in T> = {\n    f: (x: T) => T;\n}\n\ntype Invariant2<out T> = {\n    f: (x: T) => T;\n}\ntype Foo1<in T> = {\n    x: T;\n    f: FooFn1<T>;\n}\n\ntype Foo2<out T> = {\n    x: T;\n    f: FooFn2<T>;\n}\n\ntype Foo3<in out T> = {\n    x: T;\n    f: FooFn3<T>;\n}\n\ntype T21<in out T> = T;\n\ninterface Baz<out T> {}\ninterface Baz<in T> {}\n\ninterface Parent<out A> {\n    child: Child<A> | null;\n    parent: Parent<A> | null;\n}\n\ndeclare class StateNode<TContext, in out TEvent extends { type: string }> {\n    _storedEvent: TEvent;\n    _action: ActionObject<TEvent>;\n    _state: StateNode<TContext, any>;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "type Covariant<out T> = {\n  x: T;\n};\ntype Contravariant<in T> = {\n  f: (x: T) => void;\n};\ntype Invariant<in out T> = {\n  f: (x: T) => T;\n};\ntype T10<out T> = T;\ntype T11<in T> = keyof T;\ntype T12<out T, out K extends keyof T> = T[K];\ntype T13<in out T> = T[keyof T];\n\ntype Covariant1<in T> = {\n  x: T;\n};\n\ntype Contravariant1<out T> = keyof T;\n\ntype Contravariant2<out T> = {\n  f: (x: T) => void;\n};\n\ntype Invariant1<in T> = {\n  f: (x: T) => T;\n};\n\ntype Invariant2<out T> = {\n  f: (x: T) => T;\n};\ntype Foo1<in T> = {\n  x: T;\n  f: FooFn1<T>;\n};\n\ntype Foo2<out T> = {\n  x: T;\n  f: FooFn2<T>;\n};\n\ntype Foo3<in out T> = {\n  x: T;\n  f: FooFn3<T>;\n};\n\ntype T21<in out T> = T;\n\ninterface Baz<out T> {}\ninterface Baz<in T> {}\n\ninterface Parent<out A> {\n  child: Child<A> | null;\n  parent: Parent<A> | null;\n}\n\ndeclare class StateNode<TContext, in out TEvent extends { type: string }> {\n  _storedEvent: TEvent;\n  _action: ActionObject<TEvent>;\n  _state: StateNode<TContext, any>;\n}");
}
