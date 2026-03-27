# mdbook-stripelineno

We use code blocks like this in our slides:

````markdown

```rust no_run [1|2-3|4]
fn example() {}
```

````

This works for reveal.js but does not work for `mdbook`. We need to convert to this format

````markdown

```rust,no_run
fn example() {}
```

````

This is a simple mdbook pre-processor that can do that.
