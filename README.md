# Derefable
[![Linux build status](https://img.shields.io/travis/1aim/derefable.svg?branch=master)](https://travis-ci.org/1aim/derefable)
[![](https://img.shields.io/crates/d/derefable.svg)](https://crates.io/crates/derefable)
[![](https://tokei.rs/b1/github/1aim/derefable?category=code)](https://github.com/Aaronepower/tokei)
[![Documentation](https://docs.rs/derefable/badge.svg)](https://docs.rs/derefable/)

A procedural macro that allows you to derive `std::ops::Deref` and `std::ops::DerefMut` for
your structs. This macro can only be derived on structs **with atleast one field**. You can
specify which field you want to be deref'ed to with the `#[deref]` and allow mutable
dereferencing with `#[deref(mutable)]`.

## Deriving `std::ops::Deref`
```ignore
use std::collections::HashMap;

use derefable::Derefable;

#[derive(Default, Derefable)]
struct Map {
    #[deref]
    inner: HashMap<&'static str, &'static str>
}

fn main() {
    let map = Map::default();

    assert!(map.is_empty());
}
```

## Deriving `std::ops::DerefMut`
```ignore
use std::collections::HashMap;

use derefable::Derefable;

#[derive(Default, Derefable)]
struct MutableMap {
    #[deref(mutable)]
    inner: HashMap<&'static str, &'static str>
}

fn main() {
    let mut map = MutableMap::default();

    map.insert("Hello", "World");

    assert_eq!(map.get("Hello"), Some("World"));
}
```

