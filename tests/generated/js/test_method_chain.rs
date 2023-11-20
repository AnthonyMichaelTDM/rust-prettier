#[allow(unused_imports)]
use rust_prettier::{Parsers, PrettyPrinterBuilder};
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_13018_js_format_1_baff490a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo(_a).bar().leet();\nfoo(-a).bar().leet();\nfoo(+a).bar().leet();\nfoo(~a).bar().leet();\nfoo(++a).bar().leet();\nfoo(--a).bar().leet();\nfoo(a++).bar().leet();\nfoo(a--).bar().leet();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo(_a).bar().leet();\nfoo(-a).bar().leet();\nfoo(+a).bar().leet();\nfoo(~a).bar().leet();\nfoo(++a).bar().leet();\nfoo(--a).bar().leet();\nfoo(a++).bar().leet();\nfoo(a--).bar().leet();");
}
#[test]
fn test_bracket_0_js_format_1_aa691ea0() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function a() {\n  function b() {\n\tqueryThenMutateDOM(\n      () => {\n        title = SomeThing.call(root, 'someLongStringThatPushesThisTextReallyFar')[0];\n      }\n    );\n  }\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function a() {\n  function b() {\n    queryThenMutateDOM(() => {\n      title = SomeThing.call(\n        root,\n        \"someLongStringThatPushesThisTextReallyFar\",\n      )[0];\n    });\n  }\n}");
}
#[test]
fn test_bracket_0_1_js_format_1_a0ca8034() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const thingamabobMetaAlias =\npath.scope.getProgramParent().path.get(\"body\")[0].node;",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const thingamabobMetaAlias = path.scope\n  .getProgramParent()\n  .path.get(\"body\")[0].node;");
}
#[test]
fn test_break_last_call_js_format_1_6d70c544() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default store => {\n  return callApi(endpoint, schema).then(\n    response => next(actionWith({\n      response,\n      type: successType\n    })),\n    error => next(actionWith({\n      type: failureType,\n      error: error.message || 'Something bad happened'\n    }))\n  )\n}\n\nit('should group messages with same created time', () => {\n  expect(\n    groupMessages(messages).toJS(),\n  ).toEqual({\n    '11/01/2017 13:36': [\n      {message: 'test', messageType: 'SMS', status: 'Unknown', created: '11/01/2017 13:36'},\n      {message: 'test', messageType: 'Email', status: 'Unknown', created: '11/01/2017 13:36'},\n    ],\n    '09/01/2017 17:25': [\n      {message: 'te', messageType: 'SMS', status: 'Unknown', created: '09/01/2017 17:25'},\n      {message: 'te', messageType: 'Email', status: 'Unknown', created: '09/01/2017 17:25'},\n    ],\n    '11/01/2017 13:33': [\n      {message: 'test', messageType: 'SMS', status: 'Unknown', created: '11/01/2017 13:33'},\n      {message: 'test', messageType: 'Email', status: 'Unknown', created: '11/01/2017 13:33'},\n    ],\n    '11/01/2017 13:37': [\n      {message: 'test', messageType: 'SMS', status: 'Unknown', created: '11/01/2017 13:37'},\n      {message: 'test', messageType: 'Email', status: 'Unknown', created: '11/01/2017 13:37'},\n    ],\n  });\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default (store) => {\n  return callApi(endpoint, schema).then(\n    (response) =>\n      next(\n        actionWith({\n          response,\n          type: successType,\n        }),\n      ),\n    (error) =>\n      next(\n        actionWith({\n          type: failureType,\n          error: error.message || \"Something bad happened\",\n        }),\n      ),\n  );\n};\n\nit(\"should group messages with same created time\", () => {\n  expect(groupMessages(messages).toJS()).toEqual({\n    \"11/01/2017 13:36\": [\n      {\n        message: \"test\",\n        messageType: \"SMS\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:36\",\n      },\n      {\n        message: \"test\",\n        messageType: \"Email\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:36\",\n      },\n    ],\n    \"09/01/2017 17:25\": [\n      {\n        message: \"te\",\n        messageType: \"SMS\",\n        status: \"Unknown\",\n        created: \"09/01/2017 17:25\",\n      },\n      {\n        message: \"te\",\n        messageType: \"Email\",\n        status: \"Unknown\",\n        created: \"09/01/2017 17:25\",\n      },\n    ],\n    \"11/01/2017 13:33\": [\n      {\n        message: \"test\",\n        messageType: \"SMS\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:33\",\n      },\n      {\n        message: \"test\",\n        messageType: \"Email\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:33\",\n      },\n    ],\n    \"11/01/2017 13:37\": [\n      {\n        message: \"test\",\n        messageType: \"SMS\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:37\",\n      },\n      {\n        message: \"test\",\n        messageType: \"Email\",\n        status: \"Unknown\",\n        created: \"11/01/2017 13:37\",\n      },\n    ],\n  });\n});");
}
#[test]
fn test_break_last_member_js_format_1_9752c5ca() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("SomeVeryLongUpperCaseConstant.someVeryLongCallExpression().some_very_long_member_expression\nweNeedToReachTheEightyCharacterLimitXXXXXXXXXXXXXXXXX.someNode\n  .childrenInAnArray[0];\nsuperSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered;\nsuperSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered[0];\n\nexpect(\n  findDOMNode(component.instance()).getElementsByClassName(styles.inner)[0].style.paddingRight\n).toBe('1000px');\n\nconst { course, conflicts = [], index, scheduleId, studentId, something } = a.this.props;\n\nconst { course2, conflicts2 = [], index2, scheduleId2, studentId2, something2 } = this.props;\n\nconst {\n  updated,\n  author: { identifier: ownerId },\n  location,\n  category: categories,\n} = rawAd.entry;") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "SomeVeryLongUpperCaseConstant.someVeryLongCallExpression()\n  .some_very_long_member_expression;\nweNeedToReachTheEightyCharacterLimitXXXXXXXXXXXXXXXXX.someNode\n  .childrenInAnArray[0];\nsuperSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered;\nsuperSupersuperSupersuperSupersuperSupersuperSuperLong\n  .exampleOfOrderOfGetterAndSetterReordered[0];\n\nexpect(\n  findDOMNode(component.instance()).getElementsByClassName(styles.inner)[0]\n    .style.paddingRight,\n).toBe(\"1000px\");\n\nconst {\n  course,\n  conflicts = [],\n  index,\n  scheduleId,\n  studentId,\n  something,\n} = a.this.props;\n\nconst {\n  course2,\n  conflicts2 = [],\n  index2,\n  scheduleId2,\n  studentId2,\n  something2,\n} = this.props;\n\nconst {\n  updated,\n  author: { identifier: ownerId },\n  location,\n  category: categories,\n} = rawAd.entry;");
}
#[test]
fn test_break_multiple_js_format_1_519fb536() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("object.foo().bar().baz();\n\nfoo().bar().baz();\n\nfoo().bar.baz();");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(
        formatted,
        "object.foo().bar().baz();\n\nfoo().bar().baz();\n\nfoo().bar.baz();"
    );
}
#[test]
fn test_comment_js_format_1_8248d933() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("function f() {\n  return observableFromSubscribeFunction()\n    // Debounce manually rather than using editor.onDidStopChanging so that the debounce time is\n    // configurable.\n    .debounceTime(debounceInterval);\n}\n\n_.a(a)\n  /* very very very very very very very long such that it is longer than 80 columns */\n  .a()\n\n_.a(\n  a\n)/* very very very very very very very long such that it is longer than 80 columns */\n.a();\n\n_.a(\n  a\n) /* very very very very very very very long such that it is longer than 80 columns */.a();\n\nSomething\n  // $FlowFixMe(>=0.41.0)\n  .getInstance(this.props.dao)\n  .getters()\n\n// Warm-up first\nmeasure()\n  .then(() => {\n    SomethingLong();\n  });\n\nmeasure() // Warm-up first\n  .then(() => {\n    SomethingLong();\n  });\n\nconst configModel = this.baseConfigurationService.getCache().consolidated\t\t// global/default values (do NOT modify)\n  .merge(this.cachedWorkspaceConfig);\n\nthis.doWriteConfiguration(target, value, options) // queue up writes to prevent race conditions\n  .then(() => null,\n  error => {\n    return options.donotNotifyError ? TPromise.wrapError(error) : this.onError(error, target, value);\n  });\n\nret = __DEV__ ?\n  // $FlowFixMe: this type differs according to the env\nvm.runInContext(source, ctx)\n: a\n\nangular.module('AngularAppModule')\n  // Hello, I am comment.\n  .constant('API_URL', 'http://localhost:8080/api');") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "function f() {\n  return (\n    observableFromSubscribeFunction()\n      // Debounce manually rather than using editor.onDidStopChanging so that the debounce time is\n      // configurable.\n      .debounceTime(debounceInterval)\n  );\n}\n\n_.a(a)\n  /* very very very very very very very long such that it is longer than 80 columns */\n  .a();\n\n_.a(\n  a,\n) /* very very very very very very very long such that it is longer than 80 columns */\n  .a();\n\n_.a(\n  a,\n) /* very very very very very very very long such that it is longer than 80 columns */\n  .a();\n\nSomething\n  // $FlowFixMe(>=0.41.0)\n  .getInstance(this.props.dao)\n  .getters();\n\n// Warm-up first\nmeasure().then(() => {\n  SomethingLong();\n});\n\nmeasure() // Warm-up first\n  .then(() => {\n    SomethingLong();\n  });\n\nconst configModel = this.baseConfigurationService\n  .getCache()\n  .consolidated // global/default values (do NOT modify)\n  .merge(this.cachedWorkspaceConfig);\n\nthis.doWriteConfiguration(target, value, options) // queue up writes to prevent race conditions\n  .then(\n    () => null,\n    (error) => {\n      return options.donotNotifyError\n        ? TPromise.wrapError(error)\n        : this.onError(error, target, value);\n    },\n  );\n\nret = __DEV__\n  ? // $FlowFixMe: this type differs according to the env\n    vm.runInContext(source, ctx)\n  : a;\n\nangular\n  .module(\"AngularAppModule\")\n  // Hello, I am comment.\n  .constant(\"API_URL\", \"http://localhost:8080/api\");");
}
#[test]
fn test_complex_args_js_format_1_440decba() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("client.execute(\n  Post.selectAll()\n    .where(Post.id.eq(42))\n    .where(Post.published.eq(true))\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "client.execute(\n  Post.selectAll().where(Post.id.eq(42)).where(Post.published.eq(true)),\n);");
}
#[test]
fn test_computed_js_format_1_6c288839() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("nock(/test/)\n  .matchHeader('Accept', 'application/json')\n  [httpMethodNock(method)]('/foo')\n  .reply(200, {\n    foo: 'bar',\n  });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "nock(/test/)\n  .matchHeader(\"Accept\", \"application/json\")\n  [httpMethodNock(method)](\"/foo\")\n  .reply(200, {\n    foo: \"bar\",\n  });");
}
#[test]
fn test_computed_merge_js_format_1_831ea3a3() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("[].forEach(key => {\n  data[key]('foo')\n    .then(() => console.log('bar'))\n    .catch(() => console.log('baz'));\n});\n\n[].forEach(key => {\n  data('foo')\n    [key]('bar')\n    .then(() => console.log('bar'))\n    .catch(() => console.log('baz'));\n});\n\nwindow.Data[key](\"foo\")\n  .then(() => a)\n  .catch(() => b);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "[].forEach((key) => {\n  data[key](\"foo\")\n    .then(() => console.log(\"bar\"))\n    .catch(() => console.log(\"baz\"));\n});\n\n[].forEach((key) => {\n  data(\"foo\")\n    [key](\"bar\")\n    .then(() => console.log(\"bar\"))\n    .catch(() => console.log(\"baz\"));\n});\n\nwindow.Data[key](\"foo\")\n  .then(() => a)\n  .catch(() => b);");
}
#[test]
fn test_conditional_js_format_1_ee40f47c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(a ? b : c).d();\n\n(a ? b : c).d().e();\n\n(a ? b : c).d().e().f();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser))\n.map();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser))\n.map().filter();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(defaultUser))\n.map();\n\nobject[valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(defaultUser)]\n.map();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(a ? b : c).d();\n\n(a ? b : c).d().e();\n\n(a ? b : c).d().e().f();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser)\n).map();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(this.defaultUser)\n)\n  .map()\n  .filter();\n\n(valid\n  ? helper.responseBody(this.currentUser)\n  : helper.responseBody(defaultUser)\n).map();\n\nobject[\n  valid\n    ? helper.responseBody(this.currentUser)\n    : helper.responseBody(defaultUser)\n].map();");
}
#[test]
fn test_cypress_js_format_1_7fbf1473() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("cy.get('option:first')\n  .should('be.selected')\n  .and('have.value', 'Metallica')\n\ncy.get(\".ready\")\n  .should(\"have.text\", \"FOO\")\n  .should(\"have.css\", \"color\", \"#aaa\");") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "cy.get(\"option:first\").should(\"be.selected\").and(\"have.value\", \"Metallica\");\n\ncy.get(\".ready\").should(\"have.text\", \"FOO\").should(\"have.css\", \"color\", \"#aaa\");");
}
#[test]
fn test_d_3_js_format_1_1f68be64() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("d3.select('body')\n  .append('circle')\n  .at({ width: 30, fill: '#f0f' })\n  .st({ fontWeight: 600 })\n\nconst myScale = d3.scaleLinear()\n  .domain([1950, 1980])\n  .range([0, width])\n\nnot.d3.select('body')\n  .append('circle')\n  .at({ width: 30, fill: '#f0f' })\n  .st({ fontWeight: 600 })\n\nnot.d3.scaleLinear()\n  .domain([1950, 1980])\n  .range([0, width])") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "d3.select(\"body\")\n  .append(\"circle\")\n  .at({ width: 30, fill: \"#f0f\" })\n  .st({ fontWeight: 600 });\n\nconst myScale = d3.scaleLinear().domain([1950, 1980]).range([0, width]);\n\nnot.d3\n  .select(\"body\")\n  .append(\"circle\")\n  .at({ width: 30, fill: \"#f0f\" })\n  .st({ fontWeight: 600 });\n\nnot.d3.scaleLinear().domain([1950, 1980]).range([0, width]);");
}
#[test]
fn test_first_long_js_format_1_886b5519() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("export default function theFunction(action$, store) {\n  return action$.ofType(THE_ACTION).switchMap(action => Observable\n    .webSocket({\n      url: THE_URL,\n      more: stuff(),\n      evenMore: stuff({\n        value1: true,\n        value2: false,\n        value3: false\n      })\n    })\n    .filter(data => theFilter(data))\n    .map(({ theType, ...data }) => theMap(theType, data))\n    .retryWhen(errors => errors));\n}\n\nfunction f() {\n  return this._getWorker(workerOptions)({\n    filePath,\n    hasteImplModulePath: this._options.hasteImplModulePath,\n  }).then(\n    metadata => {\n      // \\`1\\` for truthy values instead of \\`true\\` to save cache space.\n      fileMetadata[H.VISITED] = 1;\n      const metadataId = metadata.id;\n      const metadataModule = metadata.module;\n      if (metadataId && metadataModule) {\n        fileMetadata[H.ID] = metadataId;\n        setModule(metadataId, metadataModule);\n      }\n      fileMetadata[H.DEPENDENCIES] = metadata.dependencies || [];\n    }\n  );\n}") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "export default function theFunction(action$, store) {\n  return action$.ofType(THE_ACTION).switchMap((action) =>\n    Observable.webSocket({\n      url: THE_URL,\n      more: stuff(),\n      evenMore: stuff({\n        value1: true,\n        value2: false,\n        value3: false,\n      }),\n    })\n      .filter((data) => theFilter(data))\n      .map(({ theType, ...data }) => theMap(theType, data))\n      .retryWhen((errors) => errors),\n  );\n}\n\nfunction f() {\n  return this._getWorker(workerOptions)({\n    filePath,\n    hasteImplModulePath: this._options.hasteImplModulePath,\n  }).then((metadata) => {\n    // \\`1\\` for truthy values instead of \\`true\\` to save cache space.\n    fileMetadata[H.VISITED] = 1;\n    const metadataId = metadata.id;\n    const metadataModule = metadata.module;\n    if (metadataId && metadataModule) {\n      fileMetadata[H.ID] = metadataId;\n      setModule(metadataId, metadataModule);\n    }\n    fileMetadata[H.DEPENDENCIES] = metadata.dependencies || [];\n  });\n}");
}
#[test]
fn test_fluent_configuration_js_format_1_2fdfcd84() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("domain\n    .concept('Page')\n    .val('title', 'string')\n    .vals('widgets', 'Widget')\ndomain\n    .concept('Widget')\n    .val('title', 'string')\n    .val('color', 'Color')\n    .val('foo', 'Foo')\n    .val('bar', 'Bar')\ndomain\n    .concept('Widget')\n    .val('title', 'string')\n    .val('color', 'Color')\ndomain\n    .concept(CONCEPT_NAME)\n    .val('title')\n    .vals()") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "domain.concept(\"Page\").val(\"title\", \"string\").vals(\"widgets\", \"Widget\");\ndomain\n  .concept(\"Widget\")\n  .val(\"title\", \"string\")\n  .val(\"color\", \"Color\")\n  .val(\"foo\", \"Foo\")\n  .val(\"bar\", \"Bar\");\ndomain.concept(\"Widget\").val(\"title\", \"string\").val(\"color\", \"Color\");\ndomain.concept(CONCEPT_NAME).val(\"title\").vals();");
}
#[test]
fn test_inline_merge_js_format_1_7662eb31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("Object.keys(\n  availableLocales({\n    test: true\n  })\n)\n.forEach(locale => {\n  // ...\n});\n\nthis.layoutPartsToHide = this.utils.hashset(\n\t_.flatMap(this.visibilityHandlers, fn => fn())\n\t\t.concat(this.record.resolved_legacy_visrules)\n\t\t.filter(Boolean)\n);\n\nvar jqxhr = $.ajax(\"example.php\")\n  .done(doneFn)\n  .fail(failFn);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "Object.keys(\n  availableLocales({\n    test: true,\n  }),\n).forEach((locale) => {\n  // ...\n});\n\nthis.layoutPartsToHide = this.utils.hashset(\n  _.flatMap(this.visibilityHandlers, (fn) => fn())\n    .concat(this.record.resolved_legacy_visrules)\n    .filter(Boolean),\n);\n\nvar jqxhr = $.ajax(\"example.php\").done(doneFn).fail(failFn);");
}
#[test]
fn test_issue_3594_js_format_1_5e86d515() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const fetched = fetch(\"/foo\");\nfetched\n\t.then(response => response.json())\n\t.then(json => processThings(json.data.things));\n\nlet column = new Column(null, conn)\n    .table(data.table)\n    .json(data.column);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const fetched = fetch(\"/foo\");\nfetched\n  .then((response) => response.json())\n  .then((json) => processThings(json.data.things));\n\nlet column = new Column(null, conn).table(data.table).json(data.column);");
}
#[test]
fn test_issue_3621_js_format_1_e5e853b8() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const palindrome = str => {\n  const s = str.toLowerCase().replace(/[\\\\W_]/g, '');\n  return s === s.split('').reverse().join('');\n};\n\nconst apiCurrencies = api().currencies().all()\n\nexpect(cells.at(1).render().text()).toBe('link text1')\nexpect(cells.at(2).render().text()).toBe('link text2')\nexpect(cells.at(3).render().text()).toBe('link text3')\nexpect(cells.at(4).render().text()).toBe('link text4')") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const palindrome = (str) => {\n  const s = str.toLowerCase().replace(/[\\\\W_]/g, \"\");\n  return s === s.split(\"\").reverse().join(\"\");\n};\n\nconst apiCurrencies = api().currencies().all();\n\nexpect(cells.at(1).render().text()).toBe(\"link text1\");\nexpect(cells.at(2).render().text()).toBe(\"link text2\");\nexpect(cells.at(3).render().text()).toBe(\"link text3\");\nexpect(cells.at(4).render().text()).toBe(\"link text4\");");
}
#[test]
fn test_issue_4125_js_format_1_b32b9004() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// examples from https://github.com/prettier/prettier/issues/4125\n\nconst sha256 = (data) => crypto.createHash(\"sha256\").update(data).digest(\"hex\");\n\nreq.checkBody('id').isInt().optional();\nreq.checkBody('name').notEmpty().optional();\n\nconst x = moment().add(1, 'day').valueOf()\n\n// should stay on one line:\nconst y = obj.foo(1).foo(2).foo(3);\nconst z = obj.foo(-1).foo(import('2')).foo(!x).check(/[A-Z]/);\n\n// better on multiple lines:\nsomePromise.then(format).then((val)=>doSomething(val)).catch((err)=>handleError(err))\n\n// you can still force multi-line chaining with a comment:\nconst sha256_2 = (data) =>\n  crypto // breakme\n    .createHash(\"sha256\")\n    .update(data)\n    .digest(\"hex\");\n\n// examples from https://github.com/prettier/prettier/pull/4765\n\nif ($(el).attr(\"href\").includes(\"/wiki/\")) {\n}\n\nif ($(el).attr(\"href\").includes(\"/wiki/\")) {\n  if ($(el).attr(\"xyz\").includes(\"/whatever/\")) {\n    if ($(el).attr(\"hello\").includes(\"/world/\")) {\n    }\n  }\n}\n\nconst parseNumbers = s => s.split('').map(Number).sort()\n\nfunction palindrome(a, b) {\n  return a.slice().reverse().join(',') === b.slice().sort().join(',');\n}\n\n// examples from https://github.com/prettier/prettier/issues/1565\n\nd3.select(\"body\").selectAll(\"p\").data([1, 2, 3]).enter().style(\"color\", \"white\");\n\nObject.keys(props).filter(key => key in own === false).reduce((a, key) => {\n  a[key] = props[key];\n  return a;\n}, {})\n\npoint().x(4).y(3).z(6).plot();\n\nassert.equal(this.$().text().trim(), '1000');\n\nsomething().then(() => doSomethingElse()).then(result => dontForgetThisAsWell(result))\n\ndb.branch(\n  db.table('users').filter({ email }).count(),\n  db.table('users').filter({ email: 'a@b.com' }).count(),\n  db.table('users').insert({ email }),\n  db.table('users').filter({ email }),\n)\n\nsandbox.stub(config, 'get').withArgs('env').returns('dev')\n\nconst date = moment.utc(userInput).hour(0).minute(0).second(0)\n\nfetchUser(id)\n  .then(fetchAccountForUser)\n  .catch(handleFetchError)\n\nfetchUser(id) //\n  .then(fetchAccountForUser)\n  .catch(handleFetchError)\n\n// examples from https://github.com/prettier/prettier/issues/3107\n\nfunction HelloWorld() {\n  window.FooClient.setVars({\n    locale: getFooLocale({ page }),\n    authorizationToken: data.token,\n  }).initVerify('foo_container');\n\n  fejax.ajax({\n    url: '/verification/',\n    dataType: 'json',\n  }).then(\n    (data) => {\n      this.setState({ isLoading: false });\n      this.initWidget(data);\n    },\n    (data) => {\n      this.logImpression('foo_fetch_error', data);\n      Flash.error(I18n.t('offline_identity.foo_issue'));\n    },\n  );\n}\n\naction$.ofType(ActionTypes.SEARCHED_USERS)\n  .map(action => action.payload.query)\n  .filter(q => !!q)\n  .switchMap(q =>\n    Observable.timer(800) // debounce\n      .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))\n      .mergeMap(() =>\n        Observable.merge(\n          Observable.of(replace(\\`?q=\\${q}\\`)),\n          ajax\n            .getJSON(\\`https://api.github.com/search/users?q=\\${q}\\`)\n            .map(res => res.items)\n            .map(receiveUsers)\n        )\n      )\n  );\n\nwindow.FooClient\n  .setVars({\n    locale: getFooLocale({ page }),\n    authorizationToken: data.token,\n  })\n  .initVerify('foo_container');\n\nit('gets triggered by mouseenter', () => {\n  const wrapper = shallow(<CalendarDay />);\n  wrapper.dive().find(Button).prop();\n});\n\nconst a1 = x.a(true).b(null).c(123)\nconst a2 = x.d('').e(\\`\\`).f(g)\nconst a3 = x.d('').e(\\`\\${123}\\`).f(g)\nconst a4 = x.h(i.j).k(l()).m([n, o])\nclass X {\n  y() {\n    const j = x.a(this).b(super.cde()).f(/g/).h(new i()).j();\n  }\n}\n\n// should break when call expressions get complex\nx.a().b([c, [d, [e]]]).f()\nx.a().b(c(d(e()))).f()\nx.a().b(\\`\\${c(d())}\\`).f()\n\nxyz.a().b().c(a(a(b(c(d().p).p).p).p))\n\nvar l = base\n    .replace(/^\\\\w*:\\\\/\\\\//, '')\n    .replace(/\\\\/$/, '')\n    .split('/').length\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// examples from https://github.com/prettier/prettier/issues/4125\n\nconst sha256 = (data) => crypto.createHash(\"sha256\").update(data).digest(\"hex\");\n\nreq.checkBody(\"id\").isInt().optional();\nreq.checkBody(\"name\").notEmpty().optional();\n\nconst x = moment().add(1, \"day\").valueOf();\n\n// should stay on one line:\nconst y = obj.foo(1).foo(2).foo(3);\nconst z = obj.foo(-1).foo(import(\"2\")).foo(!x).check(/[A-Z]/);\n\n// better on multiple lines:\nsomePromise\n  .then(format)\n  .then((val) => doSomething(val))\n  .catch((err) => handleError(err));\n\n// you can still force multi-line chaining with a comment:\nconst sha256_2 = (data) =>\n  crypto // breakme\n    .createHash(\"sha256\")\n    .update(data)\n    .digest(\"hex\");\n\n// examples from https://github.com/prettier/prettier/pull/4765\n\nif ($(el).attr(\"href\").includes(\"/wiki/\")) {\n}\n\nif ($(el).attr(\"href\").includes(\"/wiki/\")) {\n  if ($(el).attr(\"xyz\").includes(\"/whatever/\")) {\n    if ($(el).attr(\"hello\").includes(\"/world/\")) {\n    }\n  }\n}\n\nconst parseNumbers = (s) => s.split(\"\").map(Number).sort();\n\nfunction palindrome(a, b) {\n  return a.slice().reverse().join(\",\") === b.slice().sort().join(\",\");\n}\n\n// examples from https://github.com/prettier/prettier/issues/1565\n\nd3.select(\"body\")\n  .selectAll(\"p\")\n  .data([1, 2, 3])\n  .enter()\n  .style(\"color\", \"white\");\n\nObject.keys(props)\n  .filter((key) => key in own === false)\n  .reduce((a, key) => {\n    a[key] = props[key];\n    return a;\n  }, {});\n\npoint().x(4).y(3).z(6).plot();\n\nassert.equal(this.$().text().trim(), \"1000\");\n\nsomething()\n  .then(() => doSomethingElse())\n  .then((result) => dontForgetThisAsWell(result));\n\ndb.branch(\n  db.table(\"users\").filter({ email }).count(),\n  db.table(\"users\").filter({ email: \"a@b.com\" }).count(),\n  db.table(\"users\").insert({ email }),\n  db.table(\"users\").filter({ email }),\n);\n\nsandbox.stub(config, \"get\").withArgs(\"env\").returns(\"dev\");\n\nconst date = moment.utc(userInput).hour(0).minute(0).second(0);\n\nfetchUser(id).then(fetchAccountForUser).catch(handleFetchError);\n\nfetchUser(id) //\n  .then(fetchAccountForUser)\n  .catch(handleFetchError);\n\n// examples from https://github.com/prettier/prettier/issues/3107\n\nfunction HelloWorld() {\n  window.FooClient.setVars({\n    locale: getFooLocale({ page }),\n    authorizationToken: data.token,\n  }).initVerify(\"foo_container\");\n\n  fejax\n    .ajax({\n      url: \"/verification/\",\n      dataType: \"json\",\n    })\n    .then(\n      (data) => {\n        this.setState({ isLoading: false });\n        this.initWidget(data);\n      },\n      (data) => {\n        this.logImpression(\"foo_fetch_error\", data);\n        Flash.error(I18n.t(\"offline_identity.foo_issue\"));\n      },\n    );\n}\n\naction$\n  .ofType(ActionTypes.SEARCHED_USERS)\n  .map((action) => action.payload.query)\n  .filter((q) => !!q)\n  .switchMap((q) =>\n    Observable.timer(800) // debounce\n      .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))\n      .mergeMap(() =>\n        Observable.merge(\n          Observable.of(replace(\\`?q=\\${q}\\`)),\n          ajax\n            .getJSON(\\`https://api.github.com/search/users?q=\\${q}\\`)\n            .map((res) => res.items)\n            .map(receiveUsers),\n        ),\n      ),\n  );\n\nwindow.FooClient.setVars({\n  locale: getFooLocale({ page }),\n  authorizationToken: data.token,\n}).initVerify(\"foo_container\");\n\nit(\"gets triggered by mouseenter\", () => {\n  const wrapper = shallow(<CalendarDay />);\n  wrapper.dive().find(Button).prop();\n});\n\nconst a1 = x.a(true).b(null).c(123);\nconst a2 = x.d(\"\").e(\\`\\`).f(g);\nconst a3 = x.d(\"\").e(\\`\\${123}\\`).f(g);\nconst a4 = x.h(i.j).k(l()).m([n, o]);\nclass X {\n  y() {\n    const j = x.a(this).b(super.cde()).f(/g/).h(new i()).j();\n  }\n}\n\n// should break when call expressions get complex\nx.a()\n  .b([c, [d, [e]]])\n  .f();\nx.a()\n  .b(c(d(e())))\n  .f();\nx.a()\n  .b(\\`\\${c(d())}\\`)\n  .f();\n\nxyz\n  .a()\n  .b()\n  .c(a(a(b(c(d().p).p).p).p));\n\nvar l = base\n  .replace(/^\\\\w*:\\\\/\\\\//, \"\")\n  .replace(/\\\\/$/, \"\")\n  .split(\"/\").length;");
}
#[test]
fn test_issue_11298_js_format_1_190bca31() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer
        .format("foo1(/𠮟𠮟𠮟/).foo2(bar).foo3(baz);\n\nfoo1(/叱叱叱/).foo2(bar).foo3(baz);");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo1(/𠮟𠮟𠮟/)\n  .foo2(bar)\n  .foo3(baz);\n\nfoo1(/叱叱叱/)\n  .foo2(bar)\n  .foo3(baz);");
}
#[test]
fn test_logical_js_format_1_7448747f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const someLongVariableName = (idx(\n  this.props,\n  props => props.someLongPropertyName\n) || []\n).map(edge => edge.node);\n\n(veryLongVeryLongVeryLong || e).map(tickets =>\n  TicketRecord.createFromSomeLongString());\n\n(veryLongVeryLongVeryLong || e).map(tickets =>\n  TicketRecord.createFromSomeLongString()).filter(obj => !!obj);\n\n(veryLongVeryLongVeryLong || anotherVeryLongVeryLongVeryLong || veryVeryVeryLongError).map(tickets =>\n  TicketRecord.createFromSomeLongString());\n\n(veryLongVeryLongVeryLong || anotherVeryLongVeryLongVeryLong || veryVeryVeryLongError).map(tickets =>\n  TicketRecord.createFromSomeLongString()).filter(obj => !!obj);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const someLongVariableName = (\n  idx(this.props, (props) => props.someLongPropertyName) || []\n).map((edge) => edge.node);\n\n(veryLongVeryLongVeryLong || e).map((tickets) =>\n  TicketRecord.createFromSomeLongString(),\n);\n\n(veryLongVeryLongVeryLong || e)\n  .map((tickets) => TicketRecord.createFromSomeLongString())\n  .filter((obj) => !!obj);\n\n(\n  veryLongVeryLongVeryLong ||\n  anotherVeryLongVeryLongVeryLong ||\n  veryVeryVeryLongError\n).map((tickets) => TicketRecord.createFromSomeLongString());\n\n(\n  veryLongVeryLongVeryLong ||\n  anotherVeryLongVeryLongVeryLong ||\n  veryVeryVeryLongError\n)\n  .map((tickets) => TicketRecord.createFromSomeLongString())\n  .filter((obj) => !!obj);");
}
#[test]
fn test_multiple_members_js_format_1_88f341e1() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("if (testConfig.ENABLE_ONLINE_TESTS === \"true\") {\n  describe(\"POST /users/me/pet\", function() {\n    it(\"saves pet\", function() {\n      function assert(pet) {\n        expect(pet).to.have.property(\"OwnerAddress\").that.deep.equals({\n          AddressLine1: \"Alexanderstrasse\",\n          AddressLine2: \"\",\n          PostalCode: \"10999\",\n          Region: \"Berlin\",\n          City: \"Berlin\",\n          Country: \"DE\"\n        });\n      }\n    });\n  });\n}\n\nwrapper.find('SomewhatLongNodeName').prop('longPropFunctionName')().then(function() {\n  doSomething();\n});\n\nwrapper.find('SomewhatLongNodeName').prop('longPropFunctionName')('argument').then(function() {\n  doSomething();\n});\n\nwrapper.find('SomewhatLongNodeName').prop('longPropFunctionName', 'second argument that pushes this group past 80 characters')('argument').then(function() {\n  doSomething();\n});\n\nwrapper.find('SomewhatLongNodeName').prop('longPropFunctionName')('argument', 'second argument that pushes this group past 80 characters').then(function() {\n  doSomething();\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "if (testConfig.ENABLE_ONLINE_TESTS === \"true\") {\n  describe(\"POST /users/me/pet\", function () {\n    it(\"saves pet\", function () {\n      function assert(pet) {\n        expect(pet).to.have.property(\"OwnerAddress\").that.deep.equals({\n          AddressLine1: \"Alexanderstrasse\",\n          AddressLine2: \"\",\n          PostalCode: \"10999\",\n          Region: \"Berlin\",\n          City: \"Berlin\",\n          Country: \"DE\",\n        });\n      }\n    });\n  });\n}\n\nwrapper\n  .find(\"SomewhatLongNodeName\")\n  .prop(\"longPropFunctionName\")()\n  .then(function () {\n    doSomething();\n  });\n\nwrapper\n  .find(\"SomewhatLongNodeName\")\n  .prop(\"longPropFunctionName\")(\"argument\")\n  .then(function () {\n    doSomething();\n  });\n\nwrapper\n  .find(\"SomewhatLongNodeName\")\n  .prop(\n    \"longPropFunctionName\",\n    \"second argument that pushes this group past 80 characters\",\n  )(\"argument\")\n  .then(function () {\n    doSomething();\n  });\n\nwrapper\n  .find(\"SomewhatLongNodeName\")\n  .prop(\"longPropFunctionName\")(\n    \"argument\",\n    \"second argument that pushes this group past 80 characters\",\n  )\n  .then(function () {\n    doSomething();\n  });");
}
#[test]
fn test_object_literal_js_format_1_c3ae3918() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("of(\"test\")\n  .pipe(throwIfEmpty())\n  .subscribe({\n    error(err) {\n      thrown = err;\n    }\n  });\n\nof(\"test\")\n  .pipe(throwIfEmpty())\n  .subscribe({\n    get foo() {\n      bar();\n    }\n  });") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "of(\"test\")\n  .pipe(throwIfEmpty())\n  .subscribe({\n    error(err) {\n      thrown = err;\n    },\n  });\n\nof(\"test\")\n  .pipe(throwIfEmpty())\n  .subscribe({\n    get foo() {\n      bar();\n    },\n  });");
}
#[test]
fn test_pr_7889_js_format_1_3f8d25d4() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const Profile = view.with({ name: (state) => state.name }).as((props) => (\n  <div>\n    <h1>Hello, {props.name}</h1>\n  </div>\n))\n\nconst Profile2 = view.with({ name }).as((props) => (\n  <div>\n    <h1>Hello, {props.name}</h1>\n  </div>\n))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const Profile = view.with({ name: (state) => state.name }).as((props) => (\n  <div>\n    <h1>Hello, {props.name}</h1>\n  </div>\n));\n\nconst Profile2 = view.with({ name }).as((props) => (\n  <div>\n    <h1>Hello, {props.name}</h1>\n  </div>\n));");
}
#[test]
fn test_short_names_js_format_1_df4f578d() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const svgJsFiles = fs\n  .readdirSync(svgDir)\n  .filter(f => svgJsFileExtRegex.test(f))\n  .map(f => path.join(svgDir, f));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const svgJsFiles = fs\n  .readdirSync(svgDir)\n  .filter((f) => svgJsFileExtRegex.test(f))\n  .map((f) => path.join(svgDir, f));");
}
#[test]
fn test_simple_args_js_format_1_293ae6fd() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const fieldsToSend = _([\"id\", extra]).without(\"transition\").uniq();\n\nconsole.log(values.filter(isValid).map(extractId).slice(-5, -1));") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const fieldsToSend = _([\"id\", extra]).without(\"transition\").uniq();\n\nconsole.log(values.filter(isValid).map(extractId).slice(-5, -1));");
}
#[test]
fn test_square_0_js_format_1_d378aa74() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const version = someLongString\n  .split('jest version =')\n  .pop()\n  .split(EOL)[0]\n  .trim();\n\nconst component = find('.org-lclp-edit-copy-url-banner__link')[0]\n  .getAttribute('href')\n  .indexOf(this.landingPageLink);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const version = someLongString\n  .split(\"jest version =\")\n  .pop()\n  .split(EOL)[0]\n  .trim();\n\nconst component = find(\".org-lclp-edit-copy-url-banner__link\")[0]\n  .getAttribute(\"href\")\n  .indexOf(this.landingPageLink);");
}
#[test]
fn test_test_js_format_1_30e0346e() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("method().then(x => x)\n  [\"abc\"](x => x)\n  [abc](x => x);\n\n({}.a().b());\n({}).a().b();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "method()\n  .then((x) => x)\n  [\"abc\"]((x) => x)\n  [abc]((x) => x);\n\n({}).a().b();\n({}).a().b();");
}
#[test]
fn test_this_js_format_1_4480a943() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("const sel = this.connections\n  .concat(this.activities.concat(this.operators))\n  .filter(x => x.selected);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const sel = this.connections\n  .concat(this.activities.concat(this.operators))\n  .filter((x) => x.selected);");
}
#[test]
fn test_tuple_and_record_js_acorn_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_espree_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_flow_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_meriyah_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_typescript_format_1_d41d8cd9() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .build()
        .unwrap();
    let formatted = pretty_printer.format("");
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq!(formatted, "");
}
#[test]
fn test_tuple_and_record_js_format_1_636c064c() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("foo.a().b().c([n, o])\nfoo.a().b().c(#[n, o])\n\nfoo.a().b().c({n, o})\nfoo.a().b().c(#{n, o})") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "foo.a().b().c([n, o]);\nfoo.a().b().c(#[n, o]);\n\nfoo.a().b().c({ n, o });\nfoo.a().b().c(#{ n, o });");
}
