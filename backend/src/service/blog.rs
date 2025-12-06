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
        username: String,
        creation_date: DateTime<Utc>,
    ) -> ServiceResult<BlogPost> {
        BlogPostBuilder::new(self.database_service.clone())
            .title(title)
            .content(content)
            .user_id(user_id)
            .username(username)
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use bson::{doc, oid::ObjectId};
    use chrono::{DateTime, Utc};

    use crate::{
        model::{BlogPost, BlogPostBuilder},
        service::{
            blog::BlogService,
            database::{DatabaseServiceTrait, memory_service::MemoryDatabaseService},
        },
    };

    async fn create_blog(
        database_service: Arc<MemoryDatabaseService>,
        title: String,
        content: String,
        user_id: &ObjectId,
    ) -> BlogPost {
        BlogPostBuilder::new(database_service)
            .title(title)
            .content(content)
            .user_id(*user_id)
            .creation_date(DateTime::<Utc>::default())
            .build(None)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_posts() {
        let database_service = Arc::new(MemoryDatabaseService::default());

        let first_user = ObjectId::new();
        let second_user = ObjectId::new();

        let blog_service = BlogService::new(database_service.clone(), None);

        let mut blog_posts = vec![];
        for i in 0..5 {
            let user_id = if i < 3 { first_user } else { second_user };
            blog_posts.push(
                create_blog(
                    database_service.clone(),
                    format!("Blog title {i}"),
                    format!("Blog content {i}"),
                    &user_id,
                )
                .await,
            );
        }

        let first_user_posts = blog_service
            .get_posts(Some(first_user.clone()))
            .await
            .unwrap();
        assert_eq!(first_user_posts.len(), 3);

        let second_user_posts = blog_service
            .get_posts(Some(second_user.clone()))
            .await
            .unwrap();
        assert_eq!(second_user_posts.len(), 2);

        let all_user_posts = blog_service.get_posts(None).await.unwrap();
        assert_eq!(all_user_posts.len(), 5);
    }

    #[tokio::test]
    async fn test_publish_post() {
        let database_service = Arc::new(MemoryDatabaseService::default());
        let blog_service = BlogService::new(database_service.clone(), None);

        let mut titles = vec![];
        let first_user = ObjectId::new();
        let second_user = ObjectId::new();

        for i in 0..5 {
            let user_id = if i < 3 { first_user } else { second_user };
            let title = format!("title {i}");
            blog_service
                .publish_post(
                    title.clone(),
                    title.clone(),
                    user_id,
                    "username".to_string(),
                    DateTime::<Utc>::default(),
                )
                .await
                .unwrap();
            titles.push(title);
        }

        assert_eq!(
            database_service
                .find_many::<BlogPost>(doc! {})
                .await
                .unwrap()
                .len(),
            5
        );
    }
}
