use std::collections::HashMap;

pub type StepProvider = for<'a> fn(&'a str, &'a str, Option<&'a str>) -> String;
pub type FlagsProvider = fn() -> String;

#[derive(Copy, Clone)]
pub struct Step<'a> {
    name: &'a str,
    step_provider: StepProvider,
    flags: Option<&'a str>,
}

impl Step<'_> {
    pub fn new<'a>(name: &'a str, step_provider: StepProvider, flags: Option<&'a str>) -> Step<'a> {
        match flags {
            Some(flags) => {
                if flags.len() == 0 {
                    return Step {
                        name,
                        step_provider,
                        flags: None,
                    };
                }

                Step {
                    name,
                    step_provider,
                    flags: Some(flags),
                }
            }
            None => Step {
                name,
                step_provider,
                flags: None,
            },
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn step_provider(&self) -> &StepProvider {
        &self.step_provider
    }

    pub fn flags(&self) -> Option<&str> {
        self.flags
    }
}

pub struct VerificationStepsCollection<'a> {
    steps: HashMap<String, Step<'a>>,
}

impl VerificationStepsCollection<'_> {
    pub fn new(steps: HashMap<String, Step>) -> VerificationStepsCollection {
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
