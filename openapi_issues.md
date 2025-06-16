#### TOC

[TOC]

## :rotating_light: CRASHES

In those issues, a valid use of the API (as descried by the schema) result in an exception being raised in the server, which in turn means that the request fails with a 500 and that raw HTML output is returned to the user.

### `POST /api/config/` can't be used more than once

```
$ ./curl POST '/api/config/' -d '{"user_args": "null", "barcode_tag_mapping": "null"}' | jq 'del(.. | select(. == null))'
{
  "id": 1
}

$ ./curl POST '/api/config/' -d '{"user_args": "null", "barcode_tag_mapping": "null"}'
<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

Running the same valid POST query twice results in a 500 in the server, because the config gets assigned the same id.

```
django.db.utils.IntegrityError: UNIQUE constraint failed: paperless_applicationconfiguration.id
```

It doesn't seem to be possible to create more than one instance of the configuration? Even when switching to another user, no POST request succeeds after the first.

<ins>_Suggestion:_</ins> if this is intentional, because the config must be unique, then maybe the API could be reduced to just GET and PATCH on `/api/config/`, and a default configuration (with `null`) be created at launch if it doesn't exist? Having a per-id API and endpoints to create and list the configuration when it must be unique is quite misleading.

### `POST /api/mail_rules/` requires optional fields

The `MailRuleRequest` schema only lists two required fields: `name` and `account`. But only providing them results in a 500.

```
$ ./curl POST '/api/mail_rules/' -d '{"name": "foo", "account": 1}'
<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

This is because the `action` field is assumed present despite being optional.

```
  File "/usr/src/paperless/src/paperless_mail/serialisers.py", line 120, in validate
    attrs["action"] == MailRule.MailAction.TAG
    ~~~~~^^^^^^^^^^
KeyError: 'action'
```

But providing it isn't enough:

```
$ ./curl POST '/api/mail_rules/' -d '{"name": "foo", "account": 1, "action": 1}'
<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

Because `assign_tags` is also assumed to be present:

```
  File "/usr/src/paperless/src/paperless_mail/serialisers.py", line 114, in create
    if assign_tags:
       ^^^^^^^^^^^
UnboundLocalError: cannot access local variable 'assign_tags' where it is not associated with a value
```

Providing both optional fields succeeds.

### `POST /api/saved_views` doesn't handle permissions

The `SavedViewRequest` model lists `set_permissions` as a valid optional field, but using it results in a crash.

```
$ ./curl POST '/api/saved_views/' -d '{"name": "sv1", "show_on_dashboard": true, "show_in_sidebar": true, "filter_rules": []}' | jq 'del(.. | select(. == null))'
{
  "id": 6,
  "name": "sv1",
  "show_on_dashboard": true,
  "show_in_sidebar": true,
  "sort_reverse": false,
  "filter_rules": [],
  "owner": 3,
  "user_can_change": true
}

$ ./curl POST '/api/saved_views/' -d '{"name": "sv2", "show_on_dashboard": true, "show_in_sidebar": true, "filter_rules": [], "set_permissions": {"view": {"users": [3]}}}'

<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

The stack trace suggests that the error isn't with the validation of the request, but in the creation of the model:

```
  File "/usr/src/paperless/src/documents/serialisers.py", line 1242, in create
    saved_view = SavedView.objects.create(**validated_data)
                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/local/lib/python3.12/site-packages/django/db/models/manager.py", line 87, in manager_method
    return getattr(self.get_queryset(), name)(*args, **kwargs)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/local/lib/python3.12/site-packages/django/db/models/query.py", line 677, in create
    obj = self.model(**kwargs)
          ^^^^^^^^^^^^^^^^^^^^
  File "/usr/local/lib/python3.12/site-packages/django/db/models/base.py", line 567, in __init__
    raise TypeError(
TypeError: SavedView() got unexpected keyword arguments: 'set_permissions'
```

### `PATCH /api/workflow_triggers/id/`requires optional fields

