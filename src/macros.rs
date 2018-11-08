/// This macro is the primary way to make serializers. See the [top level docs](index.html) for an
/// example.
///
/// This macro expands into a function the implements [`Serializer`](trait.Serializer.html). This
/// is because [`Serializer<T>`](trait.Serializer.html) is automatically implemented for functions
/// with the signature `Fn(&T, &mut Builder)`.
///
/// ## Customization
///
/// The macro also lets you set separate JSON keys and field names. That is done by adding an
/// additional first argument to `attr`, `has_one`, or `has_many` which will be the key.
///
/// You can also write `pub` in front of the name of your serializer to make the serializer public. Additionally you can write `pub(crate)` to make it public within your crate.
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
///     pub serialize_user<User> {
///         attr(identifier, id)
///         has_one(homeland, country, serialize_country)
///         has_many(buddies, friends, serialize_user)
///     }
/// }
///
/// serializer! {
///     pub(crate) serialize_country<Country> {
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
///     let json: String = serialize_user.serialize(&bob);
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
        pub(crate) $name:ident<$type:ty> { $($rest:tt)* }
    } => {
        #[allow(missing_docs, dead_code)]
        pub(crate) fn $name(v: &$type, b: &mut Builder) {
            serializer! { [b, v] $($rest)* }
        }
    };

    {
        pub $name:ident<$type:ty> { $($rest:tt)* }
    } => {
        #[allow(missing_docs, dead_code)]
        pub fn $name(v: &$type, b: &mut Builder) {
            serializer! { [b, v] $($rest)* }
        }
    };

    {
        $name:ident<$type:ty> { $($rest:tt)* }
    } => {
        #[allow(missing_docs, dead_code)]
        fn $name(v: &$type, b: &mut Builder) {
            serializer! { [b, v] $($rest)* }
        }
    };

    // base case
    { [$b:expr, $v:expr] } => {};

    // attr
    {
        [$b:expr, $v:expr] attr($attr:ident) $($rest:tt)*
    } => {
        serializer! { [$b, $v] attr($attr, $attr) $($rest)* }
    };

    {
        [$b:expr, $v:expr] attr($key:ident, $field:ident) $($rest:tt)*
    } => {
        $b.attr(stringify!($key), &$v.$field);
        serializer! { [$b, $v] $($rest)* }
    };

    // has_one
    {
        [$b:expr, $v:expr] has_one($key:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        serializer! { [$b, $v] has_one($key, $key, $has_one_ser) $($rest)* }
    };

    {
        [$b:expr, $v:expr] has_one($key:ident, $field:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        $b.has_one(stringify!($key), &$v.$field, &$has_one_ser);
        serializer! { [$b, $v] $($rest)* }
    };

    // has_many
    {
        [$b:expr, $v:expr] has_many($key:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        serializer! { [$b, $v] has_many($key, $key, $has_one_ser) $($rest)* }
    };

    {
        [$b:expr, $v:expr] has_many($key:ident, $field:ident, $has_one_ser:ident) $($rest:tt)*
    } => {
        $b.has_many(stringify!($key), &$v.$field, &$has_one_ser);
        serializer! { [$b, $v] $($rest)* }
    };
}
