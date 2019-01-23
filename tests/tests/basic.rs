use derefable_derive::Derefable;

#[derive(Derefable)]
struct Foo {
    #[deref]
    bar: u32,
}

#[test]
fn basic() {
    let foo = Foo { bar: 5 };

    assert_eq!(*foo, 5);
}

#[derive(Derefable)]
struct Tuple(#[deref] u32);

#[test]
fn tuple() {
    let foo = Tuple(5);

    assert_eq!(*foo, 5);
}

#[derive(Derefable)]
struct Mut {
    #[deref(mutable)]
    num: u32,
}

#[test]
fn deref_mut() {
    let mut foo = Mut { num: 5 };

    *foo = 10;

    assert_eq!(*foo, 10);
}
