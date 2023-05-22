# Property Testing

## This is your brain

-   Everything we know is subject to bias
-   Everything we build reflects these biases

## Problem:

Our code reflects our biases, our tests are often biased similarly

## Solution:

Don't write tests

## Solution:

Write expectations

---

-   Have the machine generate random test cases
-   Make beliefs explicit, force them to pay rent

---

This is called property testing

## Crate: **proptest**

```rust ignore
// this property is false, but perhaps
// not unreasonable to expect to be true
proptest! {
  #[test]
  fn mult_and_div(ref a in any::<usize>()) {
    let result = (a * 5) / 5;
    assert_eq!(result, a);
  }
}
```

## Crate: **proptest**

```console
$ cargo test
test mult_and_div ... FAILED
Test failed: attempt to multiply with overflow;
minimal failing input: ref a = 3689348814741910324
test result: FAILED. 0 passed; 1 failed
```

## Crate: **proptest**

```console
$ cat proptest-regressions/main.txt
 # Seeds for failure cases proptest has
 # generated. It is automatically read
 # and these particular cases re-run before
 # any novel cases are generated.
 # shrinks to ref a = 3689348814741910324
 xs 4050946508 1278147119 4151624343 875310407
```

---

Wonderful for testing codecs, serialization, compression, or any set of operations that should retain equality.

```rust ignore
proptest! {
  #[test]
  fn compress_roundtrip(ref s in ".*") {
    let result = decompress(compress(s));
    assert_eq!(result, s);
  }
}
```

---

It's easy to generate more structured input, too

```rust ignore
proptest! {
  #[test]
  fn parses_all_valid_dates(
    ref s in "[0-9]{4}-[0-9]{2}-[0-9]{2}"
  ) {
    parse_date(s).unwrap();
  }
}
```

## Configuration is a great target

```rust ignore
proptest! {
  #[test]
  fn doesnt_crash(
    bit in 0usize..1_000_000,
    page_sz_exponent in 0usize..30
  ) {
    let page_sz = 1 << page_sz_exponent;
    let mut bits = Bitfield::new(page_sz);
    assert_eq!(bits.set(bit, true), Change::Changed);
    assert_eq!(bits.get(bit), true);
  }
}
```

## Miscellaneous Tips

-   Isolate business logic from IO concerns
-   Use `assert!` and `debug_assert!` on non-trivial things! this makes our "fuzzers" extremely effective
-   Try not to use `unwrap()` everywhere, at least use `expect("helpful message")` to speed up debugging
-   When propagating errors, include context that helps you get back to the root
