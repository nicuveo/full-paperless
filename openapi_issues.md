## Issues in the schema

The following endpoints have incorrect schema for either their parameters, their request body, or their response. This list will be collated into one big issue once all the endponts have been implemented and tested.

- `POST /api/documents/{id}/email/`: wrong request body ([original issue](https://github.com/paperless-ngx/paperless-ngx/issues/10146))
- `POST /api/documents/{id}/notes/`: wrong request body ([original issue](https://github.com/paperless-ngx/paperless-ngx/issues/10147))
- `GET /api/documents/next_asn/`: wrong response ([original issue](https://github.com/paperless-ngx/paperless-ngx/issues/10149))
- `POST /api/storage_paths/test`: wrong request body, wrong response ([original issue](https://github.com/paperless-ngx/paperless-ngx/issues/10161))
- `POST /api/users/deactivate_totp`: wrong request body, wrong response
- `GET /api/profile/totp`: missing response schema
- `GET /api/profile/social_account_providers`: missing response schema
- `POST /api/profile/totp`: missing response schema
- `GET /api/remote_version`: missing response schema
- `GET /api/statistics`: missing response schema
- `GET /api/trash`: missing response schema
- `POST /api/trash`: missing response schema
- `GET /api/ui_setttings`: incomplete schema
- `POST /api/trash`: incomplete schema
- `POST /api/users/`: use enum instead of string for permissions
- `SocialMediaRequest`: unused model
- `NotesRequest`: unused model (possibly linked to [this issue](https://github.com/paperless-ngx/paperless-ngx/issues/10147))

Crashes:
- `POST /api/mail_rules` crashes with 500 if any of the following non-required fields is missing (but the mail rule is still created!):
  - `action`
  - `assign_tags`
- `PATCH /api/workflow_triggers/{id}/` crashes with 500 if the type is missing despite it not being a required field
- `POST /api/saved_views/` crashes with 500 if the field `set_permissions` is set despite the fact that the field is present in `SavedViewRequest`. it accepts `permissions`, however, which is not supposed to be a valid field.

Peculiarities worth exploring:
- in `WorkflowTrigger` the `id` field is not labelled as required?
- in `WorkflowAction` the `id` field is not labelled as required?
- `PatchedWorkflowTriggerRequest` crashes if type is not present despite it being an optional field, and allows `id` as a field despite the fact that the API requires an id. this creates the workflow if the `id` is different from the one in the request. it also requires one of the matches despite being a patch?
- `PatchedWorkflowAction` does not crash but has the same `id` issue.
- `Paginated` types do not define the type in the list for `all`.
- `/api/mail_rules`: both the list and query are missing the `full_perms=true` parameter in the schema
- `SavedView` doesn't have a `permissions` field, but seem to be storing them nonetheless?
- the `StoragePath` model doesn't include the `permissions` field, but `GET /api/storage_fields/` and `GET /api/storage_fields/{id}/` do include it when full_perms is given. same for tags. same for mail_rules.

#### Correspondent
- `Correspondent` list several fields as required while they are in fact optional (tested in the response to create):
  - `document_count`
  - `last_correspondence`
  - `permissions`
  - `user_can_change`
- `Correspondent.last_correspondence` can contain `null` while not being declared as `nullable`
- `PUT /api/correspondents/{id}/` and `PATCH /api/correspondents/{id}` both accept `full_perms=true` as an undocumented query parameter

- `PUT /api/tags/` returns `user_can_change = true` even if the user doesn't have the change_tag permissions
