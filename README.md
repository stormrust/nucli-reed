
### nu-cli-reed

I have developed a nushell cli that depends on
[reedline](https://github.com/jonathandturner/reedline) instead of [rustyline](https://github.com/kkawakam/rustyline).

Every time a new release of nushell comes out I update the cli
code with the latest nu-cli crate and then remove all of the
dependencies on rustyline.

This code is packaged as a nu crate called **nu-cli-reed**.

The Cargo.toml file keys off the new released nushell crates
along with a code snapshot of an internal reedline crate.

Because a reedline crate is not yet published I reference my own
internal reedline crate.

### Integrate reedline into nushell

For the initial code we use this commit point:

### Working commit points

##### June 13, 2021

[7257797](https://github.com/jonathandturner/reedline/commit/725779728c078fa62ee7b16a6589ae4cc03ee44a)
