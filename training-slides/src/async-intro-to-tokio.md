# Intro to Tokio

## What is Tokio

* Async runtime for Rust
* Provides async version of common I/O
* Provides network APIs and more

## More than one project

* Mio
* Tokio Runtime
* Hyper
* Tonic
* Tower

## Mio

* Metal I/O
* Lowest layer

## Example Mio

```rust [], ignore
let addr = "127.0.0.1:13265".parse()?;
let mut server = TcpListener::bind(addr)?;
// Start listening for incoming connections.
poll.registry()
    .register(&mut server, SERVER, Interest::READABLE)?;

// Setup the client socket.
let mut client = TcpStream::connect(addr)?;
// Register the socket.
poll.registry()
    .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;
```

## Tokio Runtime

* Foundational API
* Async version of common std I/O commands
* Efficient executor for async tasks

## Example Tokio

```rust [], ignore
#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}
```

## Hyper

* HTTP Client and Server APIs
* Support HTTP/1 and HTTP/2

## Example Hyper

```rust [], ignore
#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
```

## Others

* Tonic: gRPC Client/Server library
* Tower: Modular server components (retry, load-balance, etc)
* Tracing: Structured tracing and data-collection
* Bytes: Network byte manipulation
