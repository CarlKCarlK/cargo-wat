# Nine Rust Cargo Wats

In JavaScript and other languages, we call a surpising behavior or inconsistany a Wat! [that is, a "What!?"](ref cmk). For example, in JavaScript, an empty array plus an empty array produces an empty string, `[] +[] === ""`.

Rust is generally more consistant than JavaScript. Some Rust-related formats, however, offer surprises. Specificaally, this article will look at nine Wats in Rust's `Cargo.toml`.

Recall that `Cargo.toml` is the is the manifest file that defines the configuration and dependencies of a Rust project. TOML (Tom's Obvious, Minimal Language) is a configuration file format that represents nested key/value pairs and/or arrays. Unlike JSON, a similar format, TOML is designed for easy reading and writing by humans.

This journey of the Nine Wats may be (at least mildly) entertaining. Also, if you've ever found the `Cargo.toml`'s format confusing, it may help you feel better about
yourself. Finally and most importantly, when you learn the Nine Wats, you will be able to write your `Cargo.toml` more easily and effectively.

This list is not about "fixing" `Cargo.toml`. It's already great at its main purpose: specifying the configuration and dependencies of a Rust project. Instead, this list is about understanding the format and its quirks.

## Wat 1: Dependancies and Profile Section Names

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

Indeed, if you want forgo all section notation in the file and use only dots:

```toml
profile.release.opt-level = 3
```

or, no section notation and no dots:

```toml
profile = { release = { opt-level = 3 } }
```

## Wat 2: Inheritance

You might argue that dots are fine for profiles, but hyphens are better for
dependencies because `[dev-dependencies]` inherits from `[dependencies]`. In other words, the dependencies in `[dependencies]` are also available in `[dev-dependencies]`.

So, does this mean that `[build-dependencies]` inherits from `[dependencies]`?

No! `[build-dependencies]` does not inherit from `[dependencies]`. Wat?

## Wat 3: Default Keys

You likely know that instead of this:

```toml
[dependencies]
serde = { version = "1.0" }
```

you can write this:

```toml
[dependencies]
serde = "1.0"
```

What's the principle here? How in TOML do you designate one key as the default key?

You can't! TOML has no default keys. Wat?

Cargo TOML does special processing on the `version` key in the `[dependencies]` section. This is a Cargo-specific feature, not a TOML feature. As far as I can tell, Cargo TOML offers no other default keys.

## Wat 4: Lists

TOML offers two syntaxes for lists:

```toml
a = [{b = 1}, {b = 2}]
```

or

```toml
[[a]]
b = 1
[[a]]
b = 2
```

This is legal in Cargo TOML:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive", "std"] }
```

So, is this?

```toml
[dependencies]
serde.version = "1.0"
[[serde.features]]
derive = true
[[serde.features]]
std = true
```

or

2 Target.TARGET can only prefix dependency related sections
3 'cfg(…)' expressions allowed only in TARGET
'cfg(…)’ expressions not required in target.TARGET
4 Full target name also allowed
5 TOML has no default keys. Cargo TOML has ‘version’
6 TOML offers two syntaxes for lists. Cargo TOML does too but always requires one or the other.
[[bin]]
7 features = [“skdfsd”,
8 You can subfeatures two ways, but the inline way isn’t TOML’s inline syntax. Related to features that require features
ot

```

```

```

```
