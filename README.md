# Full stack web application template in Rust

This repository provides an Hello World blog use case for a full-stack Rust web application in which users can publish and read blog posts.
The focus is not on the application logic but on building all the required components that a web application needs from the frontend to the database interactions.

Table 1 shows the chosen framework for each component in the stack.

| Component      | Framework      |
| -------------- | -------------- |
| Backend        | [Axum](https://github.com/tokio-rs/axum)           |
| Frontend       | [Yew](https://yew.rs/)            |
| Database       | [MongoDB](https://www.mongodb.com/docs/drivers/rust/current/)        |
| Infrastructure | [Docker Compose](https://docs.docker.com/compose/) |

_Table 1: frameworks of architectural components._

The application is accessibile via Browser and API and provides all functionalities that a common application has: CRUD operations to database, authorization and access control, routers, facades and services components.

## Backend

The backend entry point, `main.rs`, configures basic services and starts the `axum` application to listen for requests from the clients:

1. The `EnvironemntService` and the `MongoDBDatabaseService` objects are created to form the `AppState` that will be shared among the rest of the application.
2. Logging is setup initializing `tracing_subcriber`.
3. A `TcpListener` is created to listen on the specified port
4. The application `Router` is built and then served by the listener

### Application Router

The application router is `axum::Router` struct and according to the variable `FrontendMode` serves only backend routes or static frontend resources as well.
The router is composed of serveral nested routers, one for each application usage line (guest, admin, user, ...).

> Note that the definition of routers in this way is totally arbitrary but I prefer to divide routes like this because it separate user personas facilitating the consequent RBAC.

Routers are added using utility methods present in `backend::router` by specifying the base url for the rout and the application struct.

Middlewares are attached to the application router:

- _Database Transaction:_ starts a new transaction when the request method is different from GET comitting or aborting it according to the result type
- _Logging:_ setup logging for the routes
- _CORS:_ defines CORS policy for the application

Finally, the state with added to the application.

### Nested Routers

All the nested routers follows the same structure, hence I will explain the underlying idea.

Each module has a utility method `add_this_router` that is used to nest the router as describe above.

The router has several routes that it serves, since it is a REST API, the routes names are for the resources and the HTTP method defines the behiavior.

A route handler takes as parameters:

- _State_: the state of the application extracted with `State(state): State<Arc<AppState>>`
- _Authentication token_: it will be explained later but for now, all you need to know is that it is `JWTAuthClaim` struct and contains all the information to authenticate the client

Optionally, according to the specific handler, there are additional parameters:

- _Database transaction_: injected by the transaction middleware, it is extracted with  `Extension(transaction): Extension<Arc<RwLock<MongoDBDatabaseService>>>`
- _Request body_: extracted with `Json(payload): Json<PAYLOAD_TYPE>`
- _Query parameter_: extracted with `Path(param): Path<PARAM_TYPE>`

The route handler returns a `AppResult<T>` type that is an alias for `Result<AppJson<T>, AppError>`, more details in the next sections.

Usually, the body of the handler creates an instance of the respective facade struct and calls its proper method.

That's why the responsibility of the handler is to authenticate the user, extract all the information from the request delegating the application logic to the facade.

Now, is the time to talk about facades.

### Facade

The facade decouple the handler with the actual application services providing a unified interface to serve a client's request.
There is one facade for each nested router and each struct contains attributes that are useful to its method avoiding passing them as method's parameters.

Each facade returns the `FacadeResult` type which is an alias for `Result<T, AppError>`.

#### Guest Facade

The guest facade, as you can guess, is used for requests done by unauthenticated clients and beyon the contructor function `new`, it provides only the `authenticate_user` method.

#### Admin Facade

The admin facade contains all the operations that an admin user can do.
For this reason, the constructor function `new` creates a `AccessControl` struct with the information about the user who is making the request to perform access control verifying that it exists and has administration permissions.
If everything is ok, then the facade struct instance is returned.

The methods it provides do not require access control and they contain only the application logic.

#### User Facade

The user facade contains all operations of a general user that is not admin.
However, some methods require additional permissions and the access control struct is used to verify them in the methods.

### Services

Facades uses one or more services to fulfil the requests.
There are _primary services_ that define the application logic and _secondary services_ that are used to support the primary ones.

#### Access Control

The `AccessControl` service is used to verify that the user who is making the request has the correct permissions.

It is a struct with attributes:

- `user: Arc<RwLock<SmartDocumentReference<User>>>`: it is the database document with information about the user.

> In the next sections I will explain why the `User` document is wrapped with all those types.

- `database_service: Arc<D: DatabaseServiceTrait>`: it is a reference to the `DatabaseService` that is needed to query the database.

In the constructor method `new` verifies that the user exists before returning the struct instance.
Indeed, if the user does not exist then it is not allowed to perform any operation ;).

Then it provides methods to verify the permissions, for each control there are two versions:

- consuming method like `is_platform_admin(self) -> ServiceResult<Self>` that allows chaining the method calls
- non consuming method like `is_platform_admin_ref(&self) -> ServiceResult<()>` that perform the control without consuming the struct and returning it again

#### Application Services

Application services like `UserService` and `BlogService` provide methods to operate on the specific context's resources.
They are instantiated with the information that identify the context like the user or the database service and provides methods to work with them.

Note that those services do not apply any access control because the facade is responsible of that.

#### Database service

Database service is more complex because I wanted to exchange it with different implementations for actual operations or testing.

For this reason, there is the `DatabaseServiceTrait` that provides essential behaviors required by a type to be a database service.

In particular, it provides the methods to start and close a databaes connection, `connect(&mut self)` and `shutdown(&mut self)` respectively.

Then there is a method to create a new database transaction `new_transaction(&self)` and several methods to perform database operations like inserting new documents.
These methods has a generic type `T: DatabaseDocumentTrait` that is used to deserialize correctly the retrieved documents.

> Note that this database service works for _document based_ databases like MongoDB and its methods work with the `bson::Document` type.

The database service module contains several sub modules:

- _document_: defines `DatabaseDocumentTrait` and macros to define database document structs
- _memory_service_: implementation of `DatabaseServiceTrait` for in memory database that is used for testing
- _mongodb_service_: implementation of `DatabaseServiceTrait` for MongoDB database
- _smart_document_: enum used to cache a database document
- _transaction_: database transaction for MongoDB

##### Document

Document module defines `DatabaseDocumentTrait` that is used by `DatabaseServiceTrait`'s generic bound.

The most interesting part is the `database_document` macro that allows to define structs that implements `DatabaseDocumentTrait` and more, in an easy way.

Here an example of database document definition:

```rs
database_document!(
    #[doc = "Blog post document"]
    BlogPost,
    "blog_post",
    title: String,
    content: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    creation_date: DateTime<Utc>,
    user_id: ObjectId
);
```

The macro accepts:

- zero or more documentation blocks
- the struct name
- the collection name on the database
- one or more fields (with optional attributes) composed of field name and field type

It creates:

- a new struct for the document
- methods to access the fields as reference and mutable reference
- methods to set the fields
- implementation of `DatabaseDocumentTrait`
- a builder struct `<STRUCT_NAME>Builder` that directly insert in the database the document when created (via the `build` method)

##### Memory service

`MemoryDatabaseService` is injected during unit tests avoiding the need of a MongoDB cluster to perform tests for the backend.
It implements the `DatabaseServiceTrait` and stores all the documents as `RwLock<HashMap<String, Vec<Document>>>` where the key is the collection name.

#### MongoDB service

`MongoDBDatabaseService` is the actual database service and it connects to a MongoDB cluster and perform all the database operations.

#### Smart document

`SmartDocumentReference`, I know the name is bad but I had no idea, is a utility enum that allows to cache a database document or avoiding to load it from the database if not requested.

It has two variants:

- `Id(ObjectId)`: when the document is not yet retrieved from the database
- `Document(T)`: when the document is available on memory

The provided methods allows to get the id (no database query will be performed for the Id variant) and to get the document as reference (mutable as well) or consuming the object providing the directly the document.

#### Transaction

The transaction module contains the definition of the transaction trait `DatabaseTransactionTrait` and its implementations for MongoDB and for In Memory: `MongoDBDatabaseTransaction` and `MemoryDatabaseTransaction` respectively.

`MongoDBDatabaseTransaction` implementation is pretty simple: it contains a `mongodb::client::ClientSession` and it is provided to the database service to attach it during database operations.

### Error types

I defined different error types that are used at different levels of the application.

The error module uses the crate `thiserror` to facilitate the definition of the error variants.

#### DatabaseError

`DatabaseError` type is used by the `DatabaseServiceTrait` and contains all the possible outcomes when dealing with it and with the actual database.

#### AuthError

`AuthError` is used by the `JWTAuthClaim` when something goes wrong when dealing with authentication tokens and by the `login` method of `UserService`.

In its implementation there is the method `to_status_message` that translate each enum variant into the specific HTTP response code and message.

#### ServiceAppError

`ServiceAppError` is used by all the services and contains all the possible outcomes.

#### AppError

`AppError` is the error type returned by facades and routers' handlers and implements the trait `IntoReponse` to be automatically translated into HTTP response code and message.

It is separated with `ServiceAppError` because according to the context we can mask some internal errors to `InternalServerError`.

Facades are responsible to explicitly translated `ServiceAppError` returned type into `AppError` to correctly communicate the error to the client.
