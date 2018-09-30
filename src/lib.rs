//! # Serializers
//!
//! Normally when using "serde_json" and `#[derive(Serialize)]` you only can have one JSON
//! representation for a type, however sometimes you might need another one which has more or less
//! data.
//!
//! This crate makes it easy to create "serializers" that take some value and turn it into JSON.
//! You get to decide for each serializer which type it serializes, and which fields and
//! associations it includes.
//!
//! ## Example
//!
//! ```
//! #[macro_use]
//! extern crate serializers;
//!
//! use serializers::*;
//!
//! struct User {
//!     id: u64,
//!     name: String,
//!     country: Country,
//!     friends: Vec<User>,
//! }
//!
//! #[derive(Clone)]
//! struct Country {
//!     id: u64,
//! }
//!
//! serializer! {
//!     serialize_user<User> {
//!         attr(id)
//!         attr(name)
//!         has_one(country, serialize_country)
//!         has_many(friends, serialize_user)
//!     }
//! }
//!
//! serializer! {
//!     serialize_country<Country> {
//!         attr(id)
//!     }
//! }
//!
//! fn main() {
//!     let denmark = Country {
//!         id: 1,
//!     };
//!
//!     let bob = User {
//!         id: 1,
//!         name: "Bob".to_string(),
//!         country: denmark.clone(),
//!         friends: vec![
//!             User {
//!                 id: 2,
//!                 name: "Alice".to_string(),
//!                 country: denmark.clone(),
//!                 friends: vec![],
//!             }
//!         ],
//!     };
//!
//!     let json = serialize_user.serialize(&bob);
//!
//!     assert_eq!(
//!         json,
//!         "{\"country\":{\"id\":1},\"friends\":[{\"country\":{\"id\":1},\"friends\":[],\"id\":2,\"name\":\"Alice\"}],\"id\":1,\"name\":\"Bob\"}"
//!     );
//! }
//! ```
//!
//! See the [macro docs](macro.serializer.html) for more information about its options.
//!
//! ## No macros for me
//!
//! The easiest way to define serializers is using the `serializer!` macro, however if you don't
//! wish to do so you can define serializers like so:
//!
//! ```
//! # #[macro_use]
//! # extern crate serializers;
//! # use serializers::*;
//! #
//! # struct User {
//! #     id: u64,
//! #     name: String,
//! #     country: Country,
//! #     friends: Vec<User>,
//! # }
//! #
//! # struct Country {
//! #     id: u64,
//! # }
//! #
//! # serializer! {
//! #     serialize_country<Country> {
//! #         attr(id)
//! #     }
//! # }
//! #
//! fn serialize_user(user: &User, b: &mut Builder) {
//!     b.attr("id", &user.id);
//!     b.attr("name", &user.name);
//!     b.has_one("country", &user.country, &serialize_country);
//!     b.has_many("friends", &user.friends, &serialize_user);
//! }
//! #
//! # fn main() {}
//! ```
//!
//! Any function with such a signature will automatically become a [`Serializer`](trait.Serializer.html).
//!
//! Using the serializer function afterwards works the same as if you used the macro.

#![deny(
    missing_docs,
    unused_imports,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![doc(html_root_url = "https://docs.rs/serializers/0.1.2")]

extern crate serde;
#[macro_use]
extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;

mod macros;

/// The trait you implement in order to make a serializer. The key-value pairs will be gathered in
/// the [`Builder`](struct.Builder.html) and turned into a JSON string by
/// [`ToJson`](trait.ToJson.html).
pub trait Serializer<T> {
    /// Add key-value pairs to the builder for the given object.
    ///
    /// You shouldn't have to call this method yourself. Instead you should go through
    /// [`ToJson`](trait.ToJson.html).
    fn serialize_into(&self, value: &T, j: &mut Builder);
}

impl<T, F> Serializer<T> for F
where
    F: Fn(&T, &mut Builder),
{
    fn serialize_into(&self, value: &T, b: &mut Builder) {
        self(&value, b);
    }
}

/// The struct responsible for gathering keys and values for the JSON.
///
/// This is the struct you interact with through the
/// [`serialize_into`](trait.Serializer.html#tymethod.serialize_into) method on the
/// [`Serializer`](trait.Serializer.html) trait.
#[derive(Debug)]
pub struct Builder {
    map: HashMap<String, Value>,
}

impl Builder {
    fn new() -> Self {
        Builder {
            map: HashMap::new(),
        }
    }

    fn to_value(&self) -> Value {
        json!(self.map)
    }

    /// Add a single key-value pair to the JSON.
    pub fn attr<K, V>(&mut self, key: K, value: &V) -> &mut Self
    where
        K: Into<String>,
        V: serde::Serialize,
    {
        let key: String = key.into();
        let value: Value = json!(value);
        self.map.insert(key, value);
        self
    }

    /// Add an object to the JSON. The associated value will be serialized using the given
    /// serializer.
    pub fn has_one<K, V, S>(&mut self, key: K, value: &V, serializer: &S) -> &mut Self
    where
        K: Into<String>,
        S: Serializer<V>,
    {
        let key: String = key.into();
        let value: Value = serializer.to_value(value);
        self.map.insert(key, value);
        self
    }

    /// Add an array to the JSON. Each item in the iterable will be serialized using the given
    /// serializer.
    pub fn has_many<'a, K, V: 'a, S, I>(&mut self, key: K, values: I, serializer: &S) -> &mut Self
    where
        K: Into<String>,
        S: Serializer<V>,
        I: IntoIterator<Item = &'a V>,
    {
        let key: String = key.into();
        let value = values
            .into_iter()
            .map(|v| serializer.to_value(&v))
            .collect::<Vec<_>>();
        self.map.insert(key, json!(value));
        self
    }
}

/// The trait responsible for actually compiling the JSON.
///
/// You shouldn't have to implement this trait manually. It will be automatically implemented for
/// anything that implements [`Serializer`](trait.Serializer.html).
pub trait ToJson<'a, T: 'a> {
    /// Turn the given object into a `serde_json::Value`.
    fn to_value(&self, value: &T) -> Value;

    /// Turn the given object into JSON.
    fn serialize(&self, value: &T) -> String {
        self.to_value(value).to_string()
    }

    /// Turn the given iterable into JSON array. The main usecase for this is to turn `Vec`s into
    /// JSON arrays.
    fn serialize_iter<I>(&self, values: I) -> String
    where
        I: IntoIterator<Item = &'a T>,
    {
        let acc: Vec<_> = values.into_iter().map(|v| self.to_value(&v)).collect();
        json!(acc).to_string()
    }
}

impl<'a, T: 'a, K> ToJson<'a, T> for K
where
    K: Serializer<T>,
{
    fn to_value(&self, value: &T) -> Value {
        let mut builder = Builder::new();
        self.serialize_into(value, &mut builder);
        builder.to_value()
    }
}
