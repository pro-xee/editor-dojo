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
    progressive_hints: Vec<String>,
    optimal_solution: Option<String>,
    optimal_keystrokes: Option<u32>,
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
            progressive_hints: Vec::new(),
            optimal_solution: None,
            optimal_keystrokes: None,
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

    pub fn with_progressive_hints(mut self, hints: Vec<String>) -> Self {
        self.progressive_hints = hints;
        self
    }

    pub fn with_optimal_solution(mut self, solution: impl Into<String>, keystrokes: u32) -> Self {
        self.optimal_solution = Some(solution.into());
        self.optimal_keystrokes = Some(keystrokes);
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

    pub fn progressive_hints(&self) -> &[String] {
        &self.progressive_hints
    }

    pub fn has_progressive_hints(&self) -> bool {
        !self.progressive_hints.is_empty()
    }

    pub fn optimal_solution(&self) -> Option<&str> {
        self.optimal_solution.as_deref()
    }

    pub fn optimal_keystrokes(&self) -> Option<u32> {
        self.optimal_keystrokes
    }
}
