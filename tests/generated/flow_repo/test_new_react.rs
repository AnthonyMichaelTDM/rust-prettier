#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_feed_ufi_react_js_format_1_713fc1cc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n *\n * @providesModule FeedUFI.react\n * @flow\n */\n\n'use strict';\n\nvar UFILikeCount = require('UFILikeCount.react');\nvar React = require('react');\n\nvar FeedUFI = React.createClass({\n  _renderLikeCount: function(\n      feedback: any\n  ) {\n    var props = {\n      className: \"\",\n      key: \"\",\n      feedback: {feedback},\n      permalink: \"\",\n    };\n    var ignored = <UFILikeCount {...props} />;\n    return (\n      <UFILikeCount\n        className=\"\"\n        key=\"\"\n        feedback={feedback}\n        permalink=\"\"\n      />\n    );\n  },\n\n  render: function(): ?React.Element<any> {\n    return (\n      <div/>\n    );\n  }\n\n});\n\nmodule.exports = FeedUFI;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n *\n * @providesModule FeedUFI.react\n * @flow\n */\n\n\"use strict\";\n\nvar UFILikeCount = require(\"UFILikeCount.react\");\nvar React = require(\"react\");\n\nvar FeedUFI = React.createClass({\n  _renderLikeCount: function (feedback: any) {\n    var props = {\n      className: \"\",\n      key: \"\",\n      feedback: { feedback },\n      permalink: \"\",\n    };\n    var ignored = <UFILikeCount {...props} />;\n    return (\n      <UFILikeCount className=\"\" key=\"\" feedback={feedback} permalink=\"\" />\n    );\n  },\n\n  render: function (): ?React.Element<any> {\n    return <div />;\n  },\n});\n\nmodule.exports = FeedUFI;");
}
#[test]
fn test_mixin_js_format_1_3b637222() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("/* @providesModule Mixin */\nmodule.exports = {\n    success: function() { }\n};");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "/* @providesModule Mixin */\nmodule.exports = {\n  success: function () {},\n};"
    );
}
#[test]
fn test_ufi_like_count_react_js_format_1_e864ec94() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n *\n * @providesModule UFILikeCount.react\n * @flow\n */\n\n'use strict';\n\nvar React = require('react');\n\nvar UFILikeCount = React.createClass({\n  propTypes: {\n    permalink: React.PropTypes.string,\n    feedback: React.PropTypes.object.isRequired\n  },\n\n  render: function(): ?React.Element<any> {\n    return <div/>;\n  }\n});\n\nmodule.exports = UFILikeCount;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * Copyright 2004-present Facebook. All Rights Reserved.\n *\n * @providesModule UFILikeCount.react\n * @flow\n */\n\n\"use strict\";\n\nvar React = require(\"react\");\n\nvar UFILikeCount = React.createClass({\n  propTypes: {\n    permalink: React.PropTypes.string,\n    feedback: React.PropTypes.object.isRequired,\n  },\n\n  render: function (): ?React.Element<any> {\n    return <div />;\n  },\n});\n\nmodule.exports = UFILikeCount;");
}
#[test]
fn test_bad_default_props_js_format_1_b349b577() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\ntype T1 = { }\ntype T2 = { x: number }\ntype T3 = { x: number, y: number }\n\nclass C1 extends React.Component<T1, T2, any> { // error\n}\n\nclass C2 extends React.Component<void, T2, any> { // OK\n}\n\n// no need to add type arguments to React.Component\nclass C3 extends React.Component { // OK\n  static defaultProps: T1;\n  props: T2;\n}\n\nclass C4 extends React.Component { // OK, recommended\n  // no need to declare defaultProps unless necessary\n  props: T2;\n}\n\nclass C5 extends React.Component<T2, T3, any> { // error\n}\n\nclass C6 extends React.Component { // OK, recommended\n  static defaultProps: T2;\n  props: T3;\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\ntype T1 = {};\ntype T2 = { x: number };\ntype T3 = { x: number, y: number };\n\nclass C1 extends React.Component<T1, T2, any> {\n  // error\n}\n\nclass C2 extends React.Component<void, T2, any> {\n  // OK\n}\n\n// no need to add type arguments to React.Component\nclass C3 extends React.Component {\n  // OK\n  static defaultProps: T1;\n  props: T2;\n}\n\nclass C4 extends React.Component {\n  // OK, recommended\n  // no need to declare defaultProps unless necessary\n  props: T2;\n}\n\nclass C5 extends React.Component<T2, T3, any> {\n  // error\n}\n\nclass C6 extends React.Component {\n  // OK, recommended\n  static defaultProps: T2;\n  props: T3;\n}");
}
#[test]
fn test_classes_js_format_1_19d3002d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\ntype DefaultProps = { };\ntype Props = { x: number };\ntype State = { y: number };\n\nclass Foo extends React.Component {\n  props: Props;\n  state: State;\n  static defaultProps: DefaultProps;\n\n  is_mounted: boolean;\n\n  static bar(): void {}\n\n  qux(): void {\n    var _: string = this.props.x;\n  }\n\n  constructor(props) {\n    super(props);\n    this.state = { y: \"\" };\n  }\n\n  setState(o: { y_: string }): void { }\n\n  componentDidMount(): void {\n    this.is_mounted = true;\n  }\n\n  componentWillReceiveProps(\n    nextProps: Object,\n    nextContext: any\n  ): void {\n    this.qux();\n  }\n\n}\n\nFoo.defaultProps = 0;\nvar foo: $jsx<number> = <Foo/>;\n\nFoo.bar();\n\nvar FooLegacy = React.createClass({\n  is_mounted: (undefined: ?boolean),\n\n  propTypes: {\n    x: React.PropTypes.number.isRequired\n  },\n\n  getDefaultProps(): DefaultProps { return {} },\n\n  statics: {\n    bar(): void {}\n  },\n\n  qux(): void {\n    var _: string = this.props.x;\n  },\n\n  getInitialState(): { y: string } {\n    return { y: \"\" };\n  },\n\n  setState(o: { y_: string }): void { },\n\n  componentDidMount(): void {\n    this.is_mounted = true;\n  },\n\n  componentWillReceiveProps(\n    nextProps: Object,\n    nextContext: any\n  ): void {\n    this.qux();\n  },\n});\n\nFooLegacy.defaultProps = 0;\nvar foo_legacy: $jsx<number> = <FooLegacy/>;\n\nFooLegacy.bar();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\ntype DefaultProps = {};\ntype Props = { x: number };\ntype State = { y: number };\n\nclass Foo extends React.Component {\n  props: Props;\n  state: State;\n  static defaultProps: DefaultProps;\n\n  is_mounted: boolean;\n\n  static bar(): void {}\n\n  qux(): void {\n    var _: string = this.props.x;\n  }\n\n  constructor(props) {\n    super(props);\n    this.state = { y: \"\" };\n  }\n\n  setState(o: { y_: string }): void {}\n\n  componentDidMount(): void {\n    this.is_mounted = true;\n  }\n\n  componentWillReceiveProps(nextProps: Object, nextContext: any): void {\n    this.qux();\n  }\n}\n\nFoo.defaultProps = 0;\nvar foo: $jsx<number> = <Foo />;\n\nFoo.bar();\n\nvar FooLegacy = React.createClass({\n  is_mounted: (undefined: ?boolean),\n\n  propTypes: {\n    x: React.PropTypes.number.isRequired,\n  },\n\n  getDefaultProps(): DefaultProps {\n    return {};\n  },\n\n  statics: {\n    bar(): void {},\n  },\n\n  qux(): void {\n    var _: string = this.props.x;\n  },\n\n  getInitialState(): { y: string } {\n    return { y: \"\" };\n  },\n\n  setState(o: { y_: string }): void {},\n\n  componentDidMount(): void {\n    this.is_mounted = true;\n  },\n\n  componentWillReceiveProps(nextProps: Object, nextContext: any): void {\n    this.qux();\n  },\n});\n\nFooLegacy.defaultProps = 0;\nvar foo_legacy: $jsx<number> = <FooLegacy />;\n\nFooLegacy.bar();");
}
#[test]
fn test_import_react_js_format_1_ac0c84db() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n// Testing local binding of React in all kinds of ways. The only reason this\n// might even be an issue is that internally, the use of JSX triggers an\n// implicit require('react'), so any bugs in (1) interop of CJS require and ES6\n// import (2) module re-export, as used to redirect the module name 'React' to\n// 'react' might show up here.\n\nimport React from \"react\";\n//import React from \"React\";\n//var React = require(\"react\");\n//var React = require(\"React\");\n\nclass HelloMessage extends React.Component {\n  props: { name: string };\n}\n\n<HelloMessage name={007} />; // number ~/~> string error\n<HelloMessage name=\"Bond\" />; // ok") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n// Testing local binding of React in all kinds of ways. The only reason this\n// might even be an issue is that internally, the use of JSX triggers an\n// implicit require('react'), so any bugs in (1) interop of CJS require and ES6\n// import (2) module re-export, as used to redirect the module name 'React' to\n// 'react' might show up here.\n\nimport React from \"react\";\n//import React from \"React\";\n//var React = require(\"react\");\n//var React = require(\"React\");\n\nclass HelloMessage extends React.Component {\n  props: { name: string };\n}\n\n<HelloMessage name={007} />; // number ~/~> string error\n<HelloMessage name=\"Bond\" />; // ok");
}
#[test]
fn test_new_react_js_format_1_39bf4352() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar Mixin = require('Mixin');\nvar C = React.createClass({\n    mixins: [Mixin],\n    propTypes: {\n        x: React.PropTypes.string.isRequired,\n        y: React.PropTypes.array,\n        z: React.PropTypes.number\n    },\n    replaceProps(props: { }) { },\n\n    getDefaultProps(): { z: number } {\n        return { z: 0 };\n    },\n    getInitialState() { return null; },\n    render() {\n        var foo: string = this.state;\n        var bar: string = this.props;\n        var qux: string = this.props.z;\n        var w:number = this.props.x;\n        this.props.y[0];\n        var len:number = this.props.x.length;\n        this.success();\n        return <div/>;\n    }\n\n})\n\nvar element = <C x = {0}/>;\nvar element_ = <C/>;\n\nvar x: number = C.displayName;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar Mixin = require(\"Mixin\");\nvar C = React.createClass({\n  mixins: [Mixin],\n  propTypes: {\n    x: React.PropTypes.string.isRequired,\n    y: React.PropTypes.array,\n    z: React.PropTypes.number,\n  },\n  replaceProps(props: {}) {},\n\n  getDefaultProps(): { z: number } {\n    return { z: 0 };\n  },\n  getInitialState() {\n    return null;\n  },\n  render() {\n    var foo: string = this.state;\n    var bar: string = this.props;\n    var qux: string = this.props.z;\n    var w: number = this.props.x;\n    this.props.y[0];\n    var len: number = this.props.x.length;\n    this.success();\n    return <div />;\n  },\n});\n\nvar element = <C x={0} />;\nvar element_ = <C />;\n\nvar x: number = C.displayName;");
}
#[test]
fn test_prop_types_js_format_1_dc5ec592() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar PropTypes = React.PropTypes;\n\nvar C = React.createClass({\n  propTypes: {\n    statistics: PropTypes.arrayOf(PropTypes.shape({\n      label: PropTypes.string.isRequired,\n      value: PropTypes.number,\n    })).isRequired,\n  }\n});\n\n<C statistics={[\n  {},\n  {label:\"\",value:undefined},\n]}/>; // error (label is required, value not required)\n\nvar props: Array<{label: string, value?: number}> = [\n  {},\n  {label:\"\",value:undefined},\n]; // error (same as ^)") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar PropTypes = React.PropTypes;\n\nvar C = React.createClass({\n  propTypes: {\n    statistics: PropTypes.arrayOf(\n      PropTypes.shape({\n        label: PropTypes.string.isRequired,\n        value: PropTypes.number,\n      }),\n    ).isRequired,\n  },\n});\n\n<C statistics={[{}, { label: \"\", value: undefined }]} />; // error (label is required, value not required)\n\nvar props: Array<{ label: string, value?: number }> = [\n  {},\n  { label: \"\", value: undefined },\n]; // error (same as ^)");
}
#[test]
fn test_props_js_format_1_888b7513() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar TestProps = React.createClass({\n\n    propTypes: {\n        x: React.PropTypes.string,\n        z: React.PropTypes.number\n    },\n\n    getDefaultProps: function() {\n        return {x: '', y: 0}\n    },\n\n    test: function() {\n        var a: number = this.props.x; // error\n        var b: string = this.props.y; // error\n        var c: string = this.props.z; // error\n    }\n});\n\nvar element = <TestProps x={false} y={false} z={false} />; // 3 errors\n\n(element: $jsx<*>);\n(element: $jsx<TestProps>);\nvar FooProps = React.createClass({\n    propTypes: { w: React.PropTypes.string.isRequired }\n});\n(element: $jsx<FooProps>);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar TestProps = React.createClass({\n  propTypes: {\n    x: React.PropTypes.string,\n    z: React.PropTypes.number,\n  },\n\n  getDefaultProps: function () {\n    return { x: \"\", y: 0 };\n  },\n\n  test: function () {\n    var a: number = this.props.x; // error\n    var b: string = this.props.y; // error\n    var c: string = this.props.z; // error\n  },\n});\n\nvar element = <TestProps x={false} y={false} z={false} />; // 3 errors\n\n(element: $jsx<*>);\n(element: $jsx<TestProps>);\nvar FooProps = React.createClass({\n  propTypes: { w: React.PropTypes.string.isRequired },\n});\n(element: $jsx<FooProps>);");
}
#[test]
fn test_props_2_js_format_1_4ceb5961() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar C = React.createClass({\n    propTypes: {\n        foo: React.PropTypes.string.isRequired,\n        bar: React.PropTypes.string.isRequired,\n    }\n});\nvar D = React.createClass({\n    getInitialState: function(): { bar: number } {\n        return { bar: 0 };\n    },\n    render: function() {\n        var obj = { bar: 0 };\n        var s: string = this.state.bar;\n        return <C {...this.state} foo = {0} />;\n    }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar C = React.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n    bar: React.PropTypes.string.isRequired,\n  },\n});\nvar D = React.createClass({\n  getInitialState: function (): { bar: number } {\n    return { bar: 0 };\n  },\n  render: function () {\n    var obj = { bar: 0 };\n    var s: string = this.state.bar;\n    return <C {...this.state} foo={0} />;\n  },\n});");
}
#[test]
fn test_props_3_js_format_1_c370556c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar TestProps = React.createClass({\n    // Do something illegal inside of propTypes and make sure Flow notices\n    propTypes: {\n      arr: React.PropTypes.array,\n      arr_rec: React.PropTypes.array.isRequired,\n      bool: React.PropTypes.bool,\n      bool_rec: React.PropTypes.bool.isRequired,\n      func: React.PropTypes.func,\n      func_rec: React.PropTypes.func.isRequired,\n      number: React.PropTypes.number,\n      number_rec: React.PropTypes.number.isRequired,\n      object: React.PropTypes.object,\n      object_rec: React.PropTypes.object.isRequired,\n      string: React.PropTypes.string,\n      string_rec: React.PropTypes.string.isRequired,\n\n      any: React.PropTypes.any,\n      any_rec: React.PropTypes.any.isRequired,\n      element: React.PropTypes.element,\n      element_rec: React.PropTypes.element.isRequired,\n      node: React.PropTypes.node,\n      node_rec: React.PropTypes.node.isRequired,\n\n      arrayOf: React.PropTypes.arrayOf(React.PropTypes.string),\n      arrayOf_rec: React.PropTypes.arrayOf(React.PropTypes.string).isRequired,\n      instanceOf: React.PropTypes.instanceOf(Object),\n      instanceOf_rec: React.PropTypes.instanceOf(Object).isRequired,\n      objectOf: React.PropTypes.objectOf(React.PropTypes.string),\n      objectOf_rec: React.PropTypes.objectOf(React.PropTypes.string).isRequired,\n      oneOf: React.PropTypes.oneOf([\"yes\", \"no\"]),\n      oneOf_rec: React.PropTypes.oneOf([\"yes\", \"no\"]).isRequired,\n      oneOfType: React.PropTypes.oneOfType(\n        [React.PropTypes.string, React.PropTypes.number]\n      ),\n      oneOfType_rec: React.PropTypes.oneOfType(\n        [React.PropTypes.string, React.PropTypes.number]\n      ).isRequired,\n      shape: React.PropTypes.shape({\n        foo: React.PropTypes.string,\n        bar: React.PropTypes.number,\n      }),\n      shape_rec: React.PropTypes.shape({\n        foo: React.PropTypes.string,\n        bar: React.PropTypes.number,\n      }).isRequired,\n\n      // And do something bad here\n      bad_one: React.PropTypes.imaginaryType,\n      bad_two: React.PropTypes.string.inRequired,\n    },\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar TestProps = React.createClass({\n  // Do something illegal inside of propTypes and make sure Flow notices\n  propTypes: {\n    arr: React.PropTypes.array,\n    arr_rec: React.PropTypes.array.isRequired,\n    bool: React.PropTypes.bool,\n    bool_rec: React.PropTypes.bool.isRequired,\n    func: React.PropTypes.func,\n    func_rec: React.PropTypes.func.isRequired,\n    number: React.PropTypes.number,\n    number_rec: React.PropTypes.number.isRequired,\n    object: React.PropTypes.object,\n    object_rec: React.PropTypes.object.isRequired,\n    string: React.PropTypes.string,\n    string_rec: React.PropTypes.string.isRequired,\n\n    any: React.PropTypes.any,\n    any_rec: React.PropTypes.any.isRequired,\n    element: React.PropTypes.element,\n    element_rec: React.PropTypes.element.isRequired,\n    node: React.PropTypes.node,\n    node_rec: React.PropTypes.node.isRequired,\n\n    arrayOf: React.PropTypes.arrayOf(React.PropTypes.string),\n    arrayOf_rec: React.PropTypes.arrayOf(React.PropTypes.string).isRequired,\n    instanceOf: React.PropTypes.instanceOf(Object),\n    instanceOf_rec: React.PropTypes.instanceOf(Object).isRequired,\n    objectOf: React.PropTypes.objectOf(React.PropTypes.string),\n    objectOf_rec: React.PropTypes.objectOf(React.PropTypes.string).isRequired,\n    oneOf: React.PropTypes.oneOf([\"yes\", \"no\"]),\n    oneOf_rec: React.PropTypes.oneOf([\"yes\", \"no\"]).isRequired,\n    oneOfType: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      React.PropTypes.number,\n    ]),\n    oneOfType_rec: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      React.PropTypes.number,\n    ]).isRequired,\n    shape: React.PropTypes.shape({\n      foo: React.PropTypes.string,\n      bar: React.PropTypes.number,\n    }),\n    shape_rec: React.PropTypes.shape({\n      foo: React.PropTypes.string,\n      bar: React.PropTypes.number,\n    }).isRequired,\n\n    // And do something bad here\n    bad_one: React.PropTypes.imaginaryType,\n    bad_two: React.PropTypes.string.inRequired,\n  },\n});");
}
#[test]
fn test_props_4_js_format_1_1938f428() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nimport React from \"React\";\n\nclass JDiv extends React.Component {\n  // static defaultProps: { };\n  props: {\n    id: string\n  };\n}\n\n// Should be a type error ('id' takes a string, not a number..)\n<JDiv id={42} />;\n\nclass Example extends React.Component {\n  props: { bar: string };\n\n  render() {\n    return <div>{this.props.bar}</div>\n  }\n}\n\nReact.render(\n  <Example foo=\"foo\" />,\n  document.body\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nimport React from \"React\";\n\nclass JDiv extends React.Component {\n  // static defaultProps: { };\n  props: {\n    id: string,\n  };\n}\n\n// Should be a type error ('id' takes a string, not a number..)\n<JDiv id={42} />;\n\nclass Example extends React.Component {\n  props: { bar: string };\n\n  render() {\n    return <div>{this.props.bar}</div>;\n  }\n}\n\nReact.render(<Example foo=\"foo\" />, document.body);");
}
#[test]
fn test_props_5_js_format_1_abffba53() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\nvar C = React.createClass({\n  getDefaultProps: function() {\n    return { x: 0 };\n  }\n});\n\nmodule.exports = C;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\nvar C = React.createClass({\n  getDefaultProps: function () {\n    return { x: 0 };\n  },\n});\n\nmodule.exports = C;");
}
#[test]
fn test_state_js_format_1_2331c816() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\n\ntype State = {\n    bar: ?{ qux: string; };\n};\n\nvar ReactClass = React.createClass({\n    getInitialState: function():State {\n        return { bar: null };\n    },\n\n    render: function(): any {\n        // Any state access here seems to make state any\n        this.state;\n        return (\n        <div>\n                {this.state.bar.qux}\n        </div>\n        );\n    }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\n\ntype State = {\n  bar: ?{ qux: string },\n};\n\nvar ReactClass = React.createClass({\n  getInitialState: function (): State {\n    return { bar: null };\n  },\n\n  render: function (): any {\n    // Any state access here seems to make state any\n    this.state;\n    return <div>{this.state.bar.qux}</div>;\n  },\n});");
}
#[test]
fn test_state_2_js_format_1_8f387e9d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\n\nvar React = require('react');\n\ntype FooState = {\n    key: ?Object;\n};\n\nvar Comp = React.createClass({\n    getInitialState: function(): FooState {\n        return {\n            key: null, // this used to cause a missing annotation error\n        };\n    }\n});\n\nmodule.exports = Comp;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\n\nvar React = require(\"react\");\n\ntype FooState = {\n  key: ?Object,\n};\n\nvar Comp = React.createClass({\n  getInitialState: function (): FooState {\n    return {\n      key: null, // this used to cause a missing annotation error\n    };\n  },\n});\n\nmodule.exports = Comp;");
}
#[test]
fn test_state_3_js_format_1_cbdd803e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('react');\nvar TestState = React.createClass({\n\n    getInitialState: function(): { x: string; } {\n        return {\n            x: ''\n        }\n    },\n\n    test: function() {\n        var a: number = this.state.x; // error\n\n        this.setState({\n            x: false // error\n        })\n    }\n\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"react\");\nvar TestState = React.createClass({\n  getInitialState: function (): { x: string } {\n    return {\n      x: \"\",\n    };\n  },\n\n  test: function () {\n    var a: number = this.state.x; // error\n\n    this.setState({\n      x: false, // error\n    });\n  },\n});");
}
#[test]
fn test_state_4_js_format_1_d66a0244() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\nvar C = React.createClass({\n  getInitialState: function(): { x: number } {\n    return { x: 0 };\n  },\n\n  render() {\n    this.setState({ y: 0 });\n    return <div>{this.state.z}</div>\n  }\n\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\nvar C = React.createClass({\n  getInitialState: function (): { x: number } {\n    return { x: 0 };\n  },\n\n  render() {\n    this.setState({ y: 0 });\n    return <div>{this.state.z}</div>;\n  },\n});");
}
#[test]
fn test_state_5_js_format_1_dea0a683() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parsers(vec!["flow"])
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var React = require('React');\n\nclass C extends React.Component {\n  foo(): number {\n    return this.state.x; // error: need to declare type of state\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var React = require(\"React\");\n\nclass C extends React.Component {\n  foo(): number {\n    return this.state.x; // error: need to declare type of state\n  }\n}");
}
