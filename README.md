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

> Disclamer: any line of code in this repository was written by human hands! Obviously, sometimes I used an LLM as support tool for boring tasks like writing down the css classes but there is no vibe-coding here.
> The backend comes from two previous projects `sandbox-rust-web-app` and `employees-manager`, it is a more mature version.
> The frontend is completely learned here (as you can see from the commit history) and vibe-coding was totally useless to learn this framework.

## Backend

The backend entry point, `main.rs`, configures basic services and starts the `axum` application to listen for requests from the clients:

1. The `EnvironmentService` and the `MongoDBDatabaseService` objects are created to form the `AppState` that will be shared among the rest of the application.
2. Logging is setup initializing `tracing_subcriber`.
3. A `TcpListener` is created to listen on the specified port
4. The application `Router` is built and then served by the listener

### Application Router

The application router is `axum::Router` struct and according to the variable `FrontendMode` serves only backend routes or static frontend resources as well.
The router is composed of several nested routers, one for each application usage line (guest, admin, user, ...).

> Note that the definition of routers in this way is totally arbitrary but I prefer to divide routes like this because it separates user personas facilitating the consequent RBAC.

Routers are added using utility methods present in `backend::router` by specifying the base url for the rout and the application struct.

Middlewares are attached to the application router:

- _Database Transaction:_ starts a new transaction when the request method is different from GET committing or aborting it according to the result type
- _Logging:_ setup logging for the routes
- _CORS:_ defines CORS policy for the application

Finally, the state with added to the application.

### Nested Routers

All the nested routers follows the same structure, hence I will explain the underlying idea.

Each module has a utility method `add_this_router` that is used to nest the router as describe above.

The router has several routes that it serves, since it is a REST API, the routes names are for the resources and the HTTP method defines the behavior.

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

The guest facade, as you can guess, is used for requests done by unauthenticated clients and beyond the constructor function `new`, it provides only the `authenticate_user` method.

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

In particular, it provides the methods to start and close a database connection, `connect(&mut self)` and `shutdown(&mut self)` respectively.

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

`AppError` is the error type returned by facades and routers' handlers and implements the trait `IntoResponse` to be automatically translated into HTTP response code and message.

It is separated with `ServiceAppError` because according to the context we can mask some internal errors to `InternalServerError`.

Facades are responsible to explicitly translated `ServiceAppError` returned type into `AppError` to correctly communicate the error to the client.

## Frontend

> A little disclaimer before reading the frontend description: this is my first experience in writing a Rust frontend with `yew`.
> I decided to do not use external visual crates for nicer components but to leverage only the base yew crate.
> I wanted to focus on the application logic instead of UI.
>
> Moreover, being this the first experience in writing a frontend in Rust it could be possible that I used some anti-patterns.
> Any PR is kindly accepted ;)

The entry files in the frontend package are `index.html`, `index.scss` and `src/main.rs` that strictly follows the Yew tutorial.
In particular, `index.scss` contains all the styles, for more complex projects it could be split into different files.
`main.rs` is pretty straightforward and the only different is the initialization of the logging system though `wasm_logger::init(wasm_logger::Config::default());`.

The real application starts from the `src/app.rs` file with all the other components.

The frontend crate is structured as following:

- `app` contains the routes definition, initializes the context providing it to all the other components;
- `service` module defines the services used by the frontend application: api and auth;
- `page` module contains the web app pages
- `component` module contains the components used by the

Other modules are `environment`, `enums`, `types`, `model` and `error` that will be explained later.

### Environment Module

The environment service follows the same principles as the backend, it collects all the environment variables that will be used by the application.

> Note: differently from the backend, there are no environment variables available at runtime, so the current implementation always takes the default values.
> Hence, the parametrization of the environment must be done in a different way through Trunk maybe.

The `mock` environment variable is used for testing purposes to route returned mocked types instead of doing the actual API request.
This is very useful because allows the developer to write the frontend application without an actual backend implementation.

### App component

The `App` component is the entry point of the Single Page Application.

The module defines the routes using `yew-router` crate as an enum.
Each route is associated with a page function component that will render the page.

So, the html block returned by `App` is composed of different elements.
At the external level there is `BrowserRouter` that handles the navigation on the browser; then there is `ContextProvider` that allows to access to the application context; after that there is the rendered components: header, main and footer.

In addition, the `App` function component instantiate the application context and set the logged user info from the local storage.
More details in the next sections.

### AppContext

The module `types` defines general application types like `ApiResponse<T>` or results and `AppContext`.

The `AppContext` struct contains all the common information that any component in the application need to access to.
In particular, it contains the `LoggedUserInfo` that identify the current user in the session.
It is `None` for guest users and `Some` for logged ones.

The `AppContext` object is stored inside a state handle so that it can be cloned among the different components.

### AuthService

`AuthService` is responsible to manage the current session interacting with the browser's local storage.

Its constructor method requires the storage location name to use for storing and reading information and the app context.

It provides three methods:

- `get_auth_token`: returns the authorization token (jwt) from the application context without interacting with the local storage
- `remove_logged_user`: clear the local storage and the app context object from the current user
- `set_logged_user_info_from_storage`: reads from the local storage the auth information, then performs an API request to get the trusted actual user information and updates the app context object with them

> Important note: we aware that updating the context object will trigger the rendering of the entire application component since it is in the `ContextProvider` block.
> For this reason, the method to set the logged user info is done once by the `App` component.

### ApiService

`ApiService` is responsible to make requests to the API backend server.
It is initialized with the api url, the session user token and the mock parameter.

For now, there is only one ApiService struct because the application is a toy example.
For more complex scenarios, more api services can be created to separate concerns like the routers in the backend.

Any request method returns an `ApiResult` type that has `ApiResponse` type in the Ok variant and `ApiError` in the Err one.

`ApiResponse` contains the body as generic and the response status, `HttpStatus` enum.
Note that `ApiResponse` accept a generic `T` as type parameter, however, it is highly common to use `Option<T>` for non success responses.

### Application page

I will not explain all the pages in this repo but I will show the common structure and idea.

First of all, the page component access the app context to understand if there is a logged user or not and redirects accordingly.
For instance, the login page redirects to home when there is a logged user while the other pages redirect to login page when there is no logged user.
Admin page has an additional control since it can be accessed only by admin users and it automatically redirects to not-found page when there is a standard user.

The actual body of the page contains the html elements to render the page.

> Note that this application does not rely on external elements crates like yew-material because I wanted to focus on the application logic.

The crate `wasm_bindgen_futures` is used to spawn futures on the local thread to use the api service making requests to the backend.

### Components

The `component` module contains several components that are used by the application pages like the header, the footer or specific ones like post details or user list.
Usually, components requires some inputs using the properties.
