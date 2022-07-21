/// Commands are commands to perform an action on the channel
pub enum Command {
    RequestTakeControl,
    Append(String),
}
