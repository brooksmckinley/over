/// Messages that can be sent back to clients about the state of the channel
#[derive(Debug, Clone)]
pub enum Message {
    /// Contains the current channel's state to allow a new client to catch up.
    ChannelState,
    /// Signals that the current message has been appended to
    Append(String),
    /// Signals that the typewriter has changed controllers.
    TypewriterControl(Option<String>),
    /// Signals that a user has joined
    Join(String),
    /// Sends back an error
    Error(String),
}
