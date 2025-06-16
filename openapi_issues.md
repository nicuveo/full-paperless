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

## :x: INCORRECT

Those issues describe places in the schema where a piece of information is *incorrect*, such as a request body type pointing to the wrong model, or a field labelled as optional when it is required... all issues that do not result in a crash, but that make it difficult to use the API without trial and error (or reading the code).

### missing required fields in newly created models

This issue impacts the following endpoints:
  - `POST /api/correspondents/`
  - `POST /api/custom_fields/`
  - `POST /api/document_types/`

In many `POST` endpoints, that take a `<Model>Request` and return a `Model`, required fields are missing, such as `document_count`. For instance, in the `Correspondent` model, the following fields are missing:
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

In `custom_fields`, it's `document_count`, in `document_types` it's `document_count` and `permissions` (or `user_can_change`).

### the `last_correspondence` field in `Correspondent` is not labelled as nullable

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

### the fields `permissions` and `user_can_change` are both required but are mutually exclusive

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

## :no_entry_sign: MISSING

Those issues describe places in the schema where a piece of information is *missing*. Similarly to the previous set, it makes using the API more difficult, as crucial information may not be available in the schema.

### fields `user_args` and `barcode_tag_mapping` have no type

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

# Old

----
----
----
----



<ins>_Suggestion:_</ins> several fields are always present despite not being labelled as required, such as `match`, `matching_algorithm`, `owner`, and `is_insensitive`; it could be worth labelling them as required accordingly.


Every issue listed here is prefixed by an emoji that categorizes it according to the following table, in (more or less) descending severity.

| emoji | keyword | description |
| ----- | ------- | ------- |
| :rotating_light: | CRASH | a valid use of the API as described in the schema results in a 500 error in the server and ill-formatted output to be sent back to the user |
| :x: | INCORRECT | an attribute of the schema, such as a field's type, is incorrect, resulting in the API not working as described |
| :no_entry_sign: | MISSING | an attribute of the schema is missing, making it difficult to use the API without additional documentation |
| :warning: | MISLEADING | a part of the API is misleading, unintuitive, or redundant |
| :speech_balloon: | NOTE | smaller, less consequential issues |


## Config



### :warning: `PUT` behaves the same as `PATCH`

Consider the following set of commands (assuming there isn't a configuration in the DB already). Fields that are `null` will be omitted from the output of commands for brevity, in this example.

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

`PUT` behaved the same as a `PATCH`: it applies a diff, instead of replacing the value. But despite those two endpoints behaving the same, they do not accept the same request body: `PUT` expects an `ApplicationConfigurationRequest`, while `PATCH` expects a `PatchedApplicationConfigurationRequest`, the difference being that `PUT` has some required fields, as if it were a regular `PUT` request that replaces the object.

<ins>_Suggestion:_</ins> deprecate `PUT`.

### :warning: `BlankEnum` and `NullEnum` in models `ApplicationConfiguration` and `ApplicationConfigurationRequest` and `PatchedApplicationConfigurationRequest`

Several types in those models have a peculiar type, of the form:

```yaml
output_type:
  nullable: true
  title: Sets the output PDF type
  oneOf:
  - $ref: '#/components/schemas/OutputTypeEnum'
  - $ref: '#/components/schemas/BlankEnum'
  - $ref: '#/components/schemas/NullEnum'
```

This is the case of `output_type`, `mode`, `skip_archive_file`, `unpaper_clean`, `color_conversion_strategy`. It looks like the intent is to accept "null entries" in addition to the values of the enum, in form of either `null` or an empty string? In which case, the `NullEnum` part is redundant due to the fields being `nullable` in the first place.

## Correspondents









### :speech_balloon: `all` in `PaginatedCorrespondentList` doesn't have a type

Harmless mistake, as it pretty obviously is a list of all primary ids. It messes up linters and code generators, however, and seems to make the web UI spin for a long time.




## Custom fields


## Document types


## Other questions

### :speech_balloon: `user_can_change` is always set to true, even if the user doesn't have `Change` permissions

I don't know if this is intentional, and what that field is supposed to mean, but this feels counter-intuitive. The following commands were issued from a user who had the `add_correspondents` permissions, but not the `change_correspondents` one. This seems to be true for all endpoints that return a model with `user_can_change`, but I haven't tested all of them.

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
```

