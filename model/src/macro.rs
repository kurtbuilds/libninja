/// import!("bytes", a, b, c);
/// import!("bytes");
/// import!("bytes::*");
/// import!(pub "bytes::*");
#[macro_export]
macro_rules! import {
    ($path:expr) => {
        ::ln_model::Import::package($path)
    };
    ($path:expr, $($imports:ident),*) => {
        ::ln_model::Import::new($path, vec![$(stringify!($imports)),*])
    };
    (pub $path:expr, $($imports:ident),*) => {
        ::ln_model::Import::new($path, vec![$(stringify!($imports)),*]).public()
    };
}
/// Macro to create a FnArg. Called targ because the type is a TokenStream (specifically a path), rather than a &str.
/// targ!(access_token: String)
#[macro_export]
macro_rules! targ {
    ($name:ident : $ty:path) => {
        ::ln_model::FnArg {
            name: stringify!($name).to_string(),
            ty: {
                let ty = ::syn::parse_str::<::syn::Ident>(stringify!($ty)).unwrap();
                ::quote::ToTokens::to_token_stream(&ty)
            },
            default: None,
            treatment: None,
        }
    };
}
/// Macro to create a FnArg. Second arg is a expr that evaluates to a string. Optionally takes a default.
/// arg!(count: "int" = 500)
#[macro_export]
macro_rules! arg {
    ($name:ident : $ty:expr) => {
        ::ln_model::FnArg {
            name: ::ln_model::ArgIdent::from(stringify!($name)),
            ty: $ty.to_string(),
            default: None,
            treatment: None,
        }
    };
    ($name:ident : $ty:expr , $default:expr) => {
        ::ln_model::FnArg {
            name: ::ln_model::ArgIdent::from(stringify!($name).to_string()),
            ty: $ty.to_string(),
            default: $default.to_string(),
            treatment: None,
        }
    };
}


#[macro_export]
macro_rules! field {
    (pub(crate) $name:ident : $ty:expr) => {
        ::ln_model::Field {
            name: ::ln_model::Name::new(stringify!($name)),
            ty: ($ty).into(),
            visibility: ::ln_model::Visibility::Crate,
            ..Field::default()
        }
    };
    (pub $name:ident : $ty:expr) => {
        ::ln_model::Field {
            name: ::ln_model::Name::new(stringify!($name)),
            ty: ($ty).into(),
            visibility: ::ln_model::Visibility::Public,
            ..Field::default()
        }
    };
    ($name:ident : $ty:expr) => {
        ::ln_model::Field {
            name: ::ln_model::Name::new(stringify!($name)),
            ty: ($ty).into(),
            ..Field::default()
        }
    };
}

#[macro_export]
macro_rules! tfunc {
    ($name:ident ($($arg:ident : $ty:path),* $(,)?) => $body:expr) => {
        ::ln_model::Function{
            name: stringify!($name).to_string(),
            args: vec![$(targ!($arg : $ty)),*],
            ret: TokenStream::new(),
            body: $body,
            doc: None,
            async_: false,
            public: false,
            annotations: vec![],
        }
    };
    ($name:ident ($($arg:ident : $ty:path),* $(,)?) => $ret:path => $body:expr) => {
        ::ln_model::Function{
            name: stringify!($name).to_string(),
            args: vec![$(targ!($arg : $ty)),*],
            ret: {
                let ret = ::syn::parse_str::<::syn::TypePath>(stringify!($ret)).unwrap();
                ::quote::ToTokens::to_token_stream(&ret)
            },
            body: $body,
            doc: None,
            async_: false,
            public: false,
            annotations: vec![],
        }
    };
}

#[macro_export]
macro_rules! lit {
    ($($arg:tt)*) => {
        ::ln_model::Literal(format!($($arg)*), false);
    };
}