use crate::view::*;
use nddev_device_sync_adapter_io::{CancellationToken, IoError, ProcessIo, ProcessRequest};

use std::{path::PathBuf, time::Duration};

#[derive(Clone)]
pub struct Updater {
    io: ProcessIo,
    executable: PathBuf,
}

impl Updater {
    pub fn new(io: ProcessIo, executable: PathBuf) -> Result<Self, IoError> {
        if !executable.is_absolute() {
            return Err(IoError::InvalidConfiguration);
        }
        Ok(Self { io, executable })
    }

    fn request(&self, operation: &str) -> ProcessRequest {
        ProcessRequest {
            executable: self.executable.clone(),
            arguments: [operation, "--json"].into_iter().map(Into::into).collect(),
            directory: None,
            timeout: Duration::from_secs(10),
            max_output_bytes: 1_048_576,
        }
    }

    pub async fn plan(&self, cancellation: CancellationToken) -> Result<Plan, IoError> {
        let (plan, _): (Plan, _) = self
            .io
            .json(self.request("plan"), &[0], cancellation)
            .await?;
        if !valid_plan(&plan) {
            return Err(IoError::InvalidResponse);
        }
        Ok(plan)
    }

    pub async fn last_run(
        &self,
        cancellation: CancellationToken,
    ) -> Result<UpdateRunSummary, IoError> {
        let (report, _): (UpdateRunSummary, _) = self
            .io
            .json(self.request("status"), &[0], cancellation)
            .await?;
        if !valid_summary(&report) {
            return Err(IoError::InvalidResponse);
        }
        Ok(report)
    }
}

fn valid_channel(channel: &str) -> bool {
    !channel.is_empty() && channel.len() <= 32
}

fn valid_plan(plan: &Plan) -> bool {
    plan.schema == 2 && plan.actions.len() <= 16 && valid_channel(&plan.channel)
}

fn valid_summary(report: &UpdateRunSummary) -> bool {
    report.schema == 2
        && report.results.len() <= 128
        && valid_channel(&report.channel)
        && report.finished_at >= report.started_at
        && report
            .results
            .iter()
            .all(|result| !result.state.is_empty() && !result.action.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nddev_device_sync_adapter_io::NativeIo;

    fn result() -> ActionResult {
        ActionResult {
            action: "apt-observe".into(),
            ok: true,
            changed: Some(false),
            observed: true,
            detail: "ok".into(),
            exit_code: Some(0),
            state: "observed".into(),
            output_truncated: false,
        }
    }

    fn summary() -> UpdateRunSummary {
        UpdateRunSummary {
            schema: 2,
            started_at: 10,
            finished_at: 20,
            channel: "alpha".into(),
            results: vec![result()],
            catalog_sequence: None,
            bootstrap_commit: None,
        }
    }

    #[test]
    fn channel_is_required_and_bounded() {
        assert!(valid_channel("alpha"));
        assert!(!valid_channel(""));
        assert!(!valid_channel(&"x".repeat(33)));
    }

    #[test]
    fn plan_requires_schema_two_a_channel_and_bounded_actions() {
        let plan = Plan {
            schema: 2,
            channel: "alpha".into(),
            actions: vec![SystemUpdateAction::AptObserve],
        };
        assert!(valid_plan(&plan));
        let mut wrong = Plan {
            schema: 1,
            ..plan.clone()
        };
        assert!(!valid_plan(&wrong));
        wrong.schema = 2;
        wrong.channel = String::new();
        assert!(!valid_plan(&wrong));
        let mut crowded = plan;
        crowded.actions = vec![SystemUpdateAction::AptObserve; 17];
        assert!(!valid_plan(&crowded));
    }

    #[test]
    fn summary_requires_monotonic_time_and_populated_results() {
        assert!(valid_summary(&summary()));
        let mut wrong = summary();
        wrong.finished_at = 9;
        assert!(!valid_summary(&wrong));
        let mut empty_state = summary();
        empty_state.results[0].state = String::new();
        assert!(!valid_summary(&empty_state));
        let mut empty_action = summary();
        empty_action.results[0].action = String::new();
        assert!(!valid_summary(&empty_action));
        let mut old_schema = summary();
        old_schema.schema = 1;
        assert!(!valid_summary(&old_schema));
        let mut crowded = summary();
        crowded.results = vec![result(); 129];
        assert!(!valid_summary(&crowded));
    }

    #[test]
    fn constructor_requires_an_absolute_executable() {
        let io = NativeIo::new(1).unwrap().process_handle();
        assert!(matches!(
            Updater::new(io.clone(), PathBuf::from("rldyour-updater")),
            Err(IoError::InvalidConfiguration)
        ));
        assert!(Updater::new(io, PathBuf::from("/usr/bin/rldyour-updater")).is_ok());
    }

    #[test]
    fn requests_use_the_json_flag_and_bounded_output() {
        let io = NativeIo::new(1).unwrap().process_handle();
        let updater = Updater::new(io, PathBuf::from("/usr/bin/rldyour-updater")).unwrap();
        let request = updater.request("plan");
        assert_eq!(request.arguments, ["plan", "--json"]);
        assert_eq!(request.timeout, Duration::from_secs(10));
        assert_eq!(request.max_output_bytes, 1_048_576);
    }
}
