use crate::user::User;

/// Commands are commands to perform an action on the channel
pub enum Command {
    RequestTakeControl(User),
    Append((User, String)),
    Join(User),
}
