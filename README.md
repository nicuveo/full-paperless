# Paper Plane :card_index_dividers:

An unofficial rich client for [paperless-ngx](https://docs.paperless-ngx.com/),
based on its REST API. The goal of this project is to provide a feature-complete
API to manage a paperless-ngx instance while also providing an abstraction layer
that reduces the risk of misuse.

It is a fully async library and is meant to be used with a
[tokio](https://tokio.rs) runtime.

> :warning: This is a work in progress. See the [Remaining
> work](#remaining-work) section.

#### TOC

- [Overview](#overview)
   * [Service API](#service-api)
   * [Pagination](#pagination)
   * [Making changes](#making-changes)
      + [Update](#update)
      + [Patch](#patch)
   * [Networking details](#networking-details)
      + [Additional request headers](#additional-request-headers)
      + [Response info](#response-info)
      + [Manual requests](#manual-requests)
      + [Re-exports](#re-exports)
- [Limitations](#limitations)
   * [Correspondents creation requires read permissions](#correspondents-creation-requires-read-permissions)
   * [Workflow trigger patching requires read permissions](#workflow-trigger-patching-requires-read-permissions)
- [Remaining work](#remaining-work)
   * [Library design](#library-design)
      + [Service references](#service-references)
      + [Service commands](#service-commands)
      + [`reqwest` middleware](#reqwest-middleware)
      + [Ties to `tokio`](#ties-to-tokio)
      + [`From` / `Into` traits and ownership](#from-into-traits-and-ownership)
      + [Provide mocks](#provide-mocks)
   * [Missing features](#missing-features)
   * [Default values](#default-values)

----

## Overview

The API of this library is divided into a series of services, that map almost
one-to-one with the corresponding path segment of the underlying REST API: the
`Config` service maps to `/api/config/`, `Users` maps to `/api/users/`, and so
on. Each service is a distinct [async
trait](https://docs.rs/async-trait/latest/async_trait/), all implemented by a
given `Client`. This allows users to restrict a piece of code to a given subset
of the API, and it makes it easier to then implement mocks: a given mock only
needs to implement its relevant subset of the API.

```rust
use paper_plane::{auth::Auth, services};
use paper_plane::clients::reqwest::lite::Client;

// application services
struct Services {
    // access to the "Documents" API
    paperless: Box<dyn services::Documents>,
}

fn get_prod_services() -> Services {
    let url = std::env::var("PAPERLESS_URL").unwrap();
    let tok = std::env::var("PAPERLESS_TOKEN").unwrap();
    let paperless_client = Client::new(url, Auth::Token(tok.into()));

    Services {
        paperless: Box::new(paperless_client),
    }
}

struct MockService {}

impl services::Documents for MockService {
    // TODO
}

fn get_test_services() -> Services {
    Services {
        paperless: Box::new(MockService::new()),
    }
}
```

### Service API

Most services implement the following functions for the `Item` model they
operate on (for instance, in the `Users` service, this `Item` is `User`):

```rust
async fn list    (&self, params: &List  ) -> Result<Response<Paginated<Item>>>;
async fn create  (&self, body:   &Create) -> Result<Response<Item>>;
async fn retrieve(&self, id:     i32    ) -> Result<Response<Item>>;
async fn patch   (&self, body:   &Patch ) -> Result<Response<Item>>;
async fn destroy (&self, id:     i32    ) -> Result<Response<()>>;

async fn previous_page(
    &self,
    current: &Paginated<Item>,
) -> Result<Option<Response<Paginated<Item>>>>;
async fn next_page(
    &self,
    current: &Paginated<Item>,
) -> Result<Option<Response<Paginated<Item>>>>;
```

The arguments to those services are all builders defined in the corresponding
module of `paper_plane::schema::api`. For instance, to create a new document
type, one can do the following:

```rust
use paper_plane::schema::api;
use paper_plane::schema::model;

async fn new_doc_type(client: &Client) -> Result<()> {
    client
        .document_types()
        .create(
            &api::document_types::create("bank statement")
                .matching_algorithm(model::MatchingAlgorithm::AnyWord)
                .matches("bankname".to_string())
                .is_insensitive(true),
        )
        .await?;
}
```

The argument isn't consumed by the action and can be reused.

```rust
async fn reassign(client: &Client, docs: &mut [Document], new_owner: i32) -> Result<()> {
    let patch = api::documents::patch().owner(new_owner);
    for doc in docs {
        client.documents().patch(doc, &patch).await?;
    }
}
```

### Pagination

In most services, the `list` function returns a paginated version of the
result. The resulting `Paginated` struct can be given as an argument to that
service's `previous_page` and `next_page` functions to fetch the corresponding
pages using the same parameters as the original `list` call.

```rust
async fn print_all_share_links(client: &Client) -> Result<()> {
    let mut current_page = client
        .share_links()
        .list(api::share_links::list())
        .await?
        .value;
    loop {
        for link in &current_page.results {
            println!("{link:?}");
        }
        match client.share_links().next_page(&current_page).await?.value {
            Some(next_page) => current_page = next_page,
            None => break,
        }
    }
}
```

### Making changes

The API only supports applying patches, partial updates. On success, the
modified item is being returned.

```rust
async fn disable_workflow(client: &Client, id: i32) -> Result<()> {
    let mut workflow = client().workflows().retrieve(id).await?.value;
    if workflow.enabled {
        workflow = client()
            .workflows()
            .patch(&api::workflows::patch().enabled(false))
            .await?;
    }
}
```

### Network implementation

All of the network layer is behind a trait: `Client`.

While `paper_plane` abstracts away the network layer, the interface it provides
is "leaky" on purpose, allowing users to gain access to the relevant
implementation details.

#### Additional request headers

The client maintains a list of additional headers to add to each request. For
instance, if you want to provide a custom `User-Agent`, you can initialize the
client like so:

```rust
let client = paper_plane::Client::with_headers(
    paperless_url,
    paperless_auth,
    vec![("User-Agent".to_string(), "my-client".to_string())],
);
```

#### Response info

Each API call's response is wrapped in a `Response` type that includes more
information besides the parsed result: it contains a copy of all the response
headers and the status code.

#### Manual requests

All service functions are implemented using functions of the client, meaning
that it is always possible to bypass the services and manually implement a
specific call, if needed. For instance, the `Users` services's `create` function
is implemented like this:

```rust
async fn create(&self, body: &Create) -> Result<Response<Item>> {
    let path = "/api/users/";
    let req: reqwest::Reqwest =
        self.build_request(Method::POST, path, params::NONE, body::json(body))?;
    let resp: request::Response =
        self.send_request(req).await?;
    Self::decode_json(resp).await
}
```

#### Re-exports

Several dependencies of this library are re-rexported under `paper_plane:re`,
such as `reqwest`: this allows you to have full access to the specific version
used by `paper_plane`, even if your project depends on another version.

----

## Limitations

Small issues in the underlying REST API result in some limitation on this library's
side. A list of all issues in the API can be found here (TODO).

### Correspondents creation requires read permissions

This library aims at always keeping a complete version of the model client-side
to avoid `update` accidentally deleting fields. The `/api/correspondents/`
endpoint, however, does not return a full model on creation, missing several of
the model's required fields. As a workaround, the `create` function issues a
`retrieve` immediately afterwards to get the full object.

### Workflow trigger patching requires read permissions

For similar reasons, patching a workflow cannot be done without read access: the
endpoint for a call to `PATCH /api/workflow_triggers/{id}/` performs the same
checks as a call to `create` or `update`, and therefore rejects a patch that
doesn't contain all the required fields, or even fails with a 500. As a
workaround, the library inserts all those fields in a call to patch, but this
requires having a local copy of the model in the first place.

----

## Remaining work

This library is currently a work in progress. This section details all the work
that remains before a first version can be released. See also the
[Limitations](#limitations) section.

### Library design

The library's design has not been finalized yet; this section details some of
the choices being questioned and changes being considered.

#### Service references

At time of writing, the API uses Rust references to ensure that the local model
does not deviate from the remote one. There are two areas where this is an issue:

- `patch` functions require a mutable reference to the model, in order to apply
  the mutation to the local object if the network request succeeds; this makes
  it impossible to use the `patch` API if the user doesn't also have `read`
  access to the model in the first place.
- `destroy` functions take ownership of the object being destroyed, in order to
  prevent uses of the local model. if a network issue happens, the local model
  is gone while the remote one is still there.

It might be preferable to relax this API and have `patch` and `destroy` take the
object's id as an argument instead. It moves some of the responsibility of
maintaining the local models to the user, but would improve both the reliability
and the versatility of the library.

However, the `Workflow` service might retain some of this
reference-based API, just to always ensure that actions and triggers
are always created / updated in the context of their workflow, and not
in isolation.

#### Service commands

At the moment, there's a bit of repetition between the functions of a service
and their arguments: a call to `service.create` requires giving it a
`api::service::Create` argument. Would it be better for each service to just
take a `api::service::Command` instead, and to provide only one function named
`execute`?

For comparison:

```rust
async fn current(client: &Client) -> Result<()> {
    client
        .document_types()
        .create(&api::document_types::create("bank statement"))
        .await?;
}

async fn proposed(client: &Client) -> Result<()> {
    client
        .document_types()
        .execute(&api::document_types::create("bank statement"))
        .await?;
}
```

The downside is that it would make the documentation a lot less readable: since
each operation returns a different type, this would require a trait for each
service, and would probably mean making it more difficult to implement mocks.

#### `reqwest` middleware

There is no way at the moment for a user to use
[`reqwest-middleware`](https://crates.io/crates/reqwest-middleware) to modify
reqwest's behaviour. It might be worth looking into it: would it be enough to
offer an alternative `Client` constructor?

More generally, what can be done to allow customisation of the network layer?
Perhaps implement a `Client` trait and make the current implementation of the
services work for any such `Client`?

#### Ties to `tokio`

Can the library be used with another runtime library such as
[`smol`](https://crates.io/crates/smol), or do some of the implementation
details bind it to `tokio`?

#### `From` / `Into` traits and ownership

A lot of the structures use `String` for ownership, including inherently
temporary structures, such as the `api::*::Create` structs. The reasoning behind
this is that they're re-usable, and serializable: a user could read a series of
commands from a file and apply them. But as a result, the code is littered with
calls to `clone()` or `to_string()`. It would be worth looking into:

- making use of `Into<String>` in function arguments, to be a bit more liberal in what we accept,
- making use of `Cow`, to reduce unnecessary cloning?

#### Provide mocks

Would there be value in shipping mocks for all services, or would a default
implementation be unlikely to be useful and therefore dead code.

### Missing features

Not all the API is implemented yet, and the parts that are implemented aren't
all tested yet. The following table keeps track of what has been done and what
remains.

| service | corresponding api path | implementation status | test status |
| ------- | ---------------------- | --------------------- | ----------- |
| Bulk | `/api/bulk_edit_objects` <br> `/api/documents/bulk_edit` <br> `/api/documents/bulk_download` | :x: | :x: |
| System | `/api/logs/*` <br> `/api/remote_version/` <br> `/api/statistics/`<br> `/api/status/` | :construction: | :x: |
| Auth | `/api/oauth/callback` <br> `/api/token` <br> `/api/profile/*` | :construction: | :x: |
| Upload | `/api/documents/post_document` | :x: | :x: |
| Notes | `/api/documents/notes/*` | :x: | :x: |
| Email | `/api/documents/email/` | :x: | :x: |
| Config | `/api/config/*` | :white_check_mark: | :x: |
| Correspondents | `/api/correspondents/*` | :white_check_mark: | :white_check_mark: |
| CustomFields | `/api/custom_fields/*` | :white_check_mark: | :x: |
| DocumentTypes | `/api/document_types/*` | :white_check_mark: | :x: |
| Documents | `/api/documents/*` | :construction: | :x: |
| Groups | `/api/groups/*` | :white_check_mark: | :white_check_mark: |
| MailAccounts | `/api/mail_accounts/*` | :white_check_mark: | :x: |
| MailRules | `/api/mail_rules/*` | :white_check_mark: | :x: |
| Profile | `/api/profile/` | :white_check_mark: | :x: |
| SavedViews | `/api/saved_views/*` | :white_check_mark: | :x: |
| Search | `/api/search/*` | :x: | :x: |
| ShareLinks | `/api/share_links/*` | :white_check_mark: | :x: |
| StoragePaths | `/api/storage_paths/*` | :white_check_mark: | :x: |
| Tags | `/api/tags/*` | :white_check_mark: | :x: |
| Trash | `/api/trash/*` | :x: | :x: |
| UiSettings | `/api/ui_settings/*` | :x: | :x: |
| Users | `/api/users/*` | :white_check_mark: | :construction: |
| Workflows | `/api/workflows/*` <br> `/api/workflow_triggers/` <br> `/api/workflow_actions/` | :white_check_mark: | :x: |

### Default values

Due to few fields in the models being marked as `required`, the model is
currently full of `Option` fields, making it unwieldy to use. For all fields for
which there is a reasonable default, those could be used to reduce the amount of
`Option` in the schema. This table keeps track of which of those model types
have been "cleaned" this way yet.

| model | cleaned of unnecessary options |
| ----- | ------------------------------ |
| Actor | :x: |
| ApplicationConfiguration | :white_check_mark: |
| BasicUser | :x: |
| Correspondent | :white_check_mark: |
| CustomField | :x: |
| CustomFieldInstance | :x: |
| DocumentMetadata | :x: |
| Document | :x: |
| DocumentType | :x: |
| Group | :x: |
| LogEntry | :x: |
| MailAccount | :x: |
| MailRule | :x: |
| Note | :x: |
| Paginated | :x: |
| Permissions | :x: |
| PermissionsView | :x: |
| Profile | :x: |
| SavedViewFilterRule | :x: |
| SavedView | :x: |
| ShareLink | :x: |
| SocialAccount | :x: |
| StoragePath | :x: |
| Suggestions | :x: |
| Tag | :x: |
| TaskView | :x: |
| User | :x: |
| WorkflowActionEmail | :x: |
| WorkflowAction | :x: |
| WorkflowActionWebhook | :x: |
| Workflow | :x: |
| WorkflowTrigger | :x: |
