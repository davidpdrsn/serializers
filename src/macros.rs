/// This macro is the primary way to make serializers. See the [top level docs](index.html) for an
/// example.
///
/// It expands into a struct the implements [`Serializer`](trait.Serializer.html).
///
/// ## Customization
///
/// The macro also lets you set separate JSON keys and field names. That is done by adding an
/// additional first argument to `attr`, `has_one`, or `has_many` which will be the key.
///
/// You can also write `pub` in front of the name of your strcut to make it public. Additionally you can write `pub(crate)` to make it public within your crate.
///
/// You can also add `#[derive(...)]` above the struct definition.
///
/// Example:
///
/// ```
/// #[macro_use]
/// extern crate serializers;
/// #[macro_use]
/// extern crate serde_json;
///
/// use serializers::*;
///
/// pub struct User {
///     id: u64,
///     country: Country,
///     friends: Vec<User>,
/// }
///
/// #[derive(Clone)]
/// pub struct Country {
///     id: u64,
/// }
///
/// serializer! {
///     #[derive(Debug)]
///     pub struct UserSerializer<User> {
///         attr(identifier, id)
///         has_one(homeland, country, CountrySerializer)
///         has_many(buddies, friends, UserSerializer)
///     }
/// }
///
/// serializer! {
///     #[derive(Debug)]
///     pub(crate) struct CountrySerializer<Country> {
///         attr(code, id)
///     }
/// }
///
/// fn main() {
///     let denmark = Country {
///         id: 1,
///     };
///
///     let bob = User {
///         id: 1,
///         country: denmark.clone(),
///         friends: vec![
///             User {
///                 id: 2,
///                 country: denmark.clone(),
///                 friends: vec![],
///             }
///         ],
///     };
///
///     let json: String = UserSerializer::serialize(&bob);
///
///     assert_eq!(
///         json,
///         json!({
///             "buddies": [
///                 {
///                     "buddies": [],
///                     "homeland": { "code": 1 },
///                     "identifier": 2
///                 }
///             ],
///             "homeland": { "code": 1 },
///             "identifier": 1
///         }).to_string(),
///     );
/// }
/// ```
#[macro_export]
macro_rules! serializer {
    // entry points
    {
        #[derive( $($derive_tokens:tt),* )]
        pub(crate) struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [ $($derive_tokens),* ],
            struct_def = { pub(crate) struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };

    {
        pub(crate) struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [],
            struct_def = { pub(crate) struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };

    {
        #[derive( $($derive_tokens:tt),* )]
        pub struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [ $($derive_tokens),* ],
            struct_def = { pub struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };

    {
        pub struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [],
            struct_def = { pub(crate) struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };


    {
        #[derive( $($derive_tokens:tt),* )]
        struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [ $($derive_tokens),* ],
            struct_def = { struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };

    {
        struct $name:ident<$type:ty> { $($body:tt)* }
    } => {
        __serializer! {
            derives = [],
            struct_def = { pub(crate) struct $name; },
            name = ($name),
            ttype = ($type),
            body = ( $($body)* ),
        }
    };


}

#[macro_export]
#[doc(hidden)]
macro_rules! __serializer {
    {
        derives = [],
        struct_def = { $($struct_def_tokens:tt)* },
        name = ( $name:ident ),
        ttype = ( $type:ty ),
        body = ( $($body:tt)* ),
    } => {
        #[allow(dead_code, missing_docs)]
        $($struct_def_tokens)*
        __serializer! { impl $name<$type> { $($body)* } }
    };

    {
        derives = [ $($derive_tokens:tt),* ],
        struct_def = { $($struct_def_tokens:tt)* },
        name = ( $name:ident ),
        ttype = ( $type:ty ),
        body = ( $($body:tt)* ),
    } => {
        #[allow(dead_code, missing_docs)]
        #[derive( $($derive_tokens),* )]
        $($struct_def_tokens)*
        __serializer! { impl $name<$type> { $($body)* } }
    };

    {
        impl $name:ident<$type:ty> { $($rest:tt)* }
    } => {
        impl $crate::Serializer<$type> for $name {
            fn serialize_into(&self, v: &$type, b: &mut $crate::Builder) {
                __serializer! { [b, v] $($rest)* }
            }
        }

        #[allow(dead_code)]
        impl $name {
            fn serialize(v: &$type) -> String {
                $name.serialize(v)
            }

            fn serialize_iter<'a, I>(v: I) -> String
            where
                I: IntoIterator<Item = &'a $type>,
            {
                $name.serialize_iter(v)
            }
        }
    };

    // base case
    { [$b:expr, $v:expr] } => {};

    // attr
    {
        [$b:expr, $v:expr] attr($attr:ident) $($rest:tt)*
    } => {
        __serializer! { [$b, $v] attr($attr, $attr) $($rest)* }
    };

    {
        [$b:expr, $v:expr] attr($key:ident, $field:ident) $($rest:tt)*
    } => {
        $b.attr(stringify!($key), &$v.$field);
        __serializer! { [$b, $v] $($rest)* }
    };

    // has_one
    {
        [$b:expr, $v:expr] has_one($key:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        __serializer! { [$b, $v] has_one($key, $key, $has_one_ser) $($rest)* }
    };

    {
        [$b:expr, $v:expr] has_one($key:ident, $field:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        $b.has_one(stringify!($key), &$v.$field, &$has_one_ser);
        __serializer! { [$b, $v] $($rest)* }
    };

    // has_many
    {
        [$b:expr, $v:expr] has_many($key:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        __serializer! { [$b, $v] has_many($key, $key, $has_one_ser) $($rest)* }
    };

    {
        [$b:expr, $v:expr] has_many($key:ident, $field:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        $b.has_many(stringify!($key), &$v.$field, &$has_one_ser);
        __serializer! { [$b, $v] $($rest)* }
    };
}
