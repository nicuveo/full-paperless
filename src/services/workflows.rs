use async_trait::async_trait;
use reqwest::Method;

use crate::body;
use crate::client::Client;
use crate::error::Error;
use crate::error::Result;
use crate::params;
use crate::response::Response;
use crate::schema::api::workflows::{Create, List, Patch, action, trigger};
use crate::schema::model::{Paginated, Workflow, WorkflowAction, WorkflowTrigger};

pub type Item = Workflow;

#[async_trait]
pub trait Workflows {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>>;
    async fn create(&self, body: &Create) -> Result<Response<Item>>;
    async fn retrieve(&self, id: i32) -> Result<Response<Item>>;
    async fn update(&self, item: &Item) -> Result<Response<Item>>;
    async fn patch(&self, item: &mut Item, body: Patch) -> Result<Response<()>>;
    async fn destroy(&self, item: Item) -> Result<Response<()>>;

    async fn add_trigger(
        &self,
        workflow: &mut Item,
        trigger: &trigger::Create,
    ) -> Result<Response<i32>>;
    async fn update_trigger(&self, body: &WorkflowTrigger) -> Result<Response<WorkflowTrigger>>;
    async fn patch_trigger(
        &self,
        item: &mut WorkflowTrigger,
        body: trigger::Patch,
    ) -> Result<Response<()>>;
    async fn remove_trigger(&self, workflow: &mut Workflow, trigger: i32) -> Result<Response<()>>;

    async fn add_action(
        &self,
        workflow: &mut Item,
        action: &action::Create,
    ) -> Result<Response<i32>>;
    async fn update_action(&self, body: &WorkflowAction) -> Result<Response<WorkflowAction>>;
    async fn patch_action(
        &self,
        item: &mut WorkflowAction,
        body: action::Patch,
    ) -> Result<Response<()>>;
    async fn remove_action(&self, workflow: &mut Workflow, action: i32) -> Result<Response<()>>;

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>>;
}

#[async_trait]
impl Workflows for Client {
    async fn list(&self, params: &List) -> Result<Response<Paginated<Item>>> {
        let path = "/api/workflows/";
        let req = self.build_request(Method::GET, path, params, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn create(&self, body: &Create) -> Result<Response<Item>> {
        let path = "/api/workflows/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn retrieve(&self, id: i32) -> Result<Response<Item>> {
        let path = format!("/api/workflows/{id}/");
        let req = self.build_request(Method::GET, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn update(&self, body: &Item) -> Result<Response<Item>> {
        let path = format!("/api/workflows/{}/", body.id);
        let body = Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch(&self, item: &mut Item, mut body: Patch) -> Result<Response<()>> {
        let path = format!("/api/workflows/{}/", item.id);
        body.triggers.clone_from(&item.triggers);
        body.actions.clone_from(&item.actions);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(&body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn destroy(&self, item: Item) -> Result<Response<()>> {
        let path = format!("/api/workflows/{}/", item.id);
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        Self::ignore_content(resp)
    }

    async fn add_trigger(
        &self,
        workflow: &mut Item,
        trigger: &trigger::Create,
    ) -> Result<Response<i32>> {
        let path = "/api/workflow_triggers/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(trigger))?;
        let resp: Response<WorkflowTrigger> =
            Self::decode_json(self.send_request(req).await?).await?;
        let trigger_id = resp.value.id;
        workflow.triggers.push(resp.value);
        let resp = self.update(workflow).await?;
        Ok(resp.replace(trigger_id))
    }

    async fn update_trigger(&self, body: &WorkflowTrigger) -> Result<Response<WorkflowTrigger>> {
        let path = format!("/api/workflow_triggers/{}/", body.id);
        let body = trigger::Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch_trigger(
        &self,
        item: &mut WorkflowTrigger,
        mut body: trigger::Patch,
    ) -> Result<Response<()>> {
        let path = format!("/api/workflow_triggers/{}/", item.id);
        // paperless-ngx crashes if those fields aren't set!
        body.trigger_type = body.trigger_type.or(Some(item.trigger_type));
        body.filter_path = body.filter_path.or_else(|| item.filter_path.clone());
        body.filter_filename = body
            .filter_filename
            .or_else(|| item.filter_filename.clone());
        body.filter_mailrule = body.filter_mailrule.or(item.filter_mailrule);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(&body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn remove_trigger(&self, workflow: &mut Workflow, trigger: i32) -> Result<Response<()>> {
        let position = workflow
            .triggers
            .iter()
            .position(|t| t.id == trigger)
            .ok_or(Error::WorkflowItem { id: trigger })?;
        let path = format!("/api/workflow_triggers/{trigger}/");
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        workflow.triggers.swap_remove(position);
        Self::ignore_content(resp)
    }

    async fn add_action(
        &self,
        workflow: &mut Item,
        action: &action::Create,
    ) -> Result<Response<i32>> {
        let path = "/api/workflow_actionss/";
        let req = self.build_request(Method::POST, path, params::NONE, body::json(action))?;
        let resp: Response<WorkflowAction> =
            Self::decode_json(self.send_request(req).await?).await?;
        let action_id = resp.value.id;
        workflow.actions.push(resp.value);
        let resp = self.update(workflow).await?;
        Ok(resp.replace(action_id))
    }

    async fn update_action(&self, body: &WorkflowAction) -> Result<Response<WorkflowAction>> {
        let path = format!("/api/workflow_actions/{}/", body.id);
        let body = action::Create::from(body);
        let req = self.build_request(Method::PUT, &path, params::NONE, body::json(&body))?;
        let resp = self.send_request(req).await?;
        Self::decode_json(resp).await
    }

    async fn patch_action(
        &self,
        item: &mut WorkflowAction,
        body: action::Patch,
    ) -> Result<Response<()>> {
        let path = format!("/api/workflow_actions/{}/", item.id);
        let req = self.build_request(Method::PATCH, &path, params::NONE, body::json(&body))?;
        let resp = Self::decode_json(self.send_request(req).await?).await?;
        Ok(resp.assign(item))
    }

    async fn remove_action(&self, workflow: &mut Workflow, action: i32) -> Result<Response<()>> {
        let position = workflow
            .actions
            .iter()
            .position(|t| t.id == action)
            .ok_or(Error::WorkflowItem { id: action })?;
        let path = format!("/api/workflow_actions/{action}/");
        let req = self.build_request(Method::DELETE, &path, params::NONE, body::none)?;
        let resp = self.send_request(req).await?;
        workflow.actions.swap_remove(position);
        Self::ignore_content(resp)
    }

    async fn previous_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::previous_page(self, current).await
    }

    async fn next_page(
        &self,
        current: &Paginated<Item>,
    ) -> Result<Option<Response<Paginated<Item>>>> {
        Client::next_page(self, current).await
    }
}
