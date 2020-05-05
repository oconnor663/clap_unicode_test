[![Actions Status](https://github.com/oconnor663/clap_unicode_test/workflows/tests/badge.svg)](https://github.com/oconnor663/clap_unicode_test/actions)

`cargo test` in this repo passes on Linux but fails on Unix. In both
cases, the test is construcing a command line arg (`OsString`) that
contains invalid Unicode and passing it to a Clap binary. Something
about Clap v2.33.0 on Windows causes an "unexpected invalid UTF-8 code
point" error here, when we should not be enforcing UTF-8.

See https://github.com/clap-rs/clap/issues/1905.
