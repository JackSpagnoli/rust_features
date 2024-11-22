# Something I'm confused by

`lib` has a function that normally returns `"bar"`, but returns `"foo"` if run as a test, or using the feature `return_foo`.

`bin` uses `lib` as a dependency, and uses `lib` with the `return_foo` feature as a dev dependency.

# Expected

`assert_eq!(some_function(), "foo".to_string());` called from a test inside `lib` will pass.

`assert_eq!(some_function(), "foo".to_string());` called from a test inside `bin` will pass.

`assert_eq!(some_function(), "bar".to_string());` called from main inside `bin` will pass.

# Actual

`assert_eq!(some_function(), "foo".to_string());` called from a test inside `lib` will pass.

`assert_eq!(some_function(), "foo".to_string());` called from a test inside `bin` will pass.

`assert_eq!(some_function(), "bar".to_string());` called from main inside `bin` will fail.

I would expect with a `--release` flag set on the build that features exclusive to dev dependencies will not be included, but it looks like they are.
