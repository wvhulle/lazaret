use std::time::Instant;

use gtk_scene_capture::accessible_widget_lookup::WidgetTarget;

use crate::vm_journeys::{
    desktop_environment::take_screenshot, journey_failure::JourneyFailure,
    lazaret_database::LazaretDatabase, session_widgets::SessionWidgets,
};

pub const SAMPLE_PASSWORD: &str = "sample-password";
pub const SAMPLE_HOME: &str = "/home/sample-user";

pub struct JourneyRun {
    pub widgets: SessionWidgets,
    pub database: LazaretDatabase,
    product_failures: Vec<String>,
    failure_screenshots: usize,
}

impl JourneyRun {
    pub fn start() -> Result<Self, JourneyFailure> {
        Ok(Self {
            widgets: SessionWidgets::connect()?,
            database: LazaretDatabase::for_home(SAMPLE_HOME),
            product_failures: Vec::new(),
            failure_screenshots: 0,
        })
    }

    fn record_failure_evidence(&mut self, evidence_name: &str) {
        self.failure_screenshots += 1;
        take_screenshot(&format!("{:02}-{evidence_name}", self.failure_screenshots));
        eprintln!("visible widgets:");
        self.widgets
            .describe_shown_widgets()
            .iter()
            .for_each(|widget_description| eprintln!("  {widget_description}"));
    }

    pub fn step(
        &mut self,
        step_description: &str,
        step_actions: impl FnOnce(&mut Self) -> Result<(), JourneyFailure>,
    ) -> Result<(), JourneyFailure> {
        eprintln!("step: {step_description}");
        let step_started = Instant::now();
        let step_outcome = step_actions(self);
        match &step_outcome {
            Ok(()) => eprintln!(
                "step passed in {:.1}s: {step_description}",
                step_started.elapsed().as_secs_f64()
            ),
            Err(step_failure) => {
                eprintln!("step failed: {step_description}: {step_failure}");
                self.record_failure_evidence("step-failure");
            }
        }
        step_outcome
    }

    pub fn expect_behaviour(
        &mut self,
        behaviour_description: &str,
        behaviour_check: impl FnOnce(&mut Self) -> Result<(), JourneyFailure>,
    ) {
        if let Err(product_failure) = behaviour_check(self) {
            eprintln!(
                "product behaviour failed, continuing: {behaviour_description}: {product_failure}"
            );
            self.record_failure_evidence("product-failure");
            self.product_failures
                .push(format!("{behaviour_description}: {product_failure}"));
        }
    }

    pub fn finish(self) -> Result<(), JourneyFailure> {
        if self.product_failures.is_empty() {
            eprintln!("every product behaviour held");
            Ok(())
        } else {
            Err(JourneyFailure::ExpectationFailed(format!(
                "product behaviour failed:\n{}",
                self.product_failures.join("\n")
            )))
        }
    }

    pub fn press(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        self.widgets.press(widget_target)
    }

    pub fn await_shown(&self, widget_target: &WidgetTarget) -> Result<(), JourneyFailure> {
        self.widgets.await_shown(widget_target)
    }
}