For all endpoints, patch with a body of `{}` should be a valid operation, as all fields in the `Patched*Request` models are optional. However, for worfklow triggers, the following results in a 500:

```
$ ./curl PATCH '/api/workflow_triggers/1/' -d '{}'
<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

This crash occurs because the code expects an optional field, `type`:

```
  File "/usr/local/lib/python3.12/site-packages/rest_framework/mixins.py", line 82, in partial_update
    return self.update(request, *args, **kwargs)
           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/local/lib/python3.12/site-packages/rest_framework/mixins.py", line 67, in update
    serializer.is_valid(raise_exception=True)
  File "/usr/local/lib/python3.12/site-packages/rest_framework/serializers.py", line 225, in is_valid
    self._validated_data = self.run_validation(self.initial_data)
                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/local/lib/python3.12/site-packages/rest_framework/serializers.py", line 447, in run_validation
    value = self.validate(value)
            ^^^^^^^^^^^^^^^^^^^^
  File "/usr/src/paperless/src/documents/serialisers.py", line 2024, in validate
    attrs["type"] == WorkflowTrigger.WorkflowTriggerType.CONSUMPTION
    ~~~~~^^^^^^^^
KeyError: 'type'
```

The backtrace suggests that the input is validated as if it were a `WorkflowTriggerRequest` and not a `PatchedWorkflowTriggerRequest`.

### `POST /api/ui_settings/`: invalid body crashes the server's homepage

The schema for `/api/ui_settings` is incorrect and leads to a crash that prevents the entire UI from loading. The schema is described as follows (unrelated field omitted for brevity).

```yaml
/api/ui_settings/:
  get:
    responses:
      '200':
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/UiSettingsView'
  post:
    requestBody:
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/UiSettingsViewRequest'
        application/x-www-form-urlencoded:
          schema:
            $ref: '#/components/schemas/UiSettingsViewRequest'
        multipart/form-data:
          schema:
            $ref: '#/components/schemas/UiSettingsViewRequest'
    responses:
      '200':
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/UiSettingsView'
```

The corresponding models are as follows.

```yaml
UiSettingsView:
  type: object
  properties:
    id:
      type: integer
      readOnly: true
    settings:
      nullable: true
  required:
  - id
UiSettingsViewRequest:
  type: object
  properties:
    settings:
      nullable: true
