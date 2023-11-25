#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_functional_compose_js_format_1_e366f24a() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("compose(\n  sortBy(x => x),\n  flatten,\n  map(x => [x, x*2])\n);\n\nsomelib.compose(\n  sortBy(x => x),\n  flatten,\n  map(x => [x, x*2])\n);\n\ncomposeFlipped(\n  sortBy(x => x),\n  flatten,\n  map(x => [x, x*2])\n);\n\nsomelib.composeFlipped(\n  sortBy(x => x),\n  flatten,\n  map(x => [x, x*2])\n);\n\n// no regression (#4602)\nconst hasValue = hasOwnProperty(a, b);\n\nthis.compose(sortBy(x => x), flatten);\nthis.a.b.c.compose(sortBy(x => x), flatten);\nsomeObj.someMethod(this.field.compose(a, b));\n\nclass A extends B {\n  compose() {\n    super.compose(sortBy(x => x), flatten);\n  }\n}\n\nthis.subscriptions.add(\n            this.componentUpdates\n                .pipe(startWith(this.props), distinctUntilChanged(isEqual))\n                .subscribe(props => {\n\n                })\n        )") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "compose(\n  sortBy((x) => x),\n  flatten,\n  map((x) => [x, x * 2]),\n);\n\nsomelib.compose(\n  sortBy((x) => x),\n  flatten,\n  map((x) => [x, x * 2]),\n);\n\ncomposeFlipped(\n  sortBy((x) => x),\n  flatten,\n  map((x) => [x, x * 2]),\n);\n\nsomelib.composeFlipped(\n  sortBy((x) => x),\n  flatten,\n  map((x) => [x, x * 2]),\n);\n\n// no regression (#4602)\nconst hasValue = hasOwnProperty(a, b);\n\nthis.compose(\n  sortBy((x) => x),\n  flatten,\n);\nthis.a.b.c.compose(\n  sortBy((x) => x),\n  flatten,\n);\nsomeObj.someMethod(this.field.compose(a, b));\n\nclass A extends B {\n  compose() {\n    super.compose(\n      sortBy((x) => x),\n      flatten,\n    );\n  }\n}\n\nthis.subscriptions.add(\n  this.componentUpdates\n    .pipe(startWith(this.props), distinctUntilChanged(isEqual))\n    .subscribe((props) => {}),\n);");
}
#[test]
fn test_gobject_connect_js_format_1_3620be42() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("button.connect(\n  \"clicked\",\n  () => doSomething()\n);\napp.connect(\n  \"activate\",\n  async () => {\n    await data.load();\n    win.show_all();\n  }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "button.connect(\"clicked\", () => doSomething());\napp.connect(\"activate\", async () => {\n  await data.load();\n  win.show_all();\n});");
}
#[test]
fn test_lodash_flow_js_format_1_ccd1f0aa() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { flow } from \"lodash\";\n\nconst foo = flow(\n  x => x + 1,\n  x => x * 3,\n  x => x - 6,\n);\n\nfoo(6);\n\nimport * as _ from \"lodash\";\n\nconst bar = _.flow(\n  x => x + 1,\n  x => x * 3,\n  x => x - 6,\n);\n\nbar(6);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { flow } from \"lodash\";\n\nconst foo = flow(\n  (x) => x + 1,\n  (x) => x * 3,\n  (x) => x - 6,\n);\n\nfoo(6);\n\nimport * as _ from \"lodash\";\n\nconst bar = _.flow(\n  (x) => x + 1,\n  (x) => x * 3,\n  (x) => x - 6,\n);\n\nbar(6);");
}
#[test]
fn test_lodash_flow_right_js_format_1_973d73e2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { flowRight } from \"lodash\";\n\nconst foo = flowRight(\n  x => x + 1,\n  x => x * 3,\n  x => x - 6,\n);\n\nfoo(6);\n\nimport * as _ from \"lodash\";\n\nconst bar = _.flowRight(\n  x => x + 1,\n  x => x * 3,\n  x => x - 6,\n);\n\nbar(6);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { flowRight } from \"lodash\";\n\nconst foo = flowRight(\n  (x) => x + 1,\n  (x) => x * 3,\n  (x) => x - 6,\n);\n\nfoo(6);\n\nimport * as _ from \"lodash\";\n\nconst bar = _.flowRight(\n  (x) => x + 1,\n  (x) => x * 3,\n  (x) => x - 6,\n);\n\nbar(6);");
}
#[test]
fn test_mongo_connect_js_format_1_e63a6796() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("MongoClient.connect(\n  \"mongodb://localhost:27017/posts\",\n  (err, db) => {\n    assert.equal(null, err);\n    db.close();\n  }\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "MongoClient.connect(\"mongodb://localhost:27017/posts\", (err, db) => {\n  assert.equal(null, err);\n  db.close();\n});");
}
#[test]
fn test_pipe_function_calls_js_format_1_aa5881b2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(() => {\n  pipe(\n    timelines,\n    everyCommitTimestamps,\n    A.sort(ordDate),\n    A.head\n  );\n\n  pipe(\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError)\n      )\n    )\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err) => {\n      logger.error(\n        pipe(\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message))\n        )\n      );\n      process.exit(1);\n    });\n\n  pipe(\n    Changelog.timestampOfFirstCommit([[commit]]),\n    O.toUndefined\n  );\n\n  chain(\n    flow(\n      getUploadUrl,\n      E.mapLeft(Errors.unknownError),\n      TE.fromEither\n    )\n  );\n})();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(() => {\n  pipe(timelines, everyCommitTimestamps, A.sort(ordDate), A.head);\n\n  pipe(\n    serviceEventFromMessage(msg),\n    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError))),\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err) => {\n      logger.error(\n        pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message))),\n      );\n      process.exit(1);\n    });\n\n  pipe(Changelog.timestampOfFirstCommit([[commit]]), O.toUndefined);\n\n  chain(flow(getUploadUrl, E.mapLeft(Errors.unknownError), TE.fromEither));\n})();");
}
#[test]
fn test_pipe_function_calls_with_comments_js_format_1_c9d152ab() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// input with some comments added to avoid reformatting\n\n(() => {\n  pipe(\n    // add a descriptive comment here\n    timelines,\n    everyCommitTimestamps,\n    A.sort(ordDate),\n    A.head\n  );\n\n  pipe(\n    // add a descriptive comment here\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        // add a descriptive comment here\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError)\n      )\n    )\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err) => {\n      logger.error(\n        pipe(\n          // add a descriptive comment here\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message))\n        )\n      );\n      process.exit(1);\n    });\n\n  pipe(\n    // add a descriptive comment here\n    Changelog.timestampOfFirstCommit([[commit]]),\n    O.toUndefined\n  );\n\n  chain(\n    flow(\n      // add a descriptive comment here\n      getUploadUrl,\n      E.mapLeft(Errors.unknownError),\n      TE.fromEither\n    )\n  );\n})();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// input with some comments added to avoid reformatting\n\n(() => {\n  pipe(\n    // add a descriptive comment here\n    timelines,\n    everyCommitTimestamps,\n    A.sort(ordDate),\n    A.head,\n  );\n\n  pipe(\n    // add a descriptive comment here\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        // add a descriptive comment here\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError),\n      ),\n    ),\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err) => {\n      logger.error(\n        pipe(\n          // add a descriptive comment here\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message)),\n        ),\n      );\n      process.exit(1);\n    });\n\n  pipe(\n    // add a descriptive comment here\n    Changelog.timestampOfFirstCommit([[commit]]),\n    O.toUndefined,\n  );\n\n  chain(\n    flow(\n      // add a descriptive comment here\n      getUploadUrl,\n      E.mapLeft(Errors.unknownError),\n      TE.fromEither,\n    ),\n  );\n})();");
}
#[test]
fn test_ramda_compose_js_format_1_9ec930ee() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var classyGreeting = (firstName, lastName) =>\n  \"The name's \" + lastName + \", \" + firstName + \" \" + lastName;\nvar yellGreeting = R.compose(R.toUpper, classyGreeting);\nyellGreeting(\"James\", \"Bond\"); //=> \"THE NAME'S BOND, JAMES BOND\"\n\nR.compose(Math.abs, R.add(1), R.multiply(2))(-4); //=> 7\n\n//  get :: String -> Object -> Maybe *\nvar get = R.curry((propName, obj) => Maybe(obj[propName]));\n\n//  getStateCode :: Maybe String -> Maybe String\nvar getStateCode = R.composeK(\n  R.compose(Maybe.of, R.toUpper),\n  get(\"state\"),\n  get(\"address\"),\n  get(\"user\")\n);\ngetStateCode({ user: { address: { state: \"ny\" } } }); //=> Maybe.Just(\"NY\")\ngetStateCode({}); //=> Maybe.Nothing()\n\nvar db = {\n  users: {\n    JOE: {\n      name: \"Joe\",\n      followers: [\"STEVE\", \"SUZY\"]\n    }\n  }\n};\n\n// We'll pretend to do a db lookup which returns a promise\nvar lookupUser = userId => Promise.resolve(db.users[userId]);\nvar lookupFollowers = user => Promise.resolve(user.followers);\nlookupUser(\"JOE\").then(lookupFollowers);\n\n//  followersForUser :: String -> Promise [UserId]\nvar followersForUser = R.composeP(lookupFollowers, lookupUser);\nfollowersForUser(\"JOE\").then(followers => console.log(\"Followers:\", followers));\n// Followers: [\"STEVE\",\"SUZY\"]\n\nconst mapStateToProps = state => ({\n  users: R.compose( R.filter(R.propEq('status', 'active')),\n    R.values)(state.users)\n});") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var classyGreeting = (firstName, lastName) =>\n  \"The name's \" + lastName + \", \" + firstName + \" \" + lastName;\nvar yellGreeting = R.compose(R.toUpper, classyGreeting);\nyellGreeting(\"James\", \"Bond\"); //=> \"THE NAME'S BOND, JAMES BOND\"\n\nR.compose(Math.abs, R.add(1), R.multiply(2))(-4); //=> 7\n\n//  get :: String -> Object -> Maybe *\nvar get = R.curry((propName, obj) => Maybe(obj[propName]));\n\n//  getStateCode :: Maybe String -> Maybe String\nvar getStateCode = R.composeK(\n  R.compose(Maybe.of, R.toUpper),\n  get(\"state\"),\n  get(\"address\"),\n  get(\"user\"),\n);\ngetStateCode({ user: { address: { state: \"ny\" } } }); //=> Maybe.Just(\"NY\")\ngetStateCode({}); //=> Maybe.Nothing()\n\nvar db = {\n  users: {\n    JOE: {\n      name: \"Joe\",\n      followers: [\"STEVE\", \"SUZY\"],\n    },\n  },\n};\n\n// We'll pretend to do a db lookup which returns a promise\nvar lookupUser = (userId) => Promise.resolve(db.users[userId]);\nvar lookupFollowers = (user) => Promise.resolve(user.followers);\nlookupUser(\"JOE\").then(lookupFollowers);\n\n//  followersForUser :: String -> Promise [UserId]\nvar followersForUser = R.composeP(lookupFollowers, lookupUser);\nfollowersForUser(\"JOE\").then((followers) =>\n  console.log(\"Followers:\", followers),\n);\n// Followers: [\"STEVE\",\"SUZY\"]\n\nconst mapStateToProps = (state) => ({\n  users: R.compose(\n    R.filter(R.propEq(\"status\", \"active\")),\n    R.values,\n  )(state.users),\n});");
}
#[test]
fn test_ramda_pipe_js_format_1_08f01bb6() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("var f = R.pipe(Math.pow, R.negate, R.inc);\n\nf(3, 4); // -(3^4) + 1\n\n//  parseJson :: String -> Maybe *\n//  get :: String -> Object -> Maybe *\n\n//  getStateCode :: Maybe String -> Maybe String\nvar getStateCode = R.pipeK(\n  parseJson,\n  get(\"user\"),\n  get(\"address\"),\n  get(\"state\"),\n  R.compose(Maybe.of, R.toUpper)\n);\n\ngetStateCode('{\"user\":{\"address\":{\"state\":\"ny\"}}}');\n//=> Just('NY')\ngetStateCode(\"[Invalid JSON]\");\n//=> Nothing()\n\n//  followersForUser :: String -> Promise [User]\nvar followersForUser = R.pipeP(db.getUserById, db.getFollowers);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "var f = R.pipe(Math.pow, R.negate, R.inc);\n\nf(3, 4); // -(3^4) + 1\n\n//  parseJson :: String -> Maybe *\n//  get :: String -> Object -> Maybe *\n\n//  getStateCode :: Maybe String -> Maybe String\nvar getStateCode = R.pipeK(\n  parseJson,\n  get(\"user\"),\n  get(\"address\"),\n  get(\"state\"),\n  R.compose(Maybe.of, R.toUpper),\n);\n\ngetStateCode('{\"user\":{\"address\":{\"state\":\"ny\"}}}');\n//=> Just('NY')\ngetStateCode(\"[Invalid JSON]\");\n//=> Nothing()\n\n//  followersForUser :: String -> Promise [User]\nvar followersForUser = R.pipeP(db.getUserById, db.getFollowers);");
}
#[test]
fn test_redux_compose_js_format_1_e7bcbbd5() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { createStore, applyMiddleware, compose } from 'redux';\nimport thunk from 'redux-thunk';\nimport DevTools from './containers/DevTools';\nimport reducer from '../reducers';\n\nconst store = createStore(\n  reducer,\n  compose(\n    applyMiddleware(thunk),\n    DevTools.instrument()\n  )\n)\n") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { createStore, applyMiddleware, compose } from \"redux\";\nimport thunk from \"redux-thunk\";\nimport DevTools from \"./containers/DevTools\";\nimport reducer from \"../reducers\";\n\nconst store = createStore(\n  reducer,\n  compose(applyMiddleware(thunk), DevTools.instrument()),\n);");
}
#[test]
fn test_redux_connect_js_format_1_6ddbe472() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer.format(
        "const ArtistInput = connect(mapStateToProps, mapDispatchToProps, mergeProps)(Component);",
    );
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "const ArtistInput = connect(\n  mapStateToProps,\n  mapDispatchToProps,\n  mergeProps,\n)(Component);");
}
#[test]
fn test_reselect_createselector_js_format_1_a0b77bbb() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { createSelector } from 'reselect';\n\nconst foo = createSelector(\n  getIds,\n  getObjects,\n  (ids, objects) => ids.map(id => objects[id])\n);\n\nconst bar = createSelector(\n  [getIds, getObjects],\n  (ids, objects) => ids.map(id => objects[id])\n);") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { createSelector } from \"reselect\";\n\nconst foo = createSelector(getIds, getObjects, (ids, objects) =>\n  ids.map((id) => objects[id]),\n);\n\nconst bar = createSelector([getIds, getObjects], (ids, objects) =>\n  ids.map((id) => objects[id]),\n);");
}
#[test]
fn test_rxjs_pipe_js_format_1_5aaecd29() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("js")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("import { range } from 'rxjs/observable/range';\nimport { map, filter, scan } from 'rxjs/operators';\n\nconst source$ = range(0, 10);\n\nsource$.pipe(\n  filter(x => x % 2 === 0),\n  map(x => x + x),\n  scan((acc, x) => acc + x, 0)\n)\n.subscribe(x => console.log(x))") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "import { range } from \"rxjs/observable/range\";\nimport { map, filter, scan } from \"rxjs/operators\";\n\nconst source$ = range(0, 10);\n\nsource$\n  .pipe(\n    filter((x) => x % 2 === 0),\n    map((x) => x + x),\n    scan((acc, x) => acc + x, 0),\n  )\n  .subscribe((x) => console.log(x));");
}
