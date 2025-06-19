# Paper Plane :card_index_dividers:

Paper Plane is an unofficial client for
[paperless-ngx](https://docs.paperless-ngx.com/), that provides a thin type-safe
interface above its REST API. It is a fully async library and is meant to be
used with a [tokio](https://tokio.rs) runtime.

> :warning: This is a work in progress. See the [Remaining
> work](#remaining-work) section.

#### TOC

<!-- TOC start (generated with https://github.com/derlin/bitdowntoc) -->

- [Overview](#overview)
   * [Service API](#service-api)
   * [Pagination](#pagination)
   * [Making changes](#making-changes)
   * [Network implementation](#network-implementation)
   * [Re-exports](#re-exports)
- [Limitations](#limitations)
- [Remaining work](#remaining-work)
   * [Library design](#library-design)
      + [`From` / `Into` traits and ownership](#from--into-traits-and-ownership)
      + [Provide mocks](#provide-mocks)
   * [Missing features](#missing-features)
   * [Default values](#default-values)

<!-- TOC end -->

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
use paper_plane::schema::{api, model};

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
async fn reassign(client: &Client, docs: &[Document], new_owner: i32) -> Result<()> {
    let patch = api::documents::patch().owner(new_owner);
    for doc in docs {
        client.documents().patch(doc.id, &patch).await?;
    }
}
```

### Pagination

In most services, the `list` function returns a paginated version of the
result. The resulting `Paginated` struct can be given as an argument to that
service's `previous_page` and `next_page` functions to fetch the corresponding
pages using the same parameters as the original `list` call.

```rust
async fn print_all_share_links(service: &impl services::ShareLinks) -> Result<()> {
    let mut current_page = service
        .list(api::share_links::list())
        .await?
        .value;
    loop {
        for link in &current_page.results {
            println!("{link:?}");
        }
        match service.next_page(&current_page).await?.value {
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

All of the network layer is behind a trait: `Client`. With the `reqwest` feature
enabled, the library ships with two implementations of the trait:
`clients::reqwest::Client` and `clients::reqwest::lite::Client`; the difference
between them is the `Extra` associated type: an implementation of `Client` can
choose how much extra information to return alongside the result of a
request. The "lite" client does not return anything but `()`, while the full
`reqwest` client returns headers, duration, and so on.

If you have more specific needs, such as wanting to use a [reqwest
middleware](https://crates.io/crates/reqwest-middleware/), you can use your own
implementation of `Client`: all services have a blanket implementation for all
types that implement `Client`, meaning that implementing a new `Client` is all
you need to have access to all of this library's features.

### Re-exports

Several dependencies of this library are re-rexported under `paper_plane:re`,
such as `bytes`, and `reqwest` if the corresponding feature is enabled. This
allows you to have full access to the specific version used by `paper_plane`,
even if your project depends on another version.

----

## Limitations

The design of the REST API results in limitations on this library's side, the
biggest one being that it's not truly possible to use the REST API to maintain a
*complete* copy of the models client-side: not all fields are guaranteed to be
present with every request.

For instance, regarding permissions, most of the API supports a query parameter
named `full_perms`, that decides whether the result contains a single
`user_can_change` field (the semantics of which are somewhat unclear) or a
detailed `permissions` field. It is not possible to have *both*. This library
makes the choice to always supply `full_perms` on the user's behalf to attempt
to always have the most complete version of the model.

Likewise, it is currently not feasible to provide a way to *fully* override a
model. Despite the fact that the API provides endpoints of the form `PUT
/api/{type}/{id}/`, the model that is being sent in the request body is not used
to override the existing one, but used instead to apply partial updates; meaning
that omitting fields will not reset them to their default values / unset them.

Beyond this, several endpoints cannot be implemented yet, as there is no
documentation for them and the OpenAPI schema is not always in sync with the
behaviour of the API.

As a result, this library doesn't attempt to provide rich semantics on top of
the REST API, and instead aims at being a lightweight type-safe layer on top of
it.

For more details: all issues that were found with the REST API as part of the
development of this project have been collected in [one big
issue](paperless-ngx/paperless-ngx#10195) on paperless-ngx's repo.

----

## Remaining work

This library is currently a work in progress. This section details all the work
that remains before a first version can be released. See also the
[Limitations](#limitations) section.

### Library design

The library's design has not been finalized yet; this section details some of
the choices being questioned and changes being considered.

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
