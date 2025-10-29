use crate::{DatabaseResult, DatabaseServiceTrait, error::DatabaseError};
use bson::oid::ObjectId;
use paste::paste;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::sync::Arc;

/// Trait that defines the behavior for each collection in database.
///
/// Operations divide in methods and functions.
/// Methods are save and delete and refers only to the current instance.
///
/// Functions are general and operate outside the instance
pub trait DatabaseDocumentTrait: Sized + Send + Sync + Serialize + DeserializeOwned {
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
macro_rules! database_document {
    ( $(#[doc = $doc:expr])* $struct_name:ident, $collection_name:expr, $(
        $(#[$field_attr:meta])*
        $field_name:ident : $field_type:ty
    ),* $(,)? ) => {
        // struct creation
        $( #[doc = $doc] )*
        #[derive(Debug, Serialize, Deserialize, Clone)]
        pub struct $struct_name {
            #[serde(rename = "_id")]
            id: ObjectId,
            $(
                $(#[$field_attr])*
                $field_name: $field_type,
            )*
        }

        impl $struct_name {
            // implementation of getter and setters
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

        // implementation of database document trait
        impl DatabaseDocumentTrait for $struct_name {
            fn collection_name() -> &'static str {
                $collection_name
            }

            fn get_id(&self) -> &ObjectId {
                &self.id
            }
        }

        // creation of builder
        paste! {
            #[derive(Default)]
            pub struct [<$struct_name Builder>]<T>
            where T: DatabaseServiceTrait {
                database_service: Arc<T>,
                $(
                    $field_name: Option<$field_type>,
                )*
            }

            // implementation of methods that allow to set fields and build the document
            impl<T> [<$struct_name Builder>]<T>  where T: DatabaseServiceTrait{
                pub fn new(database_service: Arc<T>) -> Self {
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
                pub async fn build(self) -> DatabaseResult<$struct_name> {
                    let document = mongodb::bson::doc! {
                        $(
                            stringify!($field_name): self.$field_name.clone()
                                .ok_or_else(|| DatabaseError::DocumentNotValid(
                                    format!("Missing {}", stringify!($field_name))
                                ))?,
                        )*
                    };

                    let doc_id = self.database_service.insert_one::<$struct_name>(document).await?;
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

database_document!(
    #[doc = "Example collection"]
    ExampleCollection,
    "example_collection",
    first_element: String,
    second_element: String,
);
