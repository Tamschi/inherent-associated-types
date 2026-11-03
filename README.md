# inherent-associated-types

Emulates inherent associated types in stable Rust\
(in a way that looks clean in both code and documentation).

```rust
use inherent_associated_types::{Associated, associate};

struct Host;
struct A<T>(T);
struct B;

#[associate]
impl Host {
    pub type HostA = A<()>;
    pub type HostB = B;
}

#[associate]
impl<T> A<T> {
    /// You can document and use attributes.
    pub type AA = A<T>;
    pub type AB = B;
}

const _: Associated!(Host) = Host;
const _: Associated!(Host[HostB]) = B;
const _: Associated!(Host[HostA][AA][AB]) = B;
const _: Associated!(Associated!(Host[HostA])[AB]) = B;
```

## Features

- Appears cleanly in documentation!

  Associated types are rendered as e.g. `impl<T> Associated<AA> for A<T>`.\
  (See crate documentation for a formatting example.)

- 100% hygienic, and can be used as renamed or imported dependency via `#[associate(crate = …)]`.

## Limitations

- Associated types must be (as) public (as their host).

- Each associated type identifier is limited to about 80 Latin or 26 CJK characters.

- [As in the RFC](https://github.com/rust-lang/rfcs/blob/3bdcd22feea6848a7920fa06d17ad4c6ada9db89/text/0195-associated-items.md#inherent-associated-items),
  the (presence of an) inherent associated type can't <!-- sensibly --> be used as bound.

## Known issues

- The errors in response to invalid macro input are a little unpolished,\
  and currently they stop the macro outright instead of recovering where possible.
- The error reported for missing associated types shatters the illusion (and can be a bit long).\
  Similarly, identifiers longer than 16 bytes in UTF-8 cause spacers to appear in the documentation.
- Generic associated types (GATs) are currently not supported.

## Licensing

Both *inherent-associated-types* and *inherent-associated-types_proc-macro-definitions* are licensed under the EUPL v. 1.2.

## Versioning

Feature and patch versions are synchronised across incompatible branches and not reset[^1], except for v1.0.0. Branches may skip irrelevant releases.

Release-candidate versions are suffixed `-RC.n` where `n` is a positive integer.

[^1]: This is so you can determine feature and patch levels at a glance for my projects.
