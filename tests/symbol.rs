use typy::symbol::{Interner, SymbolId};

#[test]
fn interns_the_same_string_with_the_same_symbol() {
    let mut interner = Interner::new();

    let first = interner.intern("foo");
    let second = interner.intern("foo");

    assert_eq!(first, second);
}

#[test]
fn interns_different_strings_with_different_symbols() {
    let mut interner = Interner::new();

    let foo = interner.intern("foo");
    let bar = interner.intern("bar");

    assert_ne!(foo, bar);
}

#[test]
fn resolves_interned_symbols() {
    let mut interner = Interner::new();

    let foo = interner.intern("foo");
    let bar = interner.intern("bar");

    assert_eq!(interner.resolve(foo), "foo");
    assert_eq!(interner.resolve(bar), "bar");
}

#[test]
fn get_returns_none_for_unknown_symbol() {
    let interner = Interner::new();

    assert_eq!(interner.get(SymbolId(0)), None);
}

#[test]
#[should_panic(expected = "attempted to resolve an invalid SymbolId")]
fn resolve_panics_for_unknown_symbol() {
    let interner = Interner::new();

    let _ = interner.resolve(SymbolId(0));
}
