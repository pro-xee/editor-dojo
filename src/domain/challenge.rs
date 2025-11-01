/// Represents an editing challenge in the dojo
///
/// This is a pure domain entity with no external dependencies.
/// It contains all the information needed to present and validate a challenge.
#[derive(Debug, Clone)]
pub struct Challenge {
    title: String,
    description: String,
    starting_content: String,
    target_content: String,
    hint: String,
}

impl Challenge {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        starting_content: impl Into<String>,
        target_content: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            starting_content: starting_content.into(),
            target_content: target_content.into(),
            hint: hint.into(),
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn starting_content(&self) -> &str {
        &self.starting_content
    }

    pub fn target_content(&self) -> &str {
        &self.target_content
    }

    pub fn hint(&self) -> &str {
        &self.hint
    }
}
