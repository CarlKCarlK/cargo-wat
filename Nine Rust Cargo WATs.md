# Nine Rust Cargo Wats

In JavaScript and other languages, we call a surprising behavior or inconsistency a Wat! [that is, a "What!?"](ref cmk). For example, in JavaScript, an empty array plus an empty array produces an empty string, `[] +[] === ""`.

Rust is generally more consistent than JavaScript. Some Rust-related formats, however, offer surprises. Specifically, this article will look at nine Wats in Rust's `Cargo.toml`.

Recall that `Cargo.toml` is the is the manifest file that defines the configuration and dependencies of a Rust project. TOML (Tom's Obvious, Minimal Language) is a configuration file format that represents nested key/value pairs and/or arrays. Unlike JSON, a similar format, TOML is designed for easy reading and writing by humans.

This journey of the Nine Wats may be (at least mildly) entertaining. Also, if you've ever found the `Cargo.toml`'s format confusing, it may help you feel better about
yourself. Finally and most importantly, when you learn the Nine Wats, you will be able to write your `Cargo.toml` more easily and effectively.

This list is not about "fixing" `Cargo.toml`. It's already great at its main purpose: specifying the configuration and dependencies of a Rust project. Instead, this list is about understanding the format and its quirks.

## Wat 1: Depenancies and Profile Section Names

You probably know how to use add a `[dependencies]` section to your `Cargo.toml`. It specifies release dependencies, for example:

```toml
[dependencies]
serde = "1.0"
```

Along the sames lines, you can specify development dependencies with a `[dev-dependencies]` and build dependencies with a `[build-dependencies]` section.

You may also need to set compiler options for release, development, and build. You do this with profile sections, for example, optimization level and whether to include debugging information.

Guess the names of these three sections.

Is it `[profile]`, `[dev-profile]` and `[build-profile]`?

No! it's `[profile.release]`, `[profile.dev]`, and `[profile.build]`. Wat?

Which names are better? I like profile names better in part because the dots in the section names are useful.

In TOML dots are used to separate keys in nested tables. For example, `a.b.c` is a key `c` in a table `b` in a table `a`. This gives us two ways to, for example, set the optimization level for release:

```toml
[profile.release]
opt-level = 3
```

or

```toml
[profile]
release.opt-level = 3
```

Similary, for release, develepment, and build you can specify profiles. A profile sets options for, for example, optimization level and whether to include debugging information.

A given profile is a set of configuration options. For example, you can specify the optimization level and whether to include debugging information.

Guess th

A profile is a set of configuration options that can be used to customize the compilation process. For example optimization level
The two most common profiles are `release` and `dev`. You probably know that you use `[profile.release]` to specify the release profile.

Profiles sections are for

But did you know that you use `[profile.dev]` to specify the development profile? This inconsistency can be confusing.

But did you know that you use `[profile.dev]` to specify the development profile? This inconsistency can be confusing.
how to set your release and development de

Inconsistent Target and Profile Names
Dependencies:
toml
[dependencies] serde = "1.0" [dev-dependencies] anyhow = "1.0"
Profiles:
toml
[profile.dev] opt-level = 0 debug = true
WAT: Dependencies use hyphens while profiles use dot notation, leading to inconsistency.
Target.TARGET can only prefix dependency related sections
'cfg(…)' expressions allowed only in TARGET
'cfg(…)’ expressions not required in target.TARGET
Full target name also allowedw
TOML allows top-level in-line keys. Cargo TOML doen’t
TOML has no default keys. Cargo TOML has ‘version’
TOML offers two syntaxes for lists. Cargo TOML does too but always requires one or the other.
[[bin]]
features = [“skdfsd”,
TOML allows two syntaxes for tables. Cargo TOML does too but always requires one or the other expect in one place.
You can subfeatures two ways, but the inline way isn’t TOML’s inline syntax
Related to features that require features
