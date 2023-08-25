![Our cute logo!](images/logo_ferrous-systems_rgb.png)

# Start Here

Make sure to have `rustup`, `VSCode` and the `rust-analyzer` VSCode extension installed.

Our various Rust training courses fit together as follows:

```mermaid
graph TD;
    Fundamentals(Rust Fundamentals)-->Applied(Applied Rust);
    Applied-->Advanced(Advanced Rust);
    Applied-->NoStd(No-Std Rust);
    NoStd-->Ferrocene(Using Ferrocene);
    NoStd-->BareMetal(Bare-Metal Rust);
    Applied-->Async(Async Rust);
    BareMetal-->Embassy(Using Embassy);
    Async-->Embassy;
    Why(Why Rust?);
```

* **Rust Fundamentals**: Covers the basics - types, writing functions, using iterators.
* **Applied Rust**: Using Rust with Windows, macOS or Linux.
* **Advanced Rust**: Deep-dives into specific topics.
* **Async Rust**: Futures, Polling, Tokio, and all that jazz.
* **Ferrocene**: Working with our qualfied toolchain
* **No-Std Rust**: Rust without the Standard Library.
* **Bare-Metal Rust**: Rust on a micro-controller.
* **Using Embassy**: Async-Rust on a micro-controller.
* **Why Rust?**: A (stand-alone) half-day tour of Rust for decision-makers, technical leads and managers.

