#### TOC

<!-- TOC start (generated with https://github.com/derlin/bitdowntoc) -->

- [Nomenclature](#nomenclature)
- [Config](#config)
   * [:rotating_light: `POST /api/config/` returns a 500 if used more than once](#rotating_light-post-apiconfig-returns-a-500-if-used-more-than-once)
   * [:no_entry_sign: fields `user_args` and `barcode_tag_mapping` have no type in models `ApplicationConfiguration`, `ApplicationConfigurationRequest` and `PatchedApplicationConfigurationRequest`](#no_entry_sign-fields-user_args-and-barcode_tag_mapping-have-no-type-in-models-applicationconfiguration-applicationconfigurationrequest-and-patchedapplicationconfigurationrequest)
   * [:warning: `PUT` behaves the same as `PATCH`](#warning-put-behaves-the-same-as-patch)
   * [:warning: `BlankEnum` and `NullEnum` in models `ApplicationConfiguration` and `ApplicationConfigurationRequest` and `PatchedApplicationConfigurationRequest`](#warning-blankenum-and-nullenum-in-models-applicationconfiguration-and-applicationconfigurationrequest-and-patchedapplicationconfigurationrequest)
- [Correspondents](#correspondents)
   * [:x: the response to `POST /api/correspondents/` is missing required fields](#x-the-response-to-post-apicorrespondents-is-missing-required-fields)
   * [:x: the type of `last_correspondence` in `Correspondent` is not labelled as nullable](#x-the-type-of-last_correspondence-in-correspondent-is-not-labelled-as-nullable)
   * [:x: `permissions` and `user_can_change` are both required fields in `Correspondent`, but are mutually exclusive](#x-permissions-and-user_can_change-are-both-required-fields-in-correspondent-but-are-mutually-exclusive)
   * [:x: `full_perms` is listed as a boolean, but accepts any non-null value](#x-full_perms-is-listed-as-a-boolean-but-accepts-any-non-null-value)
   * [:no_entry_sign: `full_perms` is silently accepted on `POST`, `PATCH`, and `PUT`](#no_entry_sign-full_perms-is-silently-accepted-on-post-patch-and-put)
   * [:warning: `PUT` behaves the same as `PATCH`](#warning-put-behaves-the-same-as-patch-1)
   * [:speech_balloon: `all` in `PaginatedCorrespondentList` doesn't have a type](#speech_balloon-all-in-paginatedcorrespondentlist-doesnt-have-a-type)

<!-- TOC end -->

<!-- TOC --><a name="nomenclature"></a>
## Nomenclature

Every issue listed here is prefixed by an emoji that categorizes it according to the following table, in (more or less) descending severity.

| emoji | keyword | description |
| ----- | ------- | ------- |
| :rotating_light: | CRASH | a valid use of the API as described in the schema results in a 500 error in the server and ill-formatted output to be sent back to the user |
| :x: | INCORRECT | an attribute of the schema, such as a field's type, is incorrect, resulting in the API not working as described |
| :no_entry_sign: | MISSING | an attribute of the schema is missing, making it difficult to use the API without additional documentation |
| :warning: | MISLEADING | a part of the API is misleading, unintuitive, or redundant |
| :speech_balloon: | NOTE | smaller, less consequential issues |

<!-- TOC --><a name="config"></a>
## Config

<!-- TOC --><a name="rotating_light-post-apiconfig-returns-a-500-if-used-more-than-once"></a>
### :rotating_light: `POST /api/config/` returns a 500 if used more than once

```
$ ./curl POST '/api/config/' -d '{"user_args": "null", "barcode_tag_mapping": "null"}'
$ ./curl POST '/api/config/' -d '{"user_args": "null", "barcode_tag_mapping": "null"}'
```

Running the same valid POST query twice results in a 500 in the server, because the config gets assigned the same id.

```
django.db.utils.IntegrityError: UNIQUE constraint failed: paperless_applicationconfiguration.id
```

It doesn't seem to be possible to create more than one instance of the configuration? Even when switching to another user, no POST request succeeds after the first.

<ins>_Suggestion:_</ins> if this is intentional, because the config must be unique, then maybe the API could be reduced to just GET and PATCH on `/api/config/`, and a default configuration (with `null`) be created at launch if it doesn't exist? Having a per-id API and endpoints to create and list the configuration when it must be unique is quite misleading.

<!-- TOC --><a name="no_entry_sign-fields-user_args-and-barcode_tag_mapping-have-no-type-in-models-applicationconfiguration-applicationconfigurationrequest-and-patchedapplicationconfigurationrequest"></a>
### :no_entry_sign: fields `user_args` and `barcode_tag_mapping` have no type in models `ApplicationConfiguration`, `ApplicationConfigurationRequest` and `PatchedApplicationConfigurationRequest`

Neither of those fields have a type in the schema, but they both need to be of type string, and that string must itself contain a valid JSON value.

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

<ins>_Suggestion:_</ins> if the intent is to accept any JSON value, why not remove the layer of indirection?

<!-- TOC --><a name="warning-put-behaves-the-same-as-patch"></a>
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

<!-- TOC --><a name="warning-blankenum-and-nullenum-in-models-applicationconfiguration-and-applicationconfigurationrequest-and-patchedapplicationconfigurationrequest"></a>
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

<!-- TOC --><a name="correspondents"></a>
## Correspondents

<!-- TOC --><a name="x-the-response-to-post-apicorrespondents-is-missing-required-fields"></a>
### :x: the response to `POST /api/correspondents/` is missing required fields

The following fields are labelled as required in the `Correspondent` model, but are missing in the response:
  - `document_count`
  - `last_correspondence`
  - `permissions` (or `user_can_change`, see below)

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

<ins>_Suggestion:_</ins> several fields are always present despite not being labelled as required, such as `match`, `matching_algorithm`, `owner`, and `is_insensitive`; it could be worth labelling them as required accordingly.

<!-- TOC --><a name="x-the-type-of-last_correspondence-in-correspondent-is-not-labelled-as-nullable"></a>
### :x: the type of `last_correspondence` in `Correspondent` is not labelled as nullable

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

<!-- TOC --><a name="x-permissions-and-user_can_change-are-both-required-fields-in-correspondent-but-are-mutually-exclusive"></a>
### :x: `permissions` and `user_can_change` are both required fields in `Correspondent`, but are mutually exclusive

Depending on whether the `full_perms` argument is non-null or not, one field is included, but not the other.

```
$ ./curl GET '/api/correspondent/3/'
{
  "id": 3,
  [...]
  "owner": 3,
  "user_can_change": true

$ ./curl GET '/api/correspondent/3/full_perms=false'
{
  "id": 3,
  ...
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

Possible solutions to this would be:
- mark both as optional
- mark both as required and always send both
- merge them in one `permissions` field whose type is `oneOf` of the two

<!-- TOC --><a name="x-full_perms-is-listed-as-a-boolean-but-accepts-any-non-null-value"></a>
### :x: `full_perms` is listed as a boolean, but accepts any non-null value

As demonstrated in the previous example, `full_perms=false` returns the full permissions, because the parameter is not analyzed as a boolean: any non-empty value is considered "truthy".

<!-- TOC --><a name="no_entry_sign-full_perms-is-silently-accepted-on-post-patch-and-put"></a>
### :no_entry_sign: `full_perms` is silently accepted on `POST`, `PATCH`, and `PUT`

Despite not being indicated in the schema.

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

<!-- TOC --><a name="warning-put-behaves-the-same-as-patch-1"></a>
### :warning: `PUT` behaves the same as `PATCH`

Similarly to [Config](#warning-put-behaves-the-same-as-patch), `PUT` is redundant.

<!-- TOC --><a name="speech_balloon-all-in-paginatedcorrespondentlist-doesnt-have-a-type"></a>
### :speech_balloon: `all` in `PaginatedCorrespondentList` doesn't have a type

Harmless mistake, as it pretty obviously is a list of all primary ids. It messes up linters and code generators, however.
