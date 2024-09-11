# Testing

---

Testing is fundamental to Rust.

Unit, integration, and documentation tests all come built-in.

## Organizing Tests

Tests typically end up in 1 of 4 possible locations:

-   Immediately beside the functionality tested (Unit Tests)
-   In a `tests` submodule (Unit Tests)
-   In documentation. (Documentation Test)
-   In the `tests/` directory. (Integration Tests)

## Unit Tests

-   Allows testing functionality in the same module and environment.
-   Typically exist immediately near the functionality.
-   Good for testing to make sure a single action *works*.

## Unit Tests

-   Allows testing as if the functionality is being used elsewhere in the project.
-   For testing private APIs and functionality.
-   Good for testing expected processes and use cases.

## `tests` Submodule

```rust []
enum Direction { North, South, East, West }

fn is_north(dir: Direction) -> bool {
    match dir {
        Direction::North => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn is_north_works() {
        assert!(is_north(Direction::North) == true);
        assert!(is_north(Direction::South) == false);
    }
}
```

## `tests` Submodule

```console
$ cargo test
running 1 test
test tests::is_north_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Documentation Tests

-   Allows testing public functionality.
-   Is displayed in `rustdoc` output.
-   For demonstrating expected use cases and examples.

## Documentation Tests

```rust []
/// ```rust
/// use example::Direction;
/// let way_home = Direction::North;
/// ```
pub enum Direction { North, South, East, West }
```

## Documentation Tests

```console
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests example

running 1 test
test Direction_0 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```

## Integration Tests

-   Tests as if the crate is an external dependency.
-   Intended for longer or full-function tests.

## Integration Tests

.`/tests/basic.rs`
```rust ignore []
use example::{is_north, Direction};

#[test]
fn is_north_works() {
    assert!(is_north(Direction::North) == true);
    assert!(is_north(Direction::South) == false);
}
```

## Integration Tests

```console
$ cargo test
running 1 test
test is_north_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/deps/example-9f39afa5d2a1c6bf

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests example

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
