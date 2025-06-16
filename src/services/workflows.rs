use anyhow::anyhow;

use crate::clients::Client;
use crate::error::Error;
use crate::error::Result;
use crate::response::Response;
use crate::schema::api::workflows::{Create, List, Patch, action, trigger};
use crate::schema::model::{Paginated, Workflow, WorkflowAction, WorkflowTrigger};
use crate::utils::{Method, body, params};
use async_trait::async_trait;

pub type Item = Workflow;

#[async_trait]
pub trait Workflows<E = ()> {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, E>>;
    async fn create(&self, body: &Create) -> Result<Response<Item, E>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item, E>>;
    async fn update(&self, item: &Item) -> Result<Response<Item, E>>;
    async fn patch(&self, item: &mut Item, body: Patch) -> Result<Response<(), E>>;
    async fn destroy(&self, item: &Item) -> Result<Response<(), E>>;

    async fn add_trigger(
        &self,
        workflow: &mut Item,
        trigger: &trigger::Create,
    ) -> Result<Response<i32, E>>;
    async fn update_trigger(&self, body: &WorkflowTrigger) -> Result<Response<WorkflowTrigger, E>>;
    async fn patch_trigger(
        &self,
        item: &mut WorkflowTrigger,
        body: trigger::Patch,
    ) -> Result<Response<(), E>>;
    async fn remove_trigger(
        &self,
        workflow: &mut Workflow,
        trigger: i32,
    ) -> Result<Response<(), E>>;

    async fn add_action(
        &self,
        workflow: &mut Item,
        action: &action::Create,
    ) -> Result<Response<i32, E>>;
    async fn update_action(&self, body: &WorkflowAction) -> Result<Response<WorkflowAction, E>>;
    async fn patch_action(
        &self,
        item: &mut WorkflowAction,
        body: action::Patch,
    ) -> Result<Response<(), E>>;
    async fn remove_action(&self, workflow: &mut Workflow, action: i32) -> Result<Response<(), E>>;

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, E>>>;
    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, E>>>;
}

#[async_trait]
impl<C: Client> Workflows<C::Extra> for C {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>, C::Extra>> {
        let path = "/api/workflows/";
        let resp = self.send(Method::GET, path, params, body::NONE).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item, C::Extra>> {
        let path = "/api/workflows/";
        let resp = self
            .send(Method::POST, path, params::NONE, Some(body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/workflows/{id}/");
        let resp = self
            .send(Method::GET, &path, params::NONE, body::NONE)
            .await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item, C::Extra>> {
        let path = format!("/api/workflows/{}/", body.id);
        let body = Create::from(body);
        let resp = self
            .send(Method::PUT, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, item: &mut Item, mut body: Patch) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/workflows/{}/", item.id);
        body.triggers.clone_from(&item.triggers);
        body.actions.clone_from(&item.actions);
        let resp = self
            .send(Method::PATCH, &path, params::NONE, Some(&body))
            .await?;
        let resp = Self::decode_json(resp).await?;
        Ok(resp.assign(item))
    }

    async fn destroy(&self, item: &Item) -> Result<Response<(), C::Extra>> {
        // TODO: destroy triggers and actions
        let path = format!("/api/workflows/{}/", item.id);
        let resp = self
            .send(Method::DELETE, &path, params::NONE, body::NONE)
            .await?;
        Self::ignore_content(resp)
    }

    async fn add_trigger(
        &self,
        workflow: &mut Item,
        trigger: &trigger::Create,
    ) -> Result<Response<i32, C::Extra>> {
        let path = "/api/workflow_triggers/";
        let resp = self
            .send(Method::POST, path, params::NONE, Some(trigger))
            .await?;
        let resp: Response<WorkflowTrigger, C::Extra> = Self::decode_json(resp).await?;
        let trigger_id = resp.value.id;
        workflow.triggers.push(resp.value);
        let resp = self.update(workflow).await?;
        Ok(resp.replace(trigger_id))
    }

    async fn update_trigger(
        &self,
        body: &WorkflowTrigger,
    ) -> Result<Response<WorkflowTrigger, C::Extra>> {
        let path = format!("/api/workflow_triggers/{}/", body.id);
        let body = trigger::Create::from(body);
        let resp = self
            .send(Method::PUT, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch_trigger(
        &self,
        item: &mut WorkflowTrigger,
        mut body: trigger::Patch,
    ) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/workflow_triggers/{}/", item.id);
        // paperless-ngx crashes if those fields aren't set!
        body.trigger_type = body.trigger_type.or(Some(item.trigger_type));
        body.filter_path = body.filter_path.or_else(|| item.filter_path.clone());
        body.filter_filename = body
            .filter_filename
            .or_else(|| item.filter_filename.clone());
        body.filter_mailrule = body.filter_mailrule.or(item.filter_mailrule);
        let resp = self
            .send(Method::PATCH, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn remove_trigger(
        &self,
        workflow: &mut Workflow,
        trigger: i32,
    ) -> Result<Response<(), C::Extra>> {
        let position = workflow
            .triggers
            .iter()
            .position(|t| t.id == trigger)
            .ok_or(Error::Internal {
                source: anyhow!(
                    "workflow trigger {trigger} does not belong to workflow {}",
                    workflow.id,
                ),
            })?;
        let path = format!("/api/workflow_triggers/{trigger}/");
        let resp = self
            .send(Method::DELETE, &path, params::NONE, body::NONE)
            .await?;
        workflow.triggers.swap_remove(position);
        Self::ignore_content(resp)
    }

    async fn add_action(
        &self,
        workflow: &mut Item,
        action: &action::Create,
    ) -> Result<Response<i32, C::Extra>> {
        let path = "/api/workflow_actions/";
        let resp = self
            .send(Method::POST, path, params::NONE, Some(action))
            .await?;
        let resp: Response<WorkflowAction, C::Extra> = Self::decode_json(resp).await?;
        let action_id = resp.value.id;
        workflow.actions.push(resp.value);
        let resp = self.update(workflow).await?;
        Ok(resp.replace(action_id))
    }

    async fn update_action(
        &self,
        body: &WorkflowAction,
    ) -> Result<Response<WorkflowAction, C::Extra>> {
        let path = format!("/api/workflow_actions/{}/", body.id);
        let body = action::Create::from(body);
        let resp = self
            .send(Method::PUT, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn patch_action(
        &self,
        item: &mut WorkflowAction,
        body: action::Patch,
    ) -> Result<Response<(), C::Extra>> {
        let path = format!("/api/workflow_actions/{}/", item.id);
        let resp = self
            .send(Method::PATCH, &path, params::NONE, Some(&body))
            .await?;
        Self::decode_json(resp).await
    }

    async fn remove_action(
        &self,
        workflow: &mut Workflow,
        action: i32,
    ) -> Result<Response<(), C::Extra>> {
        let position =
            workflow
                .actions
                .iter()
                .position(|t| t.id == action)
                .ok_or(Error::Internal {
                    source: anyhow!(
                        "workflow action {action} does not belong to workflow {}",
                        workflow.id,
                    ),
                })?;
        let path = format!("/api/workflow_actions/{action}/");
        let resp = self
            .send(Method::DELETE, &path, params::NONE, body::NONE)
            .await?;
        workflow.actions.swap_remove(position);
        Self::ignore_content(resp)
    }
    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, C::Extra>>> {
        C::previous_page(self, current).await
    }

    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>, C::Extra>>> {
        C::next_page(self, current).await
    }
}
