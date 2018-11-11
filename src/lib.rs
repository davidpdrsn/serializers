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
//! #[macro_use]
//! extern crate serde_json;
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
//!     #[derive(Debug)]
//!     struct UserSerializer<User> {
//!         attr(id)
//!         attr(name)
//!         has_one(country, CountrySerializer)
//!         has_many(friends, UserSerializer)
//!     }
//! }
//!
//! serializer! {
//!     #[derive(Debug)]
//!     struct CountrySerializer<Country> {
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
//!     // Serializing a single user
//!     let json: String = UserSerializer::serialize(&bob);
//!     assert_eq!(
//!         json,
//!         json!({
//!             "country": { "id": 1 },
//!             "friends": [
//!                 {
//!                     "country": { "id": 1 },
//!                     "friends": [],
//!                     "name": "Alice",
//!                     "id": 2
//!                 }
//!             ],
//!             "name": "Bob",
//!             "id": 1
//!         }).to_string(),
//!     );
//!
//!     // Serializing a vector of users
//!     let users = vec![bob];
//!     let json: String = UserSerializer::serialize_iter(&users);
//!     assert_eq!(
//!         json,
//!         json!([
//!             {
//!                 "country": { "id": 1 },
//!                 "friends": [
//!                     {
//!                         "country": { "id": 1 },
//!                         "friends": [],
//!                         "name": "Alice",
//!                         "id": 2
//!                     }
//!                 ],
//!                 "name": "Bob",
//!                 "id": 1
//!             }
//!         ]).to_string(),
//!     );
//! }
//! ```
//!
//! See the [macro docs](macro.serializer.html) for more information about the `serializer!` macro.
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
//! #     struct CountrySerializer<Country> {
//! #         attr(id)
//! #     }
//! # }
//! #
//! struct UserSerializer;
//!
//! impl Serializer<User> for UserSerializer {
//!     fn serialize_into(&self, user: &User, b: &mut Builder) {
//!         b.attr("id", &user.id);
//!         b.attr("name", &user.name);
//!         b.has_one("country", &user.country, &CountrySerializer);
//!         b.has_many("friends", &user.friends, &UserSerializer);
//!     }
//! }
//! #
//! # fn main() {}
//! ```

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

use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

mod macros;

/// The trait you implement in order to make a serializer.
pub trait Serializer<T> {
    /// Add key-value pairs to the builder for the given object.
    ///
    /// You shouldn't have to call this method yourself. It'll be called by other method in this
    /// trait.
    fn serialize_into(&self, value: &T, builder: &mut Builder);

    /// Turn the given object into a `serde_json::Value`.
    fn to_value(&self, value: &T) -> Value {
        let mut builder = Builder::new();
        self.serialize_into(value, &mut builder);
        builder.to_value()
    }

    /// Turn the given object into a JSON string.
    fn serialize(&self, value: &T) -> String {
        self.to_value(value).to_string()
    }

    /// Turn the given iterable into JSON array. The main usecase for this is to turn `Vec`s into
    /// JSON arrays, but works for any iterator.
    fn serialize_iter<'a, I>(&self, values: I) -> String
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a,
    {
        let acc: Vec<_> = values.into_iter().map(|v| self.to_value(&v)).collect();
        json!(acc).to_string()
    }
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
        V: Serialize,
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_user_serializer {
        {
            $($tokens:tt)*
        } => {
            struct User {
                id: u64,
            }

            serializer! {
                $($tokens)*
            }

            let bob = User { id: 1 };
            let json: String = UserSerializer::serialize(&bob);
            assert_eq!(json, json!({ "id": 1 }).to_string());
        };
    }

    #[test]
    fn test_pub_crate() {
        test_user_serializer! {
            pub(crate) struct UserSerializer<User> { attr(id) }
        };
    }

    #[test]
    fn test_pub() {
        test_user_serializer! {
            pub struct UserSerializer<User> { attr(id) }
        };
    }

    #[test]
    fn test_private() {
        test_user_serializer! {
            struct UserSerializer<User> { attr(id) }
        };
    }

    #[test]
    fn test_pub_crate_attrs() {
        test_user_serializer! {
            #[derive(PartialEq, Eq, Debug)]
            pub(crate) struct UserSerializer<User> { attr(id) }
        };
        assert_eq!(UserSerializer, UserSerializer);
    }

    #[test]
    fn test_pub_attrs() {
        test_user_serializer! {
            #[derive(PartialEq, Eq, Debug)]
            pub struct UserSerializer<User> { attr(id) }
        };
        assert_eq!(UserSerializer, UserSerializer);
    }

    #[test]
    fn test_private_attrs() {
        test_user_serializer! {
            #[derive(PartialEq, Eq, Debug)]
            struct UserSerializer<User> { attr(id) }
        };
        assert_eq!(UserSerializer, UserSerializer);
    }

    #[test]
    fn generated_associated_function() {
        struct User {
            id: u64,
        }

        serializer! {
            struct UserSerializer<User> {
                attr(id)
            }
        }

        let bob = User { id: 1 };
        let json: String = UserSerializer::serialize(&bob);
        assert_eq!(json, json!({ "id": 1 }).to_string());
    }
}
