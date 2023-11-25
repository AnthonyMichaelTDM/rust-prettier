#[allow(unused_imports)]
use rust_prettier::PrettyPrinterBuilder;
#[allow(dead_code)]
static INFINITY: usize = usize::MAX;
#[test]
fn test_pipe_function_calls_ts_format_1_971d750f() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("(() => {\n\n  pipe(\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError)\n      )\n    )\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err: Error) => {\n      logger.error(\n        pipe(\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message))\n        )\n      );\n      process.exit(1);\n    });\n})();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "(() => {\n  pipe(\n    serviceEventFromMessage(msg),\n    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError))),\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err: Error) => {\n      logger.error(\n        pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message))),\n      );\n      process.exit(1);\n    });\n})();");
}
#[test]
fn test_pipe_function_calls_with_comments_ts_format_1_99976dc2() {
    let pretty_printer = PrettyPrinterBuilder::default()
        .parser("ts")
        .print_width(80)
        .build()
        .unwrap();
    let formatted = pretty_printer . format ("// input with some comments added to avoid reformatting\n\n(() => {\n  pipe(\n    // add a descriptive comment here\n    timelines,\n    everyCommitTimestamps,\n    A.sort(ordDate),\n    A.head\n  );\n\n  pipe(\n    // add a descriptive comment here\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        // add a descriptive comment here\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError)\n      )\n    )\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err: Error) => {\n      logger.error(\n        pipe(\n          // add a descriptive comment here\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message))\n        )\n      );\n      process.exit(1);\n    });\n\n  pipe(\n    // add a descriptive comment here\n    Changelog.timestampOfFirstCommit([[commit]]),\n    O.toUndefined\n  );\n\n  chain(\n    flow(\n      // add a descriptive comment here\n      getUploadUrl,\n      E.mapLeft(Errors.unknownError),\n      TE.fromEither\n    )\n  );\n})();") ;
    assert!(formatted.is_ok());
    let formatted = formatted.unwrap();
    assert_eq ! (formatted , "// input with some comments added to avoid reformatting\n\n(() => {\n  pipe(\n    // add a descriptive comment here\n    timelines,\n    everyCommitTimestamps,\n    A.sort(ordDate),\n    A.head,\n  );\n\n  pipe(\n    // add a descriptive comment here\n    serviceEventFromMessage(msg),\n    TE.chain(\n      flow(\n        // add a descriptive comment here\n        publishServiceEvent(analytics),\n        TE.mapLeft(nackFromError),\n      ),\n    ),\n  )()\n    .then(messageResponse(logger, msg))\n    .catch((err: Error) => {\n      logger.error(\n        pipe(\n          // add a descriptive comment here\n          O.fromNullable(err.stack),\n          O.getOrElse(constant(err.message)),\n        ),\n      );\n      process.exit(1);\n    });\n\n  pipe(\n    // add a descriptive comment here\n    Changelog.timestampOfFirstCommit([[commit]]),\n    O.toUndefined,\n  );\n\n  chain(\n    flow(\n      // add a descriptive comment here\n      getUploadUrl,\n      E.mapLeft(Errors.unknownError),\n      TE.fromEither,\n    ),\n  );\n})();");
}
