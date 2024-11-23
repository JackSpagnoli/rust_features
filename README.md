# Solution

Stack Overflow user [kmdreko](https://stackoverflow.com/users/2189130/kmdreko) provided a solution to this [problem](https://stackoverflow.com/questions/79216222/why-does-rust-include-dev-only-features-as-part-of-a-release-build). The solution was to specify `workspace.resolver = "2"` in the `Cargo.toml` file. The default resolver `"1"` was building `lib` with all features enabled and using this build for test and release builds. The new resolver `"2"` uses a distinct build for each target, resolving the issue.

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

I can't seem to get this to work with any combination of playing with default features, features, and dependencies. I'm not sure if I'm misunderstanding how features work, or if this is a bug.
