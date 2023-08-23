# Rust I/O Traits

## There are two kinds of computer:

* Windows NT based
* POSIX based (macOS, Linux, QNX, etc)

Rust supports both.

Note:

We're specifically talking about `libstd` targets here. Targets that only have
`libcore` have very little I/O support built-in - it's all third party crates.

## They are very different:

```c
HANDLE CreateFileW(
  /* [in]           */ LPCWSTR               lpFileName,
  /* [in]           */ DWORD                 dwDesiredAccess,
  /* [in]           */ DWORD                 dwShareMode,
  /* [in, optional] */ LPSECURITY_ATTRIBUTES lpSecurityAttributes,
  /* [in]           */ DWORD                 dwCreationDisposition,
  /* [in]           */ DWORD                 dwFlagsAndAttributes,
  /* [in, optional] */ HANDLE                hTemplateFile
);

int open(const char *pathname, int flags, mode_t mode);
```

## Abstractions

To provide a common API, Rust offers some basic abstractions:

* A [`Read`](https://doc.rust-lang.org/std/io/trait.Read.html) trait for reading bytes
* A [`Write`](https://doc.rust-lang.org/std/io/trait.Write.html) trait for writing bytes
* Buffered wrappers for the above ([`BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html) and [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html))
* A [`Seek`](https://doc.rust-lang.org/std/io/trait.Seek.html) trait for adjusting the read/write offset in a file, etc
* A [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) type to represent open files
* Types for [`Stdin`](https://doc.rust-lang.org/std/io/struct.Stdin.html), [`Stdout`](https://doc.rust-lang.org/std/io/struct.Stdout.html) and [`Stderr`](https://doc.rust-lang.org/std/io/struct.Stderr.html)
* The [`Cursor`](https://doc.rust-lang.org/std/io/struct.Cursor.html) type to make a `[u8]` readable/writable

## The Read Trait

<https://doc.rust-lang.org/std/io/trait.Read.html>

```rust []
use std::io::Result;

pub trait Read {
    // One required method
    fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
    // Lots of provided methods, such as:
    fn read_to_string(&mut self, buf: &mut String) -> Result<usize> { todo!() }
}
```

## Immutable Files

* A `File` on POSIX is just an integer (recall `open` returns an `int`)
* Do you need a `&mut File` to write?
  * No - the OS handles shared mutability internally
* But the trait requires `&mut self`...

## Implementing Traits on `&Type`

```rust ignore
impl Read for File {

}

impl Read for &File {

}
```

See the [std::io::File docs](https://doc.rust-lang.org/std/fs/struct.File.html#impl-Read-for-%26File).

## OS Syscalls

* Remember, Rust is *explicit*
* If you ask to read 8 bytes, Rust will ask the OS to get 8 bytes from the device
* Asking the OS for anything is expensive!
* Asking the OS for a million small things is *really expensive*...

## Buffered Readers

* There is a [`BufRead` *trait*](https://doc.rust-lang.org/std/io/trait.BufRead.html), for *buffered I/O devices*
* There is a [`BufReader` *struct*](https://doc.rust-lang.org/std/io/struct.BufReader.html)
  * Owns a `R: Read`, and `impl BufRead`
  * Has a buffer in RAM and reads in large-ish chunks

```rust []
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let f = std::fs::File::open("/etc/hosts")?;
    let reader = std::io::BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
```

## The `write!` macro

* You can `println!` to standard output
* You can `format!` to a `String`
* You can also `write!` to any `T: std::io::Write`

```rust
use std::io::Write;

fn main() -> std::io::Result<()> {
    let filling = "Cheese and Jam";
    let f = std::fs::File::create("lunch.txt")?;
    write!(&f, "I have {filling} sandwiches")?;
    Ok(())
}
```

## Networking

* In Rust, a [`TcpStream`](https://doc.rust-lang.org/std/io/struct.TcpStream.html) also implements the `Read` and `Write` traits.
* You create a `TcpStream` with either:
  * [`TcpStream::connect`](https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect) - for outbound connections
  * [`TcpListener::accept`](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept) - for incoming connections
  * [`TcpListener::incoming`](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming) - an iterator over incoming connections
* As before, you might want to wrap your `TcpStream` in a `BufReader`

## End of the Line

* It's obvious when you've hit the end of a `File`
* When do you hit the end of a `TcpStream`?
  * When either side does a `shutdown`

Note:

* `Read` trait has a method `read_to_end()`

## Binding Ports

* `TcpListener` needs to know which IP address and port to bind
* Rust has a [`ToSocketAddrs`](https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html) trait impl'd on *many things*
  * `&str`, `(IpAddr, u16)`, `(&str, u16)`, etc
* It does DNS lookups automatically (which may return multiple addresses...)

```rust
fn main() -> Result<(), std::io::Error> {
    let listener = std::net::TcpListener::bind("127.0.0.1:7878")?;
    Ok(())
}
```

## More Networking

* There is also [`std::net::UdpSocket`](https://doc.rust-lang.org/std/net/struct.UdpSocket.html)
* [`IpAddr`](https://doc.rust-lang.org/std/net/enum.IpAddr.html) is an enum of [`Ipv4Addr`](https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html) and [`Ipv6Addr`](https://doc.rust-lang.org/std/net/struct.Ipv6Addr.html)
* [`SocketAddr`](https://doc.rust-lang.org/std/net/enum.SocketAddr.html) is an enum of [`SocketAddrV4`](https://doc.rust-lang.org/std/net/struct.SocketAddrV4.html) and [`SocketAddrV6`](https://doc.rust-lang.org/std/net/struct.SocketAddrV6.html)
* But TLS, HTTP and QUIC are all third-party crates

Note:

Some current prominent examples of each -

* TLS - [RusTLS](https://github.com/rustls/rustls)
* HTTP - [hyperium/http](https://github.com/hyperium/http)
* QUIC - [cloudflare/quiche](https://github.com/cloudflare/quiche)

## Failures

* Almost any I/O operation can fail
* Almost all `std::io` APIs return `Result<T, std::io::Error>`
* [`std::io::Result<T>`](https://doc.rust-lang.org/std/io/type.Result.html) is an alias
* Watch out for it [in the docs](https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind)!
