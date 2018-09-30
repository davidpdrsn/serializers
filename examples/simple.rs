#[macro_use]
extern crate serializers;

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
    serialize_user<User> {
        attr(id)
        attr(name)
        has_one(country, serialize_country)
        has_many(friends, serialize_user)
    }
}

serializer! {
    serialize_country<Country> {
        attr(id)
    }
}

fn main() {
    let denmark = Country { id: 1 };

    let bob = User {
        id: 1,
        name: "Bob".to_string(),
        country: denmark.clone(),
        friends: vec![User {
            id: 2,
            name: "Alice".to_string(),
            country: denmark.clone(),
            friends: vec![],
        }],
    };

    let json = serialize_user.serialize(&bob);

    assert_eq!(
        json,
        "{\"country\":{\"id\":1},\"friends\":[{\"country\":{\"id\":1},\"friends\":[],\"id\":2,\"name\":\"Alice\"}],\"id\":1,\"name\":\"Bob\"}"
    );

    println!("{}", json);
}
