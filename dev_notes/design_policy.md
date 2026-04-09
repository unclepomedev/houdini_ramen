## Implementations that go against theory, etc.

* fail-fast: Instead of allowing users to handle errors flexibly, cause a panic immediately (`Result` is not returned)
    * Typically, instead of returning Result as convention, use `panic!` to immediately halt.
    * However, for errors that are only discovered when Python is executed, an explicit error message should be returned by runtime error handling.
