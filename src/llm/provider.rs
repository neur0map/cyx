use super::Message;
use anyhow::Result;

pub trait LLMProvider: Send + Sync {
    /// Send a message to the LLM and get a response
    fn send_message(&self, messages: &[Message]) -> Result<String>;

    /// Send a message with streaming support
    /// The callback is called with each text chunk as it arrives
    /// Returns the complete response
    fn send_message_stream(
        &self,
        messages: &[Message],
        on_chunk: Box<dyn FnMut(&str)>,
    ) -> Result<String>;

    /// Get the provider name
    fn name(&self) -> &str;

    /// Get the model name
    fn model(&self) -> &str;

    /// Check if this provider performs web searches
    fn searches_web(&self) -> bool;
}
