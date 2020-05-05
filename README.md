`cargo test` in this repo passes on Linux but fails on Unix. In both
cases, the test is construcing a command line arg (`OsString`) that
contains invalid Unicode and passing it to a Clap binary. Something
about Clap v2.33.0 on Windows causes an "unexpected invalid UTF-8 code
point" error here, when we should not be enforcing UTF-8.
