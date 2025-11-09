use bson::oid::ObjectId;
use serde::{Serialize, de::DeserializeOwned};

/// Extension of DatabaseDocumentTrait useful to avoid adding all the additional
/// traits all over the code base
pub trait DecoratedDatabaseDocumentTrait:
    DatabaseDocumentTrait + Sized + Send + Sync + Serialize + DeserializeOwned
{
}

/// Trait that defines the behavior for each collection in database
pub trait DatabaseDocumentTrait: Sized {
    fn get_id(&self) -> &ObjectId;
    fn collection_name() -> &'static str;
}

/// The macro generates struct that implements DatabaseDocumentTrait trait
///
/// You need to provide struct level docstring, the name of the struct,
/// the name of the mongodb collection and the fields with their type.
///
/// The macro creates a builder that on the build method will insert the
/// document on the database
///
/// We specify the full path of entities because macro expansions happen
/// in the caller's module scope and therefore they need to be imported there
#[macro_export]
macro_rules! database_document {
    ( $(#[doc = $doc:expr])* $struct_name:ident, $collection_name:expr, $(
        $(#[$field_attr:meta])*
        $field_name:ident : $field_type:ty
    ),* $(,)? ) => {
        // struct creation
        $( #[doc = $doc] )*
        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
        pub struct $struct_name {
            #[serde(rename = "_id")]
            id: ::bson::oid::ObjectId,
            $(
                $(#[$field_attr])*
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            // implementation of getter and setters
            ::paste::paste!{
                $(
                    #[allow(dead_code)]
                    pub fn $field_name(&self) -> &$field_type {
                        &self.$field_name
                    }
                    #[allow(dead_code)]
                    pub fn [<$field_name _mut>](&mut self) -> &mut $field_type {
                        &mut self.$field_name
                    }
                    #[allow(dead_code)]
                    pub fn [<set_ $field_name>](&mut self, value: $field_type) {
                        self.$field_name = value;
                    }
                )*
            }

        }

        // implementation of database document trait
        impl $crate::service::database::document::DatabaseDocumentTrait for $struct_name {
            fn collection_name() -> &'static str {
                $collection_name
            }

            fn get_id(&self) -> &::bson::oid::ObjectId {
                &self.id
            }
        }
        impl $crate::service::database::document::DecoratedDatabaseDocumentTrait for $struct_name {}

        // creation of builder
        ::paste::paste! {
            #[derive(Default)]
            pub struct [<$struct_name Builder>]<T>
            where T: $crate::service::database::DatabaseServiceTrait {
                database_service: std::sync::Arc<T>,
                $(
                    $field_name: Option<$field_type>,
                )*
            }

            // implementation of methods that allow to set fields and build the document
            impl<T> [<$struct_name Builder>]<T>  where T: $crate::service::database::DatabaseServiceTrait{
                pub fn new(database_service: std::sync::Arc<T>) -> Self {
                    Self {
                        database_service,
                        $(
                            $field_name: None,
                        )*
                    }
                }

                $(
                    #[allow(dead_code)]
                    pub fn $field_name(mut self, value: $field_type) -> Self{
                        self.$field_name = Some(value);
                        self
                    }
                )*

                /// Build the database document by creating it on the database via
                /// the database service
                pub async fn build(
                    self,
                    transaction: Option<std::sync::Arc<tokio::sync::RwLock<T::Transaction>>>,
                ) -> $crate::error::DatabaseResult<$struct_name> {
                    let document = mongodb::bson::doc! {
                        $(
                            stringify!($field_name): self.$field_name.clone()
                                .ok_or_else(|| $crate::error::DatabaseError::DocumentNotValid(
                                    format!("Missing {}", stringify!($field_name))
                                ))?,
                        )*
                    };

                    let doc_id = self.database_service.insert_one::<$struct_name>(
                        document,
                        transaction
                    ).await?;
                    Ok($struct_name {
                        id: doc_id,
                        $(
                            $field_name: self.$field_name.unwrap(),
                        )*
                    })
                }

            }
        }
    };
}

/// The macro generates struct used as an object inside the database document
///
/// You need to provide struct level docstring, the name of the struct
#[macro_export]
macro_rules! embedded_document {
    ( $(#[doc = $doc:expr])* $struct_name:ident, $ ( $field_name:ident : $field_type:ty ),* ) => {
        $( #[doc = $doc] )*
        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct $struct_name {
            $ ($field_name: $field_type, )*
        }

        impl $struct_name {
            #[allow(dead_code)]
            #[allow(clippy::too_many_arguments)]
            pub fn new($($field_name: $field_type),*) -> Self {
                Self { $($field_name),*}
            }

            paste!{
                $(
                    #[allow(dead_code)]
                    pub fn $field_name(&self) -> &$field_type {
                        &self.$field_name
                    }
                    #[allow(dead_code)]
                    pub fn [<$field_name _mut>](&mut self) -> &mut $field_type {
                        &mut self.$field_name
                    }
                    #[allow(dead_code)]
                    pub fn [<set_ $field_name>](&mut self, value: $field_type) {
                        self.$field_name = value;
                    }
                )*
            }
        }
    };
}
