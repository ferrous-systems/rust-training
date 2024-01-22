<!-- markdownlint-disable MD031 MD033 -->
# Strategies for organizing memory in Rust applications

## Part 1: Working with Large long-lived mutating object graphs

## Two types of changes

* Topology changes
* Node content changes

## Naïve approach

* `Option<T>`
* `Cell<T>` and `RefCell<T>`

## `Option<T>`

* access requires `if let`
* operations via `Option` methods introduce closures with own scopes
  * cross-scope data access is hard
  * doesn't play well with `Result` and `?`

## Aside: Optional Chaining in Rust

Other languages have "optional chaining operator"

```rust ignore
fn perform_action() -> Result<..., ...> {
    // won't work, because the outer function returns Result, not Option
    let data: Option<...> = deeply.nested()?.lookup()?.of?.optional?.data();
}
```

## Aside: Optional Chaining in Rust, Recipe

by extracting lookup chain into a function that returns Option we enable `?` to act as an optional chaining operator in Rust

```rust ignore
fn deeply_nested_lookup(data: &Data) -> Option<NestedComponent> {
    // notice how we can mix and match method calls and field accesses
    deeply.nested()?.lookup()?.of?.optional?.data()
}

fn perform_action() -> Result<..., ...> {
    // now works
    let data: Option<...> = deeply_nested_lookup(&data);
}
```

## `Cell` and `RefCell`

* many `Cell` / `RefCell` methods can `panic!`
* `try_borrow_mut` gives a `Result`
  * how to handle `Err` case?
* harder to reason about mutability
    ```rust ignore
    fn does_this_fn_mutate_data_or_not(data: &Data) -> Result<...> {        //.
        if let Ok(item) = data.item.try_borrow_mut() {
            item.counter += 1;
        }
    }
    ```

## Alternatives to `Option+RefCell`

* strict hierarchy and common-root sharing
* entity-component systems
* query systems

## Strict Hierarchy and Common-Root Sharing

* represent your data as a tree
* prefer owned data
* no cross-references (parent, siblings, etc.)
* the root is owned by a top-level function and is mutable
  * `main`, `thread::spawn` closure, etc.

## Common-Root Sharing

when a mutation is performed and it needs mutable access to some sub-tree `A` and read-only access to other sub-trees `B` and `C`

* a common root for all subtrees `A`, `B`, and `C` is selected
* `&mut CommonRoot` is passed to the mutating function

## Common-Root Sharing: Pros

* functions work with plain Rust structures
  * no internal references in those structures
  * exclusive access
* all references to sub-components of the tree are created inside the function
  * straightforward borrow checker interactions
* mutability is declared in function signature

## Common-Root Sharing: Cons

* over-elevated access rights
  * when a function needs a mutable access to a small portion of the tree it may be receiving a mutable reference to much larger subtree
  * over-sharing can propagate up the function call chain and introduce access rights conflicts over time
* strict topology rules (no cross-references) limit applicability of the pattern
  * can provoke data over-cloning

## References without References

Handle

* an *id* of an object
* instead of storing references to objects store their ids

## Type-safe Handles

```rust ignore
struct Handle<T>(usize, PhantomData<T>);

// derive macro generates `impl<T: Clone> Clone for Handle<T>`
// that's why we use explicit implementations for Clone and Copy
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}
impl<T> Copy for Handle<T> {}

impl<T> From<usize> for Handle<T> {
    fn from(value: usize) -> Self {
        Handle(value, PhantomData)
    }
}
```

## Use of Type-Safe Handles

```rust ignore
struct User { id: Handle<User> }
struct Document { id: Handle<Document> }

struct Group { users: Vec<Handle<User>> }

fn main() {
    let user = User { id: Handle::from(1) };
    let document = Document { id: Handle::from(1) };
    let mut group = Group { users: vec![] };
    group.users.push(user.id);
    // fails with mismatch types
    // group.users.push(document.id);
}
```

## Handle Pros

* Structures holding handles don't hold references
  * no lifetimes involved
  * no cross-references within larger structures
    * can use common-roots sharing!
* In Rust typed handles prevent invalid associations

## Handle Cons

* Data access needs to perform handle lookups:
  * `all_users.get(user_handle)`
  * lookup failures have to be handles explicitly (`Option`)
* Object lifetime management can be complicated
  * explicit cleanups
  * reference counting
    * may accidentally introduce `Rc<RefCell<T>>` patterns even for simplest lookups

## Where the Data is Stored?

* Maps:
    ```rust ignore
    // same for other types
    let mut user_storage: HashMap<Handle<User>, User> = HashMap.new();
    // potentially need to provide multiple maps for operations
    invalidate_sessions(&mut session_storage, &mut user_storage, &cut_off_date);
    ```
    <p>&nbsp<!-- run-button placeholder --></p>

* "database" or "storage"
    ```rust ignore
    invalidate_sessions(&mut db, &cut_off_date);
    // presumable storage API
    db.get_mut::<Session>(&session_handle);
    ```
    <p>&nbsp<!-- run-button placeholder --></p>
    * associate multiple structs with the same handle (compartmentalization of mutability)

## TODO: Salsa

## TODO: ECS

## Part 2: Sharing Data between Threads

## Multithreading Influence on Data Design

* `thread::spawn` requires data to be `Send` and `'static`
* `Arc` vs `Rc`, `Mutex` vs `RefCell`
* async runtimes often run tasks on a thread pool
  * fragments of the same async function may be running on different threads
  * same `Send + Sync + 'static` requirements often apply

## Types of Application Memory

* short-lived single task
  * local variables
  * data transfer objects
* long-lived single task
  * web requests
  * database transactions
* long-lived, shared, and read-only
  * configuration
* **long-lived, shared between tasks**
  * user sessions
  * database connection pool

## TODO Long-lived Shared Mutable Data
