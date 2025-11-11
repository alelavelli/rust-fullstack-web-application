use std::sync::Arc;

use bson::doc;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;

use crate::{
    ServiceResult,
    error::{DatabaseError, ServiceAppError},
    model::{BlogPost, BlogPostBuilder},
    service::database::DatabaseServiceTrait,
};

/// BlogService manages the BlogPost resources providing
/// methods to create and retrieve them
pub struct BlogService<D: DatabaseServiceTrait> {
    database_service: Arc<D>,
    transaction: Option<Arc<RwLock<D::Transaction>>>,
}

impl<D: DatabaseServiceTrait> BlogService<D> {
    pub fn new(database_service: Arc<D>, transaction: Option<Arc<RwLock<D::Transaction>>>) -> Self {
        Self {
            database_service,
            transaction,
        }
    }

    /// Creates a new blog post and saves it into the database
    pub async fn publish_post(
        &self,
        title: String,
        content: String,
        user_id: ObjectId,
        creation_date: DateTime<Utc>,
    ) -> ServiceResult<BlogPost> {
        BlogPostBuilder::new(self.database_service.clone())
            .title(title)
            .content(content)
            .user_id(user_id)
            .creation_date(creation_date)
            .build(self.transaction.clone())
            .await
            .map_err(|err| match err {
                DatabaseError::DocumentNotValid(msg) => ServiceAppError::InvalidRequest(msg),
                other => ServiceAppError::DatabaseError(other),
            })
    }

    /// Returns all the blog posts in the database, if the user_id
    /// is specified then only the posts created by him are returned
    pub async fn get_posts(&self, user_id: Option<ObjectId>) -> ServiceResult<Vec<BlogPost>> {
        let query = if let Some(user_id) = user_id {
            doc! { "user_id": user_id }
        } else {
            doc! {}
        };

        let blog_posts = self.database_service.find_many::<BlogPost>(query).await?;

        Ok(blog_posts)
    }
}
