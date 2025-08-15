use std::collections::HashMap;

#[derive(PartialEq,Clone,Copy)]
pub enum ActionStatus {
    OK = 0,
    WARRNING = 1,
    ERROR = 2
}

pub struct ActionDetail {
    pub status : ActionStatus,
    pub code : u64,
    pub description : String
}

pub struct ActionResult {
    pub details : Vec<ActionDetail>
}

impl ActionResult {
    fn status(&self) -> ActionStatus {
        let statuses : Vec<ActionStatus> = self.details.iter().map(|d| {d.status}).collect();
        if statuses.contains(&ActionStatus::ERROR) { return ActionStatus::ERROR; }
        if statuses.contains(&ActionStatus::WARRNING) { return ActionStatus::WARRNING; }
        return ActionStatus::OK;
    }
}

pub trait IUniversalService {
    async fn check_action(&self, executor_id : &str, action_name : &str, id : Option<&str>, params : Option<HashMap<String, String>>) -> Option<bool>;
    async fn execute_action(&self, action_name : &str, id : Option<&str>, params : Option<HashMap<String, String>>) -> ActionResult;

}