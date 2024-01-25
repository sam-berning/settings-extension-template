# settings-extension-template

A template for [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) that generates a
settings extension for [Bottlerocket](https://github.com/bottlerocket-os/bottlerocket).

## How to use

After installing `cargo-generate`, simply run:

```
cargo generate sam-berning/settings-extension-template
```

This will create a new local git repo with the structure of a basic settings extension.

When asked for the setting name, it will be the top-level namespace for any settings you provide, e.g.
`motd`, `kernel`, etc. For examples of existing settings, see the
[Bottlerocket website](https://bottlerocket.dev/en/os/1.18.x/api/settings/).
