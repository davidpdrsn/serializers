# Serializers

[![Build Status](https://travis-ci.org/davidpdrsn/serializers.svg?branch=master)](https://travis-ci.org/davidpdrsn/serializers)
[![Crates.io](https://img.shields.io/crates/v/serializers.svg)](https://crates.io/crates/serializers)
[![Documentation](https://docs.rs/serializers/badge.svg)](https://docs.rs/serializers/)

Normally when using "serde_json" and `#[derive(Serialize)]` you only can have one JSON
representation for a type, however sometimes you might need another one which has more or less
data.

This crate makes it easy to create "serializers" that take some value and turn it into JSON.
You get to decide for each serializer which type it serializes, and which fields and
associations it includes.

## Install

```toml
[dependencies]
serializers = "0.2.0"
```

## Example

```rust
#[macro_use]
extern crate serializers;
#[macro_use]
extern crate serde_json;

use serializers::*;

struct User {
    id: u64,
    name: String,
    country: Country,
    friends: Vec<User>,
}

#[derive(Clone)]
struct Country {
    id: u64,
}

serializer! {
    #[derive(Debug, Copy, Clone)]
    struct UserSerializer<User> {
        attr(id)
        attr(name)
        has_one(country, CountrySerializer)
        has_many(friends, UserSerializer)
    }
}

serializer! {
    #[derive(Debug, Copy, Clone)]
    struct CountrySerializer<Country> {
        attr(id)
    }
}

fn main() {
    let denmark = Country {
        id: 1,
    };

    let bob = User {
        id: 1,
        name: "Bob".to_string(),
        country: denmark.clone(),
        friends: vec![
            User {
                id: 2,
                name: "Alice".to_string(),
                country: denmark.clone(),
                friends: vec![],
            }
        ],
    };

    let json: String = UserSerializer::serialize(&bob);

    assert_eq!(
        json,
        json!({
            "country": { "id": 1 },
            "friends": [
                {
                    "country": { "id": 1 },
                    "friends": [],
                    "name": "Alice",
                    "id": 2
                }
            ],
            "name": "Bob",
            "id": 1
        }).to_string(),
    );
}
```

See the [API docs](https://docs.rs/serializers/) for more info.
