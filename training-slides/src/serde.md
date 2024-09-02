# `serde`

## **Ser**ialization and **De**serialization

<https://serde.rs>

## `Serialize` & `Deserialize`

To make a Rust structure (de)serializable:

```rust ignore []
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum Direction { North, South, East, West }
```

## Formats

Serde supports a number of formats, such as:

* JSON
* CBOR
* YAML
* TOML
* BSON
* MessagePack
* ... More!

Did you enjoy that acronym salad?

## `Serialize`

To JSON:

```rust ignore []
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }

fn main() {
    let action = Move { id: 1, direction: West };
    let payload = serde_json::to_string(&action);
    println!("{:?}", payload);
}
```

## `Deserialize`

From JSON:

```rust ignore []
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }

fn main() {
    let payload = r#"{ "id": 1, "direction": "West" }"#;
    let action = serde_json::from_str::<Move>(&payload);
    println!("{:?}", action);
}
```

## Transcode

```rust ignore []
use serde::{Serialize, Deserialize};
use serde_transcode::transcode;

#[derive(Debug, Serialize, Deserialize)]
struct Move {
    id: usize,
    direction: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
enum Direction { North, South, East, West }

fn main() {
    let payload = r#"{ "id": 1, "direction": "West" }"#;
    let mut buffer = String::new();
    {
        let mut ser = toml::Serializer::new(&mut buffer);
        let mut de = serde_json::Deserializer::from_str(&payload);
        transcode(&mut de, &mut ser)
            .unwrap();
    }
    println!("{:?}", buffer);
}
```

## Attributes

`serde` has a large number of attributes you can utilize:

```rust ignore []
#[serde(deny_unknown_fields)] // Be extra strict
struct Move {
    #[serde(default)] // Call usize::default()
    id: usize,
    #[serde(rename = "dir")] // Use a different name
    direction: Direction,
}
```

<https://serde.rs/attributes.html>
