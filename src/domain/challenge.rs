/// Represents an editing challenge in the dojo
///
/// This is a pure domain entity with no external dependencies.
/// It contains all the information needed to present and validate a challenge.
#[derive(Debug, Clone)]
pub struct Challenge {
    id: String,
    title: String,
    description: String,
    starting_content: String,
    target_content: String,
    hint: String,
    difficulty: Option<String>,
    tags: Vec<String>,
}

impl Challenge {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
        starting_content: impl Into<String>,
        target_content: impl Into<String>,
        hint: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
            starting_content: starting_content.into(),
            target_content: target_content.into(),
            hint: hint.into(),
            difficulty: None,
            tags: Vec::new(),
        }
    }

    pub fn with_difficulty(mut self, difficulty: impl Into<String>) -> Self {
        self.difficulty = Some(difficulty.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
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

    pub fn difficulty(&self) -> Option<&str> {
        self.difficulty.as_deref()
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }
}
