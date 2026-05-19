pub struct AiOverseer {
    system_prompt: String,
}

impl AiOverseer {
    pub fn new() -> Self {
        Self {
            system_prompt: "You are the W.R. Curriculum Guide. You are here to help the user master their studies. Be encouraging, patient, and provide helpful guidance whenever requested.".to_string(),
        }
    }

    pub async fn generate_feedback(&self, _query: String) -> String {
        "How can I help you with your curriculum today?".to_string()
    }
}
