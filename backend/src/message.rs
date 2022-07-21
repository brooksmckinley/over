/// Messages that can be sent back to clients about the state of the channel
pub enum Message {
    /// Signals that the current message has been appended to
    Append(String),
    /// Signals that the typewriter has changed controllers.
    TypewriterControl(Option<String>),
}