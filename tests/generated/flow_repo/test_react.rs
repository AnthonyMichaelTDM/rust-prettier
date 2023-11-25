use pretty_assertions::assert_eq;
#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_arity_error_react_js_format_1_dcb50672() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("react.js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/**\n * @providesModule ArityError.react\n */\nvar React = require('react');\nvar AudienceInsightsContainer = React.createClass({\n  renderComponent(AudienceInsights: ReactClass<*>) {\n    return <AudienceInsights />;\n  },\n});\n\nmodule.exports = AudienceInsightsContainer;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/**\n * @providesModule ArityError.react\n */\nvar React = require(\"react\");\nvar AudienceInsightsContainer = React.createClass({\n  renderComponent(AudienceInsights: ReactClass<*>) {\n    return <AudienceInsights />;\n  },\n});\n\nmodule.exports = AudienceInsightsContainer;");
}
#[test]
fn test_create_class_js_format_1_4503caea() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\nconst A = React.createClass({\n  mixins: [{ propTypes: { foo: React.PropTypes.string.isRequired } }],\n  propTypes: { bar: React.PropTypes.number.isRequired },\n  m() {\n    (this.props.foo: empty); // error: string ~> empty\n    (this.props.bar: empty); // error: number ~> empty\n  }\n});\n\nconst B = React.createClass({\n  p: \"\",\n  m() {\n    this.p = 0; // error: number ~> string\n  },\n  mm() {\n    this.m.apply(null); // OK: this.m is autobound, so \\`this.p\\` will always be found\n  }\n});\n\nconst C = React.createClass({\n  getInitialState(): Object {\n    return { foo: 0 };\n  },\n  m() {\n    this.state.foo; // OK: state is unknown\n  }\n});\n\nconst D = React.createClass({\n  mixins: [{\n    getInitialState(): Object {\n      return { foo: 0 };\n    },\n  }],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  m() {\n    this.state.foo; // OK: state is unknown (due to unknown mixin)\n  }\n});\n\nconst E = React.createClass({\n  foo: 0,\n  m() {\n    (this.foo: string); // error: number ~> string\n  },\n  mm() {\n    var props: { m(): void } = { m: this.m };\n    props.m(); // OK: this.m is autobound, so \\`this.foo\\` will always be found\n  }\n});\n\nconst F = React.createClass({\n  getInitialState(): { [string]: mixed } {\n    return { foo: 0 };\n  },\n  m() {\n    this.state.foo;\n    this.state.bar;\n  },\n});\n\nconst G = React.createClass({\n  mixins: [],\n  autobind: true,\n  statics: {},\n  m() {\n    (this.mixins: mixed); // error: property \\`mixins\\` not found\n    (this.autobind: mixed); // error: property \\`autobind\\` not found\n    (this.statics: mixed); // error: property \\`statics\\` not found\n  },\n});\n\nconst H = React.createClass({\n  statics: { q: 0 },\n  getDefaultProps() {\n    (this.q: empty); // error: number ~> empty\n    return {};\n  },\n});\n\nconst I = React.createClass({\n  propTypes: ({}: {[string]: any}),\n  m() {\n    (this.props.foo: empty); // OK\n  }\n});\n\nconst J = React.createClass({\n  mixins: [{\n    getInitialState() {\n      return this.constructor.calculateState();\n    },\n  }],\n  statics: {\n    calculateState() {\n      return { foo: 0 };\n    },\n  },\n  m() {\n    (this.state.foo: empty); // number ~> empty\n  },\n});\n\nconst K = React.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n  getInitialState() {\n    this.mm(); // cause error in mm below\n    return this.props;\n  },\n  m() {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.foo: empty); // string ~> empty\n  },\n  mm() {\n    this.state.foo; // error: property fo not found (called by getInitialState)\n  }\n});\n\nconst L = React.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n  getInitialState() {\n    return { bar: 0 };\n  },\n  componentWillMount() {\n    (this.props.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  componentDidMount() {\n    (this.props.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  componentWillReceiveProps(nextProps) {\n    (this.props.foo: empty); // string ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  shouldComponentUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> bool\n  },\n  componentWillUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n  componentDidUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n  componentWillUnmount() {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n});\n\nReact.createClass({}); // error: spec must be [x] exact and [ ] sealed\nReact.createClass(({}: {})); // error: spec must be [ ] exact and [x] sealed") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\nconst A = React.createClass({\n  mixins: [{ propTypes: { foo: React.PropTypes.string.isRequired } }],\n  propTypes: { bar: React.PropTypes.number.isRequired },\n  m() {\n    (this.props.foo: empty); // error: string ~> empty\n    (this.props.bar: empty); // error: number ~> empty\n  },\n});\n\nconst B = React.createClass({\n  p: \"\",\n  m() {\n    this.p = 0; // error: number ~> string\n  },\n  mm() {\n    this.m.apply(null); // OK: this.m is autobound, so \\`this.p\\` will always be found\n  },\n});\n\nconst C = React.createClass({\n  getInitialState(): Object {\n    return { foo: 0 };\n  },\n  m() {\n    this.state.foo; // OK: state is unknown\n  },\n});\n\nconst D = React.createClass({\n  mixins: [\n    {\n      getInitialState(): Object {\n        return { foo: 0 };\n      },\n    },\n  ],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  m() {\n    this.state.foo; // OK: state is unknown (due to unknown mixin)\n  },\n});\n\nconst E = React.createClass({\n  foo: 0,\n  m() {\n    (this.foo: string); // error: number ~> string\n  },\n  mm() {\n    var props: { m(): void } = { m: this.m };\n    props.m(); // OK: this.m is autobound, so \\`this.foo\\` will always be found\n  },\n});\n\nconst F = React.createClass({\n  getInitialState(): { [string]: mixed } {\n    return { foo: 0 };\n  },\n  m() {\n    this.state.foo;\n    this.state.bar;\n  },\n});\n\nconst G = React.createClass({\n  mixins: [],\n  autobind: true,\n  statics: {},\n  m() {\n    (this.mixins: mixed); // error: property \\`mixins\\` not found\n    (this.autobind: mixed); // error: property \\`autobind\\` not found\n    (this.statics: mixed); // error: property \\`statics\\` not found\n  },\n});\n\nconst H = React.createClass({\n  statics: { q: 0 },\n  getDefaultProps() {\n    (this.q: empty); // error: number ~> empty\n    return {};\n  },\n});\n\nconst I = React.createClass({\n  propTypes: ({}: { [string]: any }),\n  m() {\n    (this.props.foo: empty); // OK\n  },\n});\n\nconst J = React.createClass({\n  mixins: [\n    {\n      getInitialState() {\n        return this.constructor.calculateState();\n      },\n    },\n  ],\n  statics: {\n    calculateState() {\n      return { foo: 0 };\n    },\n  },\n  m() {\n    (this.state.foo: empty); // number ~> empty\n  },\n});\n\nconst K = React.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n  getInitialState() {\n    this.mm(); // cause error in mm below\n    return this.props;\n  },\n  m() {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.foo: empty); // string ~> empty\n  },\n  mm() {\n    this.state.foo; // error: property fo not found (called by getInitialState)\n  },\n});\n\nconst L = React.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n  getInitialState() {\n    return { bar: 0 };\n  },\n  componentWillMount() {\n    (this.props.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  componentDidMount() {\n    (this.props.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  componentWillReceiveProps(nextProps) {\n    (this.props.foo: empty); // string ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    return 0; // number ~> void\n  },\n  shouldComponentUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> bool\n  },\n  componentWillUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n  componentDidUpdate(nextProps, nextState) {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    (nextProps.foo: empty); // string ~> empty\n    (nextState.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n  componentWillUnmount() {\n    (this.props.foo: empty); // string ~> empty\n    (this.state.bar: empty); // number ~> empty\n    return 0; // number ~> void\n  },\n});\n\nReact.createClass({}); // error: spec must be [x] exact and [ ] sealed\nReact.createClass(({}: {})); // error: spec must be [ ] exact and [x] sealed");
}
#[test]
fn test_create_class_initial_state_sealed_js_format_1_fd9e8cbb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\n// initial state = None\nReact.createClass({\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  }\n});\n\n// initial state = Some (exact & sealed) [lit]\nReact.createClass({\n  getInitialState() {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  }\n});\n\n// initial state = Some (exact & sealed) [annot]\nReact.createClass({\n  getInitialState(): {| p: number |} {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  }\n});\n\n// initial state = Some (inexact & sealed) [annot]\nReact.createClass({\n  getInitialState(): { p: number } {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 }); // property \\`q\\` not found\n  },\n  g() {\n    (this.state.q: empty); // property \\`q\\` not found\n  }\n});\n\n// mixins = (exact & sealed) + (exact & sealed)\nReact.createClass({\n  mixins: [{\n    getInitialState() {\n      return { foo: 0 };\n    },\n  }],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  f() {\n    this.setState({ baz: 0 });\n  },\n  g() {\n    (this.state.baz: empty); // number ~> empty\n  }\n});\n\n// mixins = (exact & sealed) + (inexact & sealed)\nReact.createClass({\n  mixins: [{\n    getInitialState(): { foo: number } {\n      return { foo: 0 };\n    },\n  }],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  f() {\n    this.setState({ baz: 0 }); // property \\`baz\\`  not found\n  },\n  g() {\n    (this.state.baz: empty); // property \\`baz\\` not found\n  }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\n// initial state = None\nReact.createClass({\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  },\n});\n\n// initial state = Some (exact & sealed) [lit]\nReact.createClass({\n  getInitialState() {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  },\n});\n\n// initial state = Some (exact & sealed) [annot]\nReact.createClass({\n  getInitialState(): {| p: number |} {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 });\n  },\n  g() {\n    (this.state.q: empty); // number ~> empty\n  },\n});\n\n// initial state = Some (inexact & sealed) [annot]\nReact.createClass({\n  getInitialState(): { p: number } {\n    return { p: 0 };\n  },\n  f() {\n    this.setState({ q: 0 }); // property \\`q\\` not found\n  },\n  g() {\n    (this.state.q: empty); // property \\`q\\` not found\n  },\n});\n\n// mixins = (exact & sealed) + (exact & sealed)\nReact.createClass({\n  mixins: [\n    {\n      getInitialState() {\n        return { foo: 0 };\n      },\n    },\n  ],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  f() {\n    this.setState({ baz: 0 });\n  },\n  g() {\n    (this.state.baz: empty); // number ~> empty\n  },\n});\n\n// mixins = (exact & sealed) + (inexact & sealed)\nReact.createClass({\n  mixins: [\n    {\n      getInitialState(): { foo: number } {\n        return { foo: 0 };\n      },\n    },\n  ],\n  getInitialState() {\n    return { bar: 0 };\n  },\n  f() {\n    this.setState({ baz: 0 }); // property \\`baz\\`  not found\n  },\n  g() {\n    (this.state.baz: empty); // property \\`baz\\` not found\n  },\n});");
}
#[test]
fn test_create_class_statics_sealed_js_format_1_0a4271dc() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\n// statics = None\nconst A = React.createClass({ p: 0 });\n(A.bar: empty); // number ~> empty (inflow below)\nA.bar = 0;\n\n// statics = Some (exact & sealed) [lit]\nconst B = React.createClass({\n  statics: { foo: 0 },\n});\n(B.foo: empty); // number ~> empty\n(B.bar: empty); // number ~> empty (inflow below)\nB.bar = 0;\n\n// statics = Some (exact & sealed) [annot]\nconst C = React.createClass({\n  statics: ({ foo: 0 }: {| foo: number |}),\n});\n(C.foo: empty); // number ~> empty\n(C.bar: empty); // number ~> empty (inflow below)\nC.bar = 0;\n\n// statics = Some (inexact & sealed) [annot]\nconst D = React.createClass({\n  statics: ({ foo: 0 }: { foo: number }),\n});\n(D.foo: empty); // number ~> empty\n(D.bar: empty); // property \\`bar\\` not found\nD.bar = 0; // property \\`bar\\` not found\n\n// mixins: (exact & sealed) + (exact & sealed)\nconst E = React.createClass({\n  mixins: [{\n    statics: { foo: 0 },\n  }],\n  statics: { bar: 0 },\n});\n(E.foo: empty); // number ~> empty\n(E.bar: empty); // number ~> empty\n(E.baz: empty); // number ~> empty (inflow below)\nE.baz = 0;\n\n// mixins: (exact & sealed) + (inexact & sealed)\nconst F = React.createClass({\n  mixins: [{\n    statics: ({ foo: 0 }: { foo: number }),\n  }],\n  statics: { bar: 0 },\n});\n(F.foo: empty); // number ~> empty\n(F.bar: empty); // number ~> empty\n(F.baz: empty); // number ~> empty (inflow below)\nF.baz = 0;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\n// statics = None\nconst A = React.createClass({ p: 0 });\n(A.bar: empty); // number ~> empty (inflow below)\nA.bar = 0;\n\n// statics = Some (exact & sealed) [lit]\nconst B = React.createClass({\n  statics: { foo: 0 },\n});\n(B.foo: empty); // number ~> empty\n(B.bar: empty); // number ~> empty (inflow below)\nB.bar = 0;\n\n// statics = Some (exact & sealed) [annot]\nconst C = React.createClass({\n  statics: ({ foo: 0 }: {| foo: number |}),\n});\n(C.foo: empty); // number ~> empty\n(C.bar: empty); // number ~> empty (inflow below)\nC.bar = 0;\n\n// statics = Some (inexact & sealed) [annot]\nconst D = React.createClass({\n  statics: ({ foo: 0 }: { foo: number }),\n});\n(D.foo: empty); // number ~> empty\n(D.bar: empty); // property \\`bar\\` not found\nD.bar = 0; // property \\`bar\\` not found\n\n// mixins: (exact & sealed) + (exact & sealed)\nconst E = React.createClass({\n  mixins: [\n    {\n      statics: { foo: 0 },\n    },\n  ],\n  statics: { bar: 0 },\n});\n(E.foo: empty); // number ~> empty\n(E.bar: empty); // number ~> empty\n(E.baz: empty); // number ~> empty (inflow below)\nE.baz = 0;\n\n// mixins: (exact & sealed) + (inexact & sealed)\nconst F = React.createClass({\n  mixins: [\n    {\n      statics: ({ foo: 0 }: { foo: number }),\n    },\n  ],\n  statics: { bar: 0 },\n});\n(F.foo: empty); // number ~> empty\n(F.bar: empty); // number ~> empty\n(F.baz: empty); // number ~> empty (inflow below)\nF.baz = 0;");
}
#[test]
fn test_create_element_string_js_format_1_890cf813() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nimport React from 'react';\n\nclass Bar extends React.Component {}\n\nclass Foo extends React.Component {\n  render() {\n    const Cmp = Math.random() < 0.5 ? 'div' : Bar;\n    return (<Cmp/>);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nimport React from \"react\";\n\nclass Bar extends React.Component {}\n\nclass Foo extends React.Component {\n  render() {\n    const Cmp = Math.random() < 0.5 ? \"div\" : Bar;\n    return <Cmp />;\n  }\n}");
}
#[test]
fn test_create_element_required_prop_string_js_format_1_a239dad3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// @flow\nimport React from 'react';\n\nclass Bar extends React.Component {\n  props: {\n    test: number,\n  };\n  render() {\n    return (\n      <div>\n        {this.props.test}\n      </div>\n    )\n  }\n}\n\nclass Foo extends React.Component {\n  render() {\n    const Cmp = Math.random() < 0.5 ? 'div' : Bar;\n    return (<Cmp/>);\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// @flow\nimport React from \"react\";\n\nclass Bar extends React.Component {\n  props: {\n    test: number,\n  };\n  render() {\n    return <div>{this.props.test}</div>;\n  }\n}\n\nclass Foo extends React.Component {\n  render() {\n    const Cmp = Math.random() < 0.5 ? \"div\" : Bar;\n    return <Cmp />;\n  }\n}");
}
#[test]
fn test_import_react_js_format_1_581d556f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nimport react from \"react\";\nimport {Component} from \"react\";\n\nvar a: Component<*,*,*> = new react.Component();\nvar b: number = new react.Component(); // Error: ReactComponent ~> number") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nimport react from \"react\";\nimport { Component } from \"react\";\n\nvar a: Component<*, *, *> = new react.Component();\nvar b: number = new react.Component(); // Error: ReactComponent ~> number");
}
#[test]
fn test_jsx_spread_js_format_1_4b0529f0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Foo = React.createClass({\n  propTypes: {\n    bar: React.PropTypes.string.isRequired,\n  },\n});\n\nvar props = {bar: 42};\nvar blah = <Foo {...props} />; // error bar, number given string expected") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Foo = React.createClass({\n  propTypes: {\n    bar: React.PropTypes.string.isRequired,\n  },\n});\n\nvar props = { bar: 42 };\nvar blah = <Foo {...props} />; // error bar, number given string expected");
}
#[test]
fn test_proptype_any_js_format_1_7c80c0bd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const React = require(\"react\");\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    foo: (0: any), // OK\n  },\n});\n\n(<AnyExample />); // OK\n(<AnyExample foo={(0: mixed)} />); // OK\n\nvar AnyFunExample = React.createClass({\n  propTypes: {\n    foo: (() => {}: Function), // OK\n  },\n});\n\n(<AnyFunExample />); // OK\n(<AnyFunExample foo={(0: mixed)} />); // OK") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const React = require(\"react\");\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    foo: (0: any), // OK\n  },\n});\n\n<AnyExample />; // OK\n<AnyExample foo={(0: mixed)} />; // OK\n\nvar AnyFunExample = React.createClass({\n  propTypes: {\n    foo: (() => {}: Function), // OK\n  },\n});\n\n<AnyFunExample />; // OK\n<AnyFunExample foo={(0: mixed)} />; // OK");
}
#[test]
fn test_proptype_array_of_js_format_1_76c8162d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(React.PropTypes.number).isRequired,\n  },\n});\n\nvar ok_empty = <Example arr={[]} />\nvar ok_numbers = <Example arr={[1, 2]} />\n\nvar fail_missing = <Example />\nvar fail_not_array = <Example arr={2} />\nvar fail_mistyped_elems = <Example arr={[1, \"foo\"]} />\n\n/* Since the \\`number\\` proptype argument is not required, React will actually\n   allow \\`null\\` and \\`undefined\\` elements in the \\`arr\\` prop, but Flow has\n   currently ignores the innter prop type's required flag. */\nvar todo_required = <Example arr={[null]} />\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(React.PropTypes.number),\n  }\n});\n\n(<OptionalExample />); // OK\n(<OptionalExample arr={[0]} />); // OK\n(<OptionalExample arr={[\"\"]} />); // error: string ~> number\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf((0:any)), // OK\n  },\n});\n\n(<AnyExample arr={0} />); // error: still needs to be an array\n(<AnyExample arr={[0]} />); // OK\n\nvar InvalidExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(0), // error: number not a prop type\n  },\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(React.PropTypes.number).isRequired,\n  },\n});\n\nvar ok_empty = <Example arr={[]} />;\nvar ok_numbers = <Example arr={[1, 2]} />;\n\nvar fail_missing = <Example />;\nvar fail_not_array = <Example arr={2} />;\nvar fail_mistyped_elems = <Example arr={[1, \"foo\"]} />;\n\n/* Since the \\`number\\` proptype argument is not required, React will actually\n   allow \\`null\\` and \\`undefined\\` elements in the \\`arr\\` prop, but Flow has\n   currently ignores the innter prop type's required flag. */\nvar todo_required = <Example arr={[null]} />;\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(React.PropTypes.number),\n  },\n});\n\n<OptionalExample />; // OK\n<OptionalExample arr={[0]} />; // OK\n<OptionalExample arr={[\"\"]} />; // error: string ~> number\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf((0: any)), // OK\n  },\n});\n\n<AnyExample arr={0} />; // error: still needs to be an array\n<AnyExample arr={[0]} />; // OK\n\nvar InvalidExample = React.createClass({\n  propTypes: {\n    arr: React.PropTypes.arrayOf(0), // error: number not a prop type\n  },\n});");
}
#[test]
fn test_proptype_custom_validator_js_format_1_3bf09f2d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const React = require(\"react\");\n\n// Custom validator must match \\`ReactPropsCheckType\\`\nvar Example = React.createClass({\n  propTypes: {\n    foo(props, propName, componentName, href) {\n      (props: empty); // ok: props is \\`any\\`\n      (propName: empty); // error: propName is a string\n      (componentName: empty); // error: componentName is a string\n      (href: empty); // error: href is an optional string\n      return (0: mixed); // error: should return ?Error\n    },\n  }\n});\n\n// Inferred prop type is optional \\`any\\`\n(<Example />);\n(<Example foo={(0: mixed)} />);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const React = require(\"react\");\n\n// Custom validator must match \\`ReactPropsCheckType\\`\nvar Example = React.createClass({\n  propTypes: {\n    foo(props, propName, componentName, href) {\n      (props: empty); // ok: props is \\`any\\`\n      (propName: empty); // error: propName is a string\n      (componentName: empty); // error: componentName is a string\n      (href: empty); // error: href is an optional string\n      return (0: mixed); // error: should return ?Error\n    },\n  },\n});\n\n// Inferred prop type is optional \\`any\\`\n<Example />;\n<Example foo={(0: mixed)} />;");
}
#[test]
fn test_proptype_func_js_format_1_d19a76c8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    func: React.PropTypes.func.isRequired\n  },\n});\n\nvar ok_void = <Example func={() => {}} />;\nvar ok_args = <Example func={(x) => {}} />;\nvar ok_retval = <Example func={() => 1} />\n\nvar fail_mistyped = <Example func={2} />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    func: React.PropTypes.func.isRequired,\n  },\n});\n\nvar ok_void = <Example func={() => {}} />;\nvar ok_args = <Example func={(x) => {}} />;\nvar ok_retval = <Example func={() => 1} />;\n\nvar fail_mistyped = <Example func={2} />;");
}
#[test]
fn test_proptype_incompatible_js_format_1_19c34d7f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const React = require(\"react\");\n\nvar Example = React.createClass({\n  propTypes: {\n    foo: 0, // error: \\`0\\` is not a prop type\n  },\n});\n\n(<Example />); // OK: don't cascade errors\n(<Example foo={(0:mixed)} />); // OK: don't cascade errors") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const React = require(\"react\");\n\nvar Example = React.createClass({\n  propTypes: {\n    foo: 0, // error: \\`0\\` is not a prop type\n  },\n});\n\n<Example />; // OK: don't cascade errors\n<Example foo={(0: mixed)} />; // OK: don't cascade errors");
}
#[test]
fn test_proptype_instance_of_js_format_1_b3fdaca7() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nclass A {}\nclass B extends A {}\nclass C extends B {}\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(B),\n  }\n});\n\n(<Example x={new A} />); // error: A ~> B\n(<Example x={new B} />); // OK\n(<Example x={new C} />); // OK (C ~> B)\n(<Example x=\"wrong\" />); // error: string ~> B\n\nclass Poly<T> {x:T}\nvar PolyExample = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(Poly).isRequired,\n  },\n  m() {\n    (this.props.x.x: empty); // OK, T instantiated to \\`any\\`\n  }\n});\n\n// Different instantiations don't interact\n(<PolyExample x={(new Poly(): Poly<string>)} />); // OK\n(<PolyExample x={(new Poly(): Poly<number>)} />); // OK\n\nclass PolyDefault<T=string> {x:T}\nvar PolyDefaultExample = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(PolyDefault).isRequired,\n  },\n  m() {\n    (this.props.x.x: empty); // OK, T instantiated to \\`any\\`\n  }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nclass A {}\nclass B extends A {}\nclass C extends B {}\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(B),\n  },\n});\n\n<Example x={new A()} />; // error: A ~> B\n<Example x={new B()} />; // OK\n<Example x={new C()} />; // OK (C ~> B)\n<Example x=\"wrong\" />; // error: string ~> B\n\nclass Poly<T> {\n  x: T;\n}\nvar PolyExample = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(Poly).isRequired,\n  },\n  m() {\n    (this.props.x.x: empty); // OK, T instantiated to \\`any\\`\n  },\n});\n\n// Different instantiations don't interact\n<PolyExample x={(new Poly(): Poly<string>)} />; // OK\n<PolyExample x={(new Poly(): Poly<number>)} />; // OK\n\nclass PolyDefault<T = string> {\n  x: T;\n}\nvar PolyDefaultExample = React.createClass({\n  propTypes: {\n    x: React.PropTypes.instanceOf(PolyDefault).isRequired,\n  },\n  m() {\n    (this.props.x.x: empty); // OK, T instantiated to \\`any\\`\n  },\n});");
}
#[test]
fn test_proptype_missing_js_format_1_5fbe8210() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\n/* If you create a react component with createClass() but don't specify the\n * propTypes, what should the type of props be?\n *\n * It used to be an empty object, but we didn't enforce that correctly, so\n * people could do whatever they wanted with this.props.\n *\n * As of 0.21.0 it started to be an error when people used this.props in a\n * strict equality situation. It was weird that this was only sometimes\n * enforced, so glevi changed this.props to be Object by default.\n *\n * We may change this back to the empty object at some point and fix the\n * situations where it didn't used to error\n */\nvar React = require('react');\nvar Foo = React.createClass({\n  getID(): string {\n    // So this would have been an error in 0.21.0 if we didn't make this.props\n    // Object\n    switch (this.props.name) {\n      case 'a': return 'Bob';\n      default: return 'Alice';\n    }\n  },\n\n  render() {\n    // But this never errored\n    return <div id={this.props.name} />;\n  }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\n/* If you create a react component with createClass() but don't specify the\n * propTypes, what should the type of props be?\n *\n * It used to be an empty object, but we didn't enforce that correctly, so\n * people could do whatever they wanted with this.props.\n *\n * As of 0.21.0 it started to be an error when people used this.props in a\n * strict equality situation. It was weird that this was only sometimes\n * enforced, so glevi changed this.props to be Object by default.\n *\n * We may change this back to the empty object at some point and fix the\n * situations where it didn't used to error\n */\nvar React = require(\"react\");\nvar Foo = React.createClass({\n  getID(): string {\n    // So this would have been an error in 0.21.0 if we didn't make this.props\n    // Object\n    switch (this.props.name) {\n      case \"a\":\n        return \"Bob\";\n      default:\n        return \"Alice\";\n    }\n  },\n\n  render() {\n    // But this never errored\n    return <div id={this.props.name} />;\n  },\n});");
}
#[test]
fn test_proptype_object_js_format_1_57660c98() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    object: React.PropTypes.object.isRequired\n  },\n});\n\nvar ok_empty = <Example object={{}} />;\nvar ok_props = <Example object={{foo: \"bar\"}} />;\n\nvar fail_mistyped = <Example object={2} />") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    object: React.PropTypes.object.isRequired,\n  },\n});\n\nvar ok_empty = <Example object={{}} />;\nvar ok_props = <Example object={{ foo: \"bar\" }} />;\n\nvar fail_mistyped = <Example object={2} />;");
}
#[test]
fn test_proptype_object_of_js_format_1_9bb74b3e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(React.PropTypes.number).isRequired\n  },\n});\n\nvar ok_empty = <Example obj={{}} />\nvar ok_numbers = <Example obj={{foo: 1, bar: 2}} />\n\nvar fail_missing = <Example />\nvar fail_not_object = <Example obj={2} />\nvar fail_mistyped_props = <Example obj={{foo: \"foo\"}} />\n\n/* Since the \\`number\\` proptype argument is not required, React will actually\n   allow \\`null\\` and \\`undefined\\` elements in the \\`obj\\` prop, but Flow has\n   currently ignores the innter prop type's required flag. */\nvar todo_required = <Example obj={{p:null}} />\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(React.PropTypes.number),\n  }\n});\n\n(<OptionalExample />); // OK\n(<OptionalExample obj={{p:0}} />); // OK\n(<OptionalExample obj={{p:\"\"}} />); // error: string ~> number\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf((0:any)), // OK\n  },\n});\n\n(<AnyExample obj={0} />); // error: still needs to be an object\n(<AnyExample obj={{p:0}} />); // OK\n\nvar InvalidExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(0), // error: number not a prop type\n  },\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(React.PropTypes.number).isRequired,\n  },\n});\n\nvar ok_empty = <Example obj={{}} />;\nvar ok_numbers = <Example obj={{ foo: 1, bar: 2 }} />;\n\nvar fail_missing = <Example />;\nvar fail_not_object = <Example obj={2} />;\nvar fail_mistyped_props = <Example obj={{ foo: \"foo\" }} />;\n\n/* Since the \\`number\\` proptype argument is not required, React will actually\n   allow \\`null\\` and \\`undefined\\` elements in the \\`obj\\` prop, but Flow has\n   currently ignores the innter prop type's required flag. */\nvar todo_required = <Example obj={{ p: null }} />;\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(React.PropTypes.number),\n  },\n});\n\n<OptionalExample />; // OK\n<OptionalExample obj={{ p: 0 }} />; // OK\n<OptionalExample obj={{ p: \"\" }} />; // error: string ~> number\n\nvar AnyExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf((0: any)), // OK\n  },\n});\n\n<AnyExample obj={0} />; // error: still needs to be an object\n<AnyExample obj={{ p: 0 }} />; // OK\n\nvar InvalidExample = React.createClass({\n  propTypes: {\n    obj: React.PropTypes.objectOf(0), // error: number not a prop type\n  },\n});");
}
#[test]
fn test_proptype_one_of_js_format_1_42499253() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    str: React.PropTypes.oneOf([\"foo\", \"bar\"]),\n    num: React.PropTypes.oneOf([0, 1, 2]),\n    bool: React.PropTypes.oneOf([true]),\n    mixed: React.PropTypes.oneOf([\"foo\", 0, true]),\n  },\n});\n\n(<Example str=\"foo\" />); // OK\n(<Example str=\"baz\" />); // error: 'baz' not in enum\n\n(<Example num={0} />); // OK\n(<Example num={3} />); // error: 3 not in enum\n\n(<Example bool={true} />); // OK\n(<Example bool={false} />); // error: false ~> true\n\n(<Example mixed={\"foo\"} />); // OK\n(<Example mixed={0} />); // OK\n(<Example mixed={\"baz\"} />); // error: 'baz' not in enum\n\nvar RequiredExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf([]).isRequired,\n  },\n});\n\n(<RequiredExample />); // error: \\`p\\` not found\n\nvar EmptyExample = React.createClass({\n  propTypes: {\n    nil: React.PropTypes.oneOf([]), // i.e., \\`empty\\`\n  },\n});\n\n(<EmptyExample nil={0} />); // number ~> empty\n\nvar AnyArrayExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOf((0:any)),\n  },\n});\n\n(<AnyArrayExample any={0} />); // OK\n\nvar AnyElemExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOf([\"foo\", (0:any)]),\n  },\n});\n\n(<AnyElemExample any={0} />); // OK\n\nvar DynamicArrayExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOf(([]: Array<string>)),\n  },\n});\n\n(<DynamicArrayExample dyn={0} />); // OK\n\nvar DynamicElemExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOf([\"foo\", (\"\": string)]),\n  },\n});\n\n(<DynamicElemExample dyn={0} />); // OK\n\nvar InvalidArrayExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf(0), // error: expected array, got 0\n  },\n});\n\n(<InvalidArrayExample p={0} />); // OK, don't cascade errors\n\nvar NonLiteralElemExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf([{}]), // OK: allow non-literals\n  },\n});\n(<NonLiteralElemExample p={0} />); // OK, result is unknown/any") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    str: React.PropTypes.oneOf([\"foo\", \"bar\"]),\n    num: React.PropTypes.oneOf([0, 1, 2]),\n    bool: React.PropTypes.oneOf([true]),\n    mixed: React.PropTypes.oneOf([\"foo\", 0, true]),\n  },\n});\n\n<Example str=\"foo\" />; // OK\n<Example str=\"baz\" />; // error: 'baz' not in enum\n\n<Example num={0} />; // OK\n<Example num={3} />; // error: 3 not in enum\n\n<Example bool={true} />; // OK\n<Example bool={false} />; // error: false ~> true\n\n<Example mixed={\"foo\"} />; // OK\n<Example mixed={0} />; // OK\n<Example mixed={\"baz\"} />; // error: 'baz' not in enum\n\nvar RequiredExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf([]).isRequired,\n  },\n});\n\n<RequiredExample />; // error: \\`p\\` not found\n\nvar EmptyExample = React.createClass({\n  propTypes: {\n    nil: React.PropTypes.oneOf([]), // i.e., \\`empty\\`\n  },\n});\n\n<EmptyExample nil={0} />; // number ~> empty\n\nvar AnyArrayExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOf((0: any)),\n  },\n});\n\n<AnyArrayExample any={0} />; // OK\n\nvar AnyElemExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOf([\"foo\", (0: any)]),\n  },\n});\n\n<AnyElemExample any={0} />; // OK\n\nvar DynamicArrayExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOf(([]: Array<string>)),\n  },\n});\n\n<DynamicArrayExample dyn={0} />; // OK\n\nvar DynamicElemExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOf([\"foo\", (\"\": string)]),\n  },\n});\n\n<DynamicElemExample dyn={0} />; // OK\n\nvar InvalidArrayExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf(0), // error: expected array, got 0\n  },\n});\n\n<InvalidArrayExample p={0} />; // OK, don't cascade errors\n\nvar NonLiteralElemExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOf([{}]), // OK: allow non-literals\n  },\n});\n<NonLiteralElemExample p={0} />; // OK, result is unknown/any");
}
#[test]
fn test_proptype_one_of_type_js_format_1_16310aee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* @flow */\n\nvar React = require('react');\nvar Example = React.createClass({\n  propTypes: {\n    prop: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      React.PropTypes.number\n    ]).isRequired\n  },\n  render() {\n    if (typeof this.props.prop === \"string\") {\n      return <div>{this.props.prop}</div>\n    } else {\n      return <div>{this.props.prop.toFixed(2)}</div>\n    }\n  }\n});\n\nvar ok_number = <Example prop={42} />;\nvar ok_string = <Example prop=\"bar\" />;\n\nvar fail_missing = <Example />;\nvar fail_bool = <Example prop={true} />;\n\n/* Since the proptype arguments are not required, React will actually allow\n   \\`null\\` and \\`undefined\\` elements in the \\`prop\\` prop, but Flow has currently\n   ignores the innter prop types' required flags. */\nvar todo_required = <Example prop={null} />;\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n    ]),\n  },\n});\n\n(<OptionalExample />); // OK\n(<OptionalExample p=\"\" />); // OK\n(<OptionalExample p={0} />); // error: number ~> string\n\nvar EmptyExample = React.createClass({\n  propTypes: {\n    nil: React.PropTypes.oneOfType([]), // i.e., \\`empty\\`\n  },\n});\n\n(<EmptyExample nil={0} />); // number ~> empty\n\nvar AnyArrayExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOfType((0:any)),\n  },\n});\n\n(<AnyArrayExample any={0} />); // OK\n\nvar AnyElemExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      (0:any),\n    ]),\n  },\n});\n\n(<AnyElemExample any={0} />); // OK\n\nvar DynamicArrayExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOfType(([]: Array<Function>)),\n  },\n});\n\n(<DynamicArrayExample dyn={0} />); // OK\n\nvar DynamicElemExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      (() => {}: Function),\n    ]),\n  },\n});\n\n(<DynamicElemExample dyn={0} />); // OK\n\nvar InvalidArrayExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType(0), // error: expected array, got 0\n  },\n});\n\n(<InvalidArrayExample p={0} />); // OK, don't cascade errors\n\nvar InvalidElemExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType([{}]), // error: expected prop type, got {}\n  },\n});\n\n(<InvalidElemExample p={0} />); // OK, don't cascade errors") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* @flow */\n\nvar React = require(\"react\");\nvar Example = React.createClass({\n  propTypes: {\n    prop: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      React.PropTypes.number,\n    ]).isRequired,\n  },\n  render() {\n    if (typeof this.props.prop === \"string\") {\n      return <div>{this.props.prop}</div>;\n    } else {\n      return <div>{this.props.prop.toFixed(2)}</div>;\n    }\n  },\n});\n\nvar ok_number = <Example prop={42} />;\nvar ok_string = <Example prop=\"bar\" />;\n\nvar fail_missing = <Example />;\nvar fail_bool = <Example prop={true} />;\n\n/* Since the proptype arguments are not required, React will actually allow\n   \\`null\\` and \\`undefined\\` elements in the \\`prop\\` prop, but Flow has currently\n   ignores the innter prop types' required flags. */\nvar todo_required = <Example prop={null} />;\n\nvar OptionalExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType([React.PropTypes.string]),\n  },\n});\n\n<OptionalExample />; // OK\n<OptionalExample p=\"\" />; // OK\n<OptionalExample p={0} />; // error: number ~> string\n\nvar EmptyExample = React.createClass({\n  propTypes: {\n    nil: React.PropTypes.oneOfType([]), // i.e., \\`empty\\`\n  },\n});\n\n<EmptyExample nil={0} />; // number ~> empty\n\nvar AnyArrayExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOfType((0: any)),\n  },\n});\n\n<AnyArrayExample any={0} />; // OK\n\nvar AnyElemExample = React.createClass({\n  propTypes: {\n    any: React.PropTypes.oneOfType([React.PropTypes.string, (0: any)]),\n  },\n});\n\n<AnyElemExample any={0} />; // OK\n\nvar DynamicArrayExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOfType(([]: Array<Function>)),\n  },\n});\n\n<DynamicArrayExample dyn={0} />; // OK\n\nvar DynamicElemExample = React.createClass({\n  propTypes: {\n    dyn: React.PropTypes.oneOfType([\n      React.PropTypes.string,\n      (() => {}: Function),\n    ]),\n  },\n});\n\n<DynamicElemExample dyn={0} />; // OK\n\nvar InvalidArrayExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType(0), // error: expected array, got 0\n  },\n});\n\n<InvalidArrayExample p={0} />; // OK, don't cascade errors\n\nvar InvalidElemExample = React.createClass({\n  propTypes: {\n    p: React.PropTypes.oneOfType([{}]), // error: expected prop type, got {}\n  },\n});\n\n<InvalidElemExample p={0} />; // OK, don't cascade errors");
}
#[test]
fn test_proptype_shape_js_format_1_3a223253() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* Shape should be a sealed, inexact object just like a type annotation. The\n * below component's \\`foo\\` property should be equivalent to \\`{ bar: string }\\`,\n * which would forbid reads/writes on an unknown \\`baz\\` property.\n *\n * If you see a single \"number incompatible with string\" error instead of two\n * separate \"property \\`baz\\` not found\" errors, this is broken and we are\n * treating the shape like an unsealed object and performing shadow read/writes.\n */\n\nimport React from \"react\";\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.shape({\n      bar: React.PropTypes.string.isRequired,\n    }).isRequired,\n  },\n\n  f() {\n    (this.props.foo.baz: string);\n  },\n\n  g() {\n    this.props.foo.baz = 0;\n  }\n});\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.shape(({}: {[string]: any})).isRequired,\n  },\n  f() {\n    (this.props.foo.bar: empty); // OK\n  },\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* Shape should be a sealed, inexact object just like a type annotation. The\n * below component's \\`foo\\` property should be equivalent to \\`{ bar: string }\\`,\n * which would forbid reads/writes on an unknown \\`baz\\` property.\n *\n * If you see a single \"number incompatible with string\" error instead of two\n * separate \"property \\`baz\\` not found\" errors, this is broken and we are\n * treating the shape like an unsealed object and performing shadow read/writes.\n */\n\nimport React from \"react\";\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.shape({\n      bar: React.PropTypes.string.isRequired,\n    }).isRequired,\n  },\n\n  f() {\n    (this.props.foo.baz: string);\n  },\n\n  g() {\n    this.props.foo.baz = 0;\n  },\n});\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.shape(({}: { [string]: any })).isRequired,\n  },\n  f() {\n    (this.props.foo.bar: empty); // OK\n  },\n});");
}
#[test]
fn test_proptypes_builtins_js_format_1_610cf014() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import React from \"react\";\n\ntype NoFun = mixed => empty;\n\n// error: mixed ~> ReactPropsCheckType\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.arrayOf : NoFun);\n\n// OK: mixed ~> any\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.instanceOf : NoFun);\n\n// error: mixed ~> ReactPropsCheckType\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.objectOf : NoFun);\n\n// error: mixed ~> Array<any>\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.oneOf : NoFun);\n\n// error: mixed ~> Array<ReactPropsCheckType>\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.oneOfType : NoFun);\n\n// error: mixed ~> object type\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.shape : NoFun);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import React from \"react\";\n\ntype NoFun = (mixed) => empty;\n\n// error: mixed ~> ReactPropsCheckType\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.arrayOf: NoFun);\n\n// OK: mixed ~> any\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.instanceOf: NoFun);\n\n// error: mixed ~> ReactPropsCheckType\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.objectOf: NoFun);\n\n// error: mixed ~> Array<any>\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.oneOf: NoFun);\n\n// error: mixed ~> Array<ReactPropsCheckType>\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.oneOfType: NoFun);\n\n// error: mixed ~> object type\n// error: ReactPropsChainableTypeChecker ~> empty\n(React.PropTypes.shape: NoFun);");
}
#[test]
fn test_proptypes_sealed_js_format_1_a15413ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("/* propTypes should be a sealed, inexact object just like a type annotation. The\n * below component's propTypes should be equivalent to \\`{ bar: string }\\`, which\n * would forbid reads/writes on an unknown \\`baz\\` property.\n *\n * If you see a single \"number incompatible with string\" error instead of two\n * separate \"property \\`baz\\` not found\" errors, this is broken and we are\n * treating propTypes like an unsealed object and performing shadow read/writes.\n */\n\nimport React from \"react\";\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n\n  f() {\n    (this.props.baz: string);\n  },\n\n  g() {\n    this.props.baz = 0;\n  }\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "/* propTypes should be a sealed, inexact object just like a type annotation. The\n * below component's propTypes should be equivalent to \\`{ bar: string }\\`, which\n * would forbid reads/writes on an unknown \\`baz\\` property.\n *\n * If you see a single \"number incompatible with string\" error instead of two\n * separate \"property \\`baz\\` not found\" errors, this is broken and we are\n * treating propTypes like an unsealed object and performing shadow read/writes.\n */\n\nimport React from \"react\";\n\nReact.createClass({\n  propTypes: {\n    foo: React.PropTypes.string.isRequired,\n  },\n\n  f() {\n    (this.props.baz: string);\n  },\n\n  g() {\n    this.props.baz = 0;\n  },\n});");
}