```

None of this is accurate.

The response to a `GET` is, instead, along those lines (deep fields elided for brevity).

```
{
  "user": { ... },
  "settings": {
    "update_checking": {
      "backend_setting": "default"
    },
    "trash_delay": 30,
    "app_title": null,
    "app_logo": null,
    "auditlog_enabled": true,
    "email_enabled": false
  "permissions": [ ... ]
}
```

As far as `POST` is concerned, the `settings` field doesn't have a type, which tools like the schema view interpret as a string by default. Passing a random string *works*, and then *prevents the server from loading the homepage*.

```
$ ./curl POST '/api/ui_settings/' -d '{"settings": "ldsfjdsfj"}' | jq .
{
  "success": true
}

```

(Note that the response to the `POST` is not the same as `GET`, despite also being listed as a `UiSettingsView` in the schema.)

After running said previous content, the homepage no longer loads, and a GET of `/api/ui_settings/` fails.

```
$ ./curl GET '/api/ui_settings/'

<!doctype html>
<html lang="en">
<head>
  <title>Server Error (500)</title>
</head>
<body>
  <h1>Server Error (500)</h1><p></p>
</body>
</html>
```

The logs reveal that this random string should not have been accepted:

```
[2025-06-16 16:20:01,312] [ERROR] [django.request] Internal Server Error: /api/ui_settings/
Traceback (most recent call last):
  [...]
  File "/usr/local/lib/python3.12/site-packages/rest_framework/views.py", line 512, in dispatch
    response = handler(request, *args, **kwargs)
               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/usr/src/paperless/src/documents/views.py", line 2122, in get
    ui_settings["update_checking"] = {
    ~~~~~~~~~~~^^^^^^^^^^^^^^^^^^^
TypeError: 'str' object does not support item assignment
```

Past this point, to use the UI, the settings must be reset to a JSON object, like so for instance:

```
$ ./curl POST '/api/ui_settings/' -d '{"settings": {}}' | jq .
{
  "success": true
}
```

## :x: INCORRECT

Those issues describe places in the schema where a piece of information is *incorrect*, such as a request body type pointing to the wrong model, or a field labelled as optional when it is required... all issues that do not result in a crash, but that make it difficult to use the API without trial and error (or reading the code).

### Missing required fields in newly created models

This issue impacts (at least) the following endpoints:
  - `POST /api/correspondents/`
  - `POST /api/custom_fields/`
  - `POST /api/document_types/`

In many `POST` endpoints, that take a `{Model}Request` and return a `{Model}`, required fields are missing, such as `document_count`. For instance, in the `Correspondent` model, the following fields are missing:
  - `document_count`
  - `last_correspondence`
  - `permissions` (or `user_can_change`, see further issue)

```
$ ./curl POST '/api/correspondent/' -d '{"name": "a"} | jq .
{
  "id": 3,
  "slug": "a",
  "name": "a",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "owner": 3,
  "user_can_change": true
}
```

In `custom_fields`, it's `document_count`, in `document_types` it's `document_count` and `permissions` (or `user_can_change`). I haven't compiled a full list, but can do so if that's useful.

### Incorrect nullability for the `last_correspondence` field in `Correspondent`

It is not declared as nullable.

```yaml
Correspondent:
  type: object
  properties:
    last_correspondence:
      type: string
      format: date
      readOnly: true
```

But the value of that field is `null` by default.

```
$ ./curl GET '/api/correspondent/3/' | jq .
{
  "id": 3,
  "slug": "a",
  "name": "a",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "document_count": 0,
  "last_correspondence": null,
  "owner": 3,
  "user_can_change": true
}
```

### `permissions` and `user_can_change` are required but mutually exclusive

This issue impacts the following models:
  - `Correspondent`
  - `Document`
  - `DocumentType`
  - `MailAccount`
  - `MailRule`
  - `SavedView`
  - `StoragePath`
  - `Tag`

In all of those models, one or both of those fields is labelled as required, but they are mutually exclusive: depending on whether the `full_perms` argument is non-null or not, one field is included, but not the other.

```
$ ./curl GET '/api/correspondents/3/' | jq .
{
  "id": 3,
  "slug": "a",
  "name": "a",
  "match": "dhkjdhf",
  "matching_algorithm": 2,
  "is_insensitive": true,
  "document_count": 0,
  "last_correspondence": null,
  "owner": null,
  "user_can_change": true
}

$ ./curl GET '/api/correspondents/3/?full_perms=sfhjdh' | jq .
{
  "id": 3,
  "slug": "a",
  "name": "a",
  "match": "dhkjdhf",
  "matching_algorithm": 2,
  "is_insensitive": true,
  "document_count": 0,
  "last_correspondence": null,
  "owner": null,
  "permissions": {
    "view": {
      "users": [],
      "groups": []
    },
    "change": {
      "users": [],
      "groups": []
    }
  }
}
```

Possible solutions to this would be:
  - mark both as optional;
  - mark both as required and always send both, deprecating the `full_perms` argument;
  - merge them in one `permissions` field whose type is `oneOf` of the two.

(I am personally biased towards that second option: it makes client-side models a lot more consistent if fields aren't conditional.)

### `full_perms` is listed as a boolean, but is interpreted as a string

This issue impacts at least the following endpoints:
  - most endpoints under `/api/correspondent/`
  - most endpoints under `/api/document/`
  - most endpoints under `/api/document_types/`
  - most endpoints under `/api/mail_accounts/`
  - most endpoints under `/api/mail_rules/`
  - most endpoints under `/api/saved_views/`
  - most endpoints under `/api/storage_paths/`
  - most endpoints under `/api/tags/`

In all endpoints that accept `full_perms` as an argument, even endpoints in which it is not declared as part of the schema, `full_perms` is not interpreted as a boolean, but as a string: its truthiness is decided on whether it's non-null or not. As a result, `full_perms=false` is the same as `full_perms=true`.

```
$ ./curl GET '/api/document_types/4/?full_perms=false' | jq .
{
  "id": 4,
  "slug": "q",
  "name": "q",
  "match": "",
  "matching_algorithm": 2,
  "is_insensitive": false,
  "document_count": 0,
  "owner": null,
  "permissions": {
    "view": {
      "users": [],
      "groups": []
    },
    "change": {
      "users": [],
      "groups": []
    }
  }
}
```

### Wrong schema for `POST /api/users/deactivate_totp`

The schema for this endpoint is described as follows (irrelevant fields omitted for brevity).

```yaml
/api/users/{id}/deactivate_totp/:
  post:
    requestBody:
      content:
        application/json:
          schema:
            $ref: '#/components/schemas/UserRequest'
        application/x-www-form-urlencoded:
          schema:
            $ref: '#/components/schemas/UserRequest'
        multipart/form-data:
          schema:
            $ref: '#/components/schemas/UserRequest'
      required: true
    responses:
      '200':
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/User'
```

This is incorrect. It looks like this endpoint doesn't require a request body, but i haven't been able through testing to figure out what it requires, and what it might return on success.

## :no_entry_sign: MISSING

Those issues describe places in the schema where a piece of information is *missing*. Similarly to the previous set, it makes using the API more difficult, as crucial information may not be available in the schema.

### Fields `user_args` and `barcode_tag_mapping` have no type

In all three models `ApplicationConfiguration`, `ApplicationConfigurationRequest` and `PatchedApplicationConfigurationRequest`, those items are listed without a type, as if they could be any JSON value. However, when validating a request body of type `ApplicationConfigurationRequest` or `PatchedApplicationConfigurationRequest`, those two fields are expected to be a string that can be parsed as a valid JSON value.

```
$ ./curl POST '/api/config/' -d '{"user_args": {}, "barcode_tag_mapping": {}}' | jq .
{
  "user_args": [
    "Value must be valid JSON."
  ],
  "barcode_tag_mapping": [
    "Value must be valid JSON."
  ]
}
```

Supplying a valid JSON value still results in that field being serialized as a string in the `ApplicationConfiguration` model:

```
$ ./curl GET '/api/config/1/' | jq 'del(.. | select(. == null))'
{
  "id": 1,
  "user_args": "{\"foo\": 3}",
  "barcode_tag_mapping": "{\"foo\": 3}"
}
```

<ins>_Suggestion:_</ins> if the intent is to accept any JSON value, why not remove the layer of indirection, and avoid a string?

### `full_perms` is silently accepted on `POST`, `PATCH`, and `PUT` (and some `GET`)

This issue impacts the following endpoints:
  - most endpoints under `/api/correspondent/`
  - most endpoints under `/api/document/`
  - most endpoints under `/api/document_types/`
  - most endpoints under `/api/mail_accounts/`
  - most endpoints under `/api/mail_rules/`
  - most endpoints under `/api/saved_views/`
  - most endpoints under `/api/storage_paths/`
  - most endpoints under `/api/tags/`

In all the aforementioned paths, all endpoints but DELETE return a model that contains either `user_can_change` or `permissions`. For most of them (except `mail_accounts`, `mail_rules`, `saved_views`), the GET requests list `full_perms` as a valid query parameter. But all of them, regardless of the method, treat `full_perms` as a valid parameter:

```
$ ./curl POST '/api/correspondents/?full_perms=jsflshf' -d '{"name": "e"}' | jq .
{
  "id": 6,
  "slug": "e",
  "name": "e",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "owner": 3,
  "permissions": {
    "view": {
      "users": [],
      "groups": []
    },
    "change": {
      "users": [],
      "groups": []
    }
  }
}
```

### `StoragePath` has an undocumented `permissions` field

This model doesn't declare a `permissions` field, but it can nonetheless be requested with `full_perms`.

```
$ ./curl GET '/api/storage_paths/1/?full_perms=adsflkj' | jq .
{
  "id": 1,
  "slug": "flkdsjflksj",
  "name": "flkdsjflksj",
  "path": "/tmp/sdjfhkjdshfk/",
  "match": "",
  "matching_algorithm": 6,
  "is_insensitive": true,
  "document_count": 0,
  "owner": 3,
  "permissions": {
    "view": {
      "users": [
        3
      ],
      "groups": []
    },
    "change": {
      "users": [],
      "groups": []
    }
  }
}
```

### Missing response schema for `/api/profile/totp`

The schema does not list a schema for the responses of GET and POST, and instead lists them as an arbitrary dictionary:

```yaml
/api/profile/totp/:
  get:
    responses:
      '200':
        content:
          application/json:
            schema:
              type: object
              additionalProperties: {}
  post:
    responses:
      '200':
        content:
          application/json:
            schema:
              type: object
              additionalProperties: {}
```

It looks like the fields in the response of GET are `url`, `qr_svg`, and `secret`, but it's unclear whether this is always true. It seems to always return something even if MFA has already been set up?

### Missing response schema for `GET /api/profile/social_account_providers`

The schema does not list a schema for the response and instead lists it as an arbitrary dictionary:

```
/api/profile/social_account_providers/:
  get:
    responses:
      '200':
        content:
          application/json:
            schema:
              type: object
              additionalProperties: {}
```

It seems to be returning an empty list in my tests, i have not managed to find a way to get it to return anything else.

### Missing response schema for `GET /api/remote_version`

The schema does not list a schema for the response and instead lists it as an arbitrary dictionary:

```
/api/remote_version/:
  get:
    responses:
      '200':
        content:
          application/json:
            schema:
              type: object
              additionalProperties: {}
```

### Missing response schema for `GET /api/statistics`

The schema does not list a schema for the response and instead lists it as an arbitrary dictionary:

```
/api/statistics/:
  get:
    responses:
      '200':
        content:
          application/json:
            schema:
              type: object
              additionalProperties: {}
```

### Missing response schema for `/api/trash`

The schema claims that neither GET nor POST return a schema, but that doesn't seem to be true, at least for GET.

```yaml
/api/trash/:
  get:
    responses:
      '200':
        description: No response body
  post:
    responses:
      '200':
        description: No response body

```

## :warning: MISLEADING

The following issues regard part of the schema that, while not technically incorrect, are misleading: they don't behave according to the standards, they are redundant, and so on.

### `PUT` behaves the same as `PATCH`

This issue is present in all PUT endpoints that I have tried.

The PUT method is commonly understood to [replace the target resource with the given value](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods), as opposed to PATCH, which applies a partial update. This distinction is present in the schema, in which PUT endpoints are called `update` and take a `{Model}Request` as an argument, while PATCH endpoints are called `partial_update` and take a `Patched{Model}Request` in which all fields are optional.

The issue is that, in all endpoints I have tried, *both operations behave the same*.

On the config:

```
$ ./curl POST '/api/config/' -d '{"user_args": "3", "barcode_tag_mapping": "{}"}' | jq 'del(.. | select(. == null))'
{
  "id": 1,
  "user_args": "3",
  "barcode_tag_mapping": "{}",
}

$ ./curl PUT '/api/config/1/' -d '{"user_args": "3", "barcode_tag_mapping": "{}", "app_title": "test"}' | jq 'del(.. | select(. == null))'
{
  "id": 1,
  "user_args": "3",
  "barcode_tag_mapping": "{}",
  "app_title": "test",
}

$ ./curl PUT '/api/config/1/' -d '{"user_args": "3", "barcode_tag_mapping": "{}", "pages": 4}' | jq 'del(.. | select(. == null))'
{
  "id": 1,
  "user_args": "3",
  "barcode_tag_mapping": "{}",
  "pages": 4,
  "app_title": "test",
}
```

On document types:

```
$ ./curl PUT '/api/document_types/2/' -d '{"name": "foo", "is_insensitive": false}' | jq .
{
  "id": 2,
  "slug": "foo",
  "name": "foo",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": false,
  "document_count": 0,
  "owner": null,
  "user_can_change": true
}

$ ./curl PUT '/api/document_types/2/' -d '{"name": "foo", "matching_algorithm": 2}' | jq .
{
  "id": 2,
  "slug": "foo",
  "name": "foo",
  "match": "",
  "matching_algorithm": 2,
  "is_insensitive": false,
  "document_count": 0,
  "owner": null,
  "user_can_change": true
}

$ ./curl PUT '/api/document_types/2/' -d '{"name": "foo"}' | jq .
{
  "id": 2,
  "slug": "foo",
  "name": "foo",
  "match": "",
  "matching_algorithm": 2,
  "is_insensitive": false,
  "document_count": 0,
  "owner": null,
  "user_can_change": true
}
```

In all those cases, `PUT` behaves like `PATCH`, and does a partial update instead of replacing the value. This is *extremely misleading*, and makes `PUT` endpoints completely redundant, as they behave the exact same but with additional constraints. A user could reasonably expect a `PUT /api/{path}/{id}/` to behave the same as a `POST /api/{path}/` and use default values for optional fields, but today that isn't the case.

My suggestion here would be to simply deprecate all `PUT` endpoints, if this is intentional.

### `PATCH /api/workflow_triggers/{id}/` and `PATCH /api/workflow_actions/{id}` can create new values

In both of those endpoints, `id` is an optional field of the request body. If specified, instead of patching the item whose id is part of the query url, *a new item is created*.

```
$ ./curl PATCH '/api/workflow_actions/1/' -d '{"id": 2}' | jq .
{
  "id": 2,
  "type": 1,
  "assign_title": null,
  "assign_tags": [],
  "assign_correspondent": null,
  "assign_document_type": null,
  "assign_storage_path": null,
  "assign_owner": null,
  "assign_view_users": [],
  "assign_view_groups": [],
  "assign_change_users": [],
  "assign_change_groups": [],
  "assign_custom_fields": [],
  "assign_custom_fields_values": {},
  "remove_all_tags": false,
  "remove_tags": [],
  "remove_all_correspondents": false,
  "remove_correspondents": [],
  "remove_all_document_types": false,
  "remove_document_types": [],
  "remove_all_storage_paths": false,
  "remove_storage_paths": [],
  "remove_custom_fields": [],
  "remove_all_custom_fields": false,
  "remove_all_owners": false,
  "remove_owners": [],
  "remove_all_permissions": false,
  "remove_view_users": [],
  "remove_view_groups": [],
  "remove_change_users": [],
  "remove_change_groups": [],
  "email": null,
  "webhook": null
}
```

This is quite misleading: `PATCH` is not expected to create a new value. `id` should ideally not be allowed in those request body types. I am assuming they are there because actions and triggers are inlined within `Workflow`, and therefore `PatchedWorkflowRequest` must be able to tell which each trigger and action patch matches to. But a non-existing `id` in a patch should be rejected, and a patch with an `id` that belongs to another workflow should be rejected.

Beyond that, it might perhaps be simpler to represent actions and triggers as foreign keys in `Workflow`?


### `BlankEnum` and `NullEnum` models

This issue is about some fields within the following models:
  - `ApplicationConfiguration`
  - `ApplicationConfigurationRequest`
  - `PatchedApplicationConfigurationRequest`
  - `SavedView`
  - `SavedViewRequest`
  - `PatchedSavedViewRequest`

In the aforementioned models, some fields have a peculiar type, of the form:

```yaml
output_type:
  nullable: true
  title: Sets the output PDF type
  oneOf:
  - $ref: '#/components/schemas/OutputTypeEnum'
  - $ref: '#/components/schemas/BlankEnum'
  - $ref: '#/components/schemas/NullEnum'
```

In `ApplicationConfiguration`, this is the case of `output_type`, `mode`, `skip_archive_file`, `unpaper_clean`, `color_conversion_strategy`. This doesn't seem to be incorrect but, if my understanding is correct, it is redundant?

It looks like the intent is to accept "null entries" in addition to the values of the enum, in form of either `null` or an empty string? In which case, the `NullEnum` part is redundant due to the fields being `nullable` in the first place.

<ins>_Suggestion:_</ins> remove `NullEnum`, rename `BlankEnum` to `EmptyString`.

### introduce `PermissionClass` instead of `string`

In both the `User` and `Group` model (and their associated models such as the request ones), the type of the permissions is `string`, which would suggest that *any* string is valid. It is, however, an enum, and should therefore be something along those lines:

```yaml
PermissionClass:
  type: string
  enum:
    - add_logentry
    - change_logentry
    - delete_logentry
    - view_logentry
    - add_group
    - change_group
    - delete_group
    - view_group
    - add_user
    - change_user
    - delete_user
    - view_user
    - add_correspondent
    - change_correspondent
    - delete_correspondent
    - view_correspondent
    - add_customfield
    - change_customfield
    - delete_customfield
    - view_customfield
    - add_document
    - change_document
    - delete_document
    - view_document
    - add_documenttype
    - change_documenttype
    - delete_documenttype
    - view_documenttype
    - add_note
    - change_note
    - delete_note
    - view_note
    - add_paperlesstask
    - change_paperlesstask
    - delete_paperlesstask
    - view_paperlesstask
    - add_savedview
    - change_savedview
    - delete_savedview
    - view_savedview
    - add_sharelink
    - change_sharelink
    - delete_sharelink
    - view_sharelink
    - add_storagepath
    - change_storagepath
    - delete_storagepath
    - view_storagepath
    - add_tag
    - change_tag
    - delete_tag
    - view_tag
    - add_uisettings
    - change_uisettings
    - delete_uisettings
    - view_uisettings
    - add_workflow
    - change_workflow
    - delete_workflow
    - view_workflow
    - add_applicationconfiguration
    - change_applicationconfiguration
    - delete_applicationconfiguration
    - view_applicationconfiguration
    - add_mailaccount
    - change_mailaccount
    - delete_mailaccount
    - view_mailaccount
    - add_mailrule
    - change_mailrule
    - delete_mailrule
    - view_mailrule
```

## :speech_balloon: NOTE

The following entries are for observations that are not necessarily issues (for which i would not have opened a bug report), for issues that are trivially understood and worked around, or simply for suggestions. I am including them here for exhaustiveness and on the off chance that might prove useful.

### `user_can_change` is always set to true, even if the user doesn't have `Change` permissions

I don't know if this is intentional, and what that field is supposed to mean, but this feels counter-intuitive? The following commands were issued from a user who had the `add_correspondents` permissions, but not the `change_correspondents` one.

```
$ ./curl POST '/api/correspondents/' -d '{"name": "f"}' | jq .
{
  "id": 7,
  "slug": "f",
  "name": "f",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "owner": 4,
  "user_can_change": true
}

$ ./curl PATCH '/api/correspondents/7/' -d '{"name": "g"}' | jq .
{
  "detail": "You do not have permission to perform this action."
}

$ ./curl POST '/api/storage_paths/' -d '{"name": "foo", "path": "bar"}' | jq .
{
  "id": 3,
  "slug": "foo",
  "name": "foo",
  "path": "bar",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "owner": 4,
  "user_can_change": true
}

$ ./curl PATCH '/api/storage_paths/3/' -d '{}' | jq .
{
  "detail": "You do not have permission to perform this action."
}
```

I've replicated this issue on several endpoints, but I haven't tried them all. I am not sure if this is an error, as perhaps this has another meaning i'm not understanding?

### `all` in `Paginated{Model}List` doesn't have an inner type

The `all` field in all the `Paginated` models is declared as an `array`, but is lacking the [required `items` field](https://swagger.io/docs/specification/v3_0/data-models/data-types/#arrays) that describes the type of the elements.

While this is technically a missing type in the schema, this one is a harmless mistake: it is pretty obvious from context that the type of the element is `integer`, and that `all` is a list of all primary ids. It does mess up linters and code generators, however, and seems to make the web UI unhappy (it spins for a while trying to describe the schema).

### `number` instead of `integer`

A few places in the schema accept a `number` where they should expect an `integer`, such as the "days" and "months" search filters. As far as i can tell, a non-integer value simply gets truncated: this returns the only document created in 2025:

```
$ ./curl GET '/api/documents/?added__year=2025.99' | jq '.results |= map({id})'
{
  "count": 1,
  "next": null,
  "previous": null,
  "all": [
    1
  ],
  "results": [
    {
      "id": 1
    }
  ]
}

```

Unless this is on purpose, it would probably be better to use an integer there.

### Duplicated enums

Several enums are duplicated and could be simplified. For instance, in `GET /api/tasks/`, the `status` parameter is an inline enum, that happens to be the exact same as the `StatusEnum` model; likewise with `task_name`, which is the same as `TaskNameEnum`. This could be deduplicated.

I haven't collected a full lists of such duplicated enums, but i can do so if that's useful.

### Fields that are always created are not all labelled as required

When calling `POST` to create a new entry, the newly created value is returned. In most endpoints, several non-nullable fields of the returned model have values from the start, despite being labelled as optional. Those fields could be marked as required, since they will never be missing.

A non-exhaustive list of such fields is:

- `Correspondent`:
  * `matching_agorithm` defaults to `1`,
  * `is_insensitive` defaults to `true`,
  * `match` defaults to `""`,
- `DocumentType`:
  * `matching_agorithm` defaults to `1`,
  * `is_insensitive` defaults to `true`,
  * `match` defaults to `""`,
- `StoragePath`:
  * `matching_agorithm` defaults to `1`,
  * `is_insensitive` defaults to `true`,
  * `match` defaults to `""`,
- `Workflow`:
  * `order` defaults to `0`,
  * `enabled` defaults to `true`.
- `WorkflowTrigger`:
  * `id` is labelled as optional
  * `sources` defaults to `[1,2,3]`,
  * `matching_algorithm` defaults to `0`,
  * `match` defaults to `""`,
  * `is_insensitive` defaults to `true`,
  * `filter_has_tags` defaults to `[]`,
  * `schedule_offset_days` defaults to `0`,
  * `schedule_is_recurring` defaults to `false`,
  * `schedule_recurring_interval_days` defaults to `1`,
  * `schedule_date_field` defaults to `"added"`,
- `WorkflowAction`:
  * `id` is labelled as optional,
  * `type` defaults to `1`,

### `owner` is `nullable`

This might be intentional, but the `owner` field is nullable on multiple models, which allows for the following:

```
$ ./curl PATCH '/api/storage_paths/4/' -d '{"owner": null}' | jq '.'
{
  "id": 4,
  "slug": "dsafhjdsalf",
  "name": "dsafhjdsalf",
  "path": "lkdjflkds",
  "match": "",
  "matching_algorithm": 1,
  "is_insensitive": true,
  "document_count": 0,
  "owner": null,
  "user_can_change": true
}
```

Is this intentional?

(Also note that `user_can_change` is still `true` despite the owner being `null`.)

### Unused models

Several models are declared but are never used:
  - `NotesRequest` (probably linked to #paperless-ngx/paperless-ngx#10147 ?)
  - `BasicUserRequest` (likewise?)
  - `SocialAccountRequest`
