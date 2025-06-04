use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PermissionClass {
    #[serde(rename = "add_logentry")]
    LogEntryAdd,
    #[serde(rename = "change_logentry")]
    LogEntryChange,
    #[serde(rename = "delete_logentry")]
    LogEntryDelete,
    #[serde(rename = "view_logentry")]
    LogEntryView,
    #[serde(rename = "add_group")]
    GroupAdd,
    #[serde(rename = "change_group")]
    GroupChange,
    #[serde(rename = "delete_group")]
    GroupDelete,
    #[serde(rename = "view_group")]
    GroupView,
    #[serde(rename = "add_user")]
    UserAdd,
    #[serde(rename = "change_user")]
    UserChange,
    #[serde(rename = "delete_user")]
    UserDelete,
    #[serde(rename = "view_user")]
    UserView,
    #[serde(rename = "add_correspondent")]
    CorrespondentAdd,
    #[serde(rename = "change_correspondent")]
    CorrespondentChange,
    #[serde(rename = "delete_correspondent")]
    CorrespondentDelete,
    #[serde(rename = "view_correspondent")]
    CorrespondentView,
    #[serde(rename = "add_customfield")]
    CustomFieldDdd,
    #[serde(rename = "change_customfield")]
    CustomFieldChange,
    #[serde(rename = "delete_customfield")]
    CustomFieldDelete,
    #[serde(rename = "view_customfield")]
    CustomFieldView,
    #[serde(rename = "add_document")]
    DocumentAdd,
    #[serde(rename = "change_document")]
    DocumentChange,
    #[serde(rename = "delete_document")]
    DocumentDelete,
    #[serde(rename = "view_document")]
    DocumentView,
    #[serde(rename = "add_documenttype")]
    DocumentTypeAdd,
    #[serde(rename = "change_documenttype")]
    DocumentTypeChange,
    #[serde(rename = "delete_documenttype")]
    DocumentTypeDelete,
    #[serde(rename = "view_documenttype")]
    DocumentTypeView,
    #[serde(rename = "add_note")]
    NoteAdd,
    #[serde(rename = "change_note")]
    NoteChange,
    #[serde(rename = "delete_note")]
    NoteDelete,
    #[serde(rename = "view_note")]
    NoteView,
    #[serde(rename = "add_paperlesstask")]
    PaperlessTaskAdd,
    #[serde(rename = "change_paperlesstask")]
    PaperlessTaskChange,
    #[serde(rename = "delete_paperlesstask")]
    PaperlessTaskDelete,
    #[serde(rename = "view_paperlesstask")]
    PaperlessTaskView,
    #[serde(rename = "add_savedview")]
    SavedViewAdd,
    #[serde(rename = "change_savedview")]
    SavedViewChange,
    #[serde(rename = "delete_savedview")]
    SavedViewDelete,
    #[serde(rename = "view_savedview")]
    SavedViewView,
    #[serde(rename = "add_sharelink")]
    ShareLinkAdd,
    #[serde(rename = "change_sharelink")]
    ShareLinkChange,
    #[serde(rename = "delete_sharelink")]
    ShareLinkDelete,
    #[serde(rename = "view_sharelink")]
    ShareLinkView,
    #[serde(rename = "add_storagepath")]
    StoragePathAdd,
    #[serde(rename = "change_storagepath")]
    StoragePathChange,
    #[serde(rename = "delete_storagepath")]
    StoragePathDelete,
    #[serde(rename = "view_storagepath")]
    StoragePathView,
    #[serde(rename = "add_tag")]
    TagAdd,
    #[serde(rename = "change_tag")]
    TagChange,
    #[serde(rename = "delete_tag")]
    TagDelete,
    #[serde(rename = "view_tag")]
    TagView,
    #[serde(rename = "add_uisettings")]
    UiSettingsAdd,
    #[serde(rename = "change_uisettings")]
    UiSettingsChange,
    #[serde(rename = "delete_uisettings")]
    UiSettingsDelete,
    #[serde(rename = "view_uisettings")]
    UiSettingsView,
    #[serde(rename = "add_workflow")]
    WorkflowAdd,
    #[serde(rename = "change_workflow")]
    WorkflowChange,
    #[serde(rename = "delete_workflow")]
    WorkflowDelete,
    #[serde(rename = "view_workflow")]
    WorkflowView,
    #[serde(rename = "add_applicationconfiguration")]
    ApplicationConfigurationAdd,
    #[serde(rename = "change_applicationconfiguration")]
    ApplicationConfigurationChange,
    #[serde(rename = "delete_applicationconfiguration")]
    ApplicationConfigurationDelete,
    #[serde(rename = "view_applicationconfiguration")]
    ApplicationConfigurationView,
    #[serde(rename = "add_mailaccount")]
    MailAccountAdd,
    #[serde(rename = "change_mailaccount")]
    MailAccountChange,
    #[serde(rename = "delete_mailaccount")]
    MailAccountDelete,
    #[serde(rename = "view_mailaccount")]
    MailAccountView,
    #[serde(rename = "add_mailrule")]
    MailRuleAdd,
    #[serde(rename = "change_mailrule")]
    MailRuleChange,
    #[serde(rename = "delete_mailrule")]
    MailRuleDelete,
    #[serde(rename = "view_mailrule")]
    MailRuleView,
}
