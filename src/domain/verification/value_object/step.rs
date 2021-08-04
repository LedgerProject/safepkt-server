use std::collections::HashMap;

pub type StepProvider = for<'a> fn(&'a str, &'a str) -> String;

#[derive(Copy, Clone)]
pub struct Step<'a> {
    name: &'a str,
    step_provider: StepProvider,
}

impl Step<'_> {
    pub fn new(name: &str, step_provider: StepProvider) -> Step {
        Step {
            name,
            step_provider,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn step_provider(&self) -> &StepProvider {
        &self.step_provider
    }
}

pub struct VerificationStepsCollection<'a> {
    steps: HashMap<String, Step<'a>>,
}

impl VerificationStepsCollection<'_> {
    pub fn new(steps: HashMap<String, Step<'_>>) -> VerificationStepsCollection {
        VerificationStepsCollection { steps }
    }

    pub fn step(&self, name: &String) -> &Step {
        &self.steps.get(name.as_str()).unwrap()
    }
}

pub struct StepInVerificationPlan<'a> {
    pub project_id: String,
    pub step: Step<'a>,
}

impl StepInVerificationPlan<'_> {
    pub fn new(project_id: String, step: Step) -> StepInVerificationPlan {
        StepInVerificationPlan { project_id, step }
    }

    pub fn project_id(&self) -> &String {
        &self.project_id
    }
    pub fn step(&self) -> &Step {
        &self.step
    }
}
