use crate::user::User;
use crate::message::Message;

struct Channel {
    speaker: Option<User>,
    users: Vec<User>,
    current_message: Option<String>,

}

impl Channel {
    pub fn new() -> Channel {
        Channel { speaker: None, users: Vec::new(), current_message: None }
    }

    pub fn addUser(&mut self, user: User) {
        self.users.push(user);
    }

    fn appendToMessage(&mut self, current_user: &User, text: &str) -> Result<(), &str> {
        if let Some(ref speaker) = self.speaker {
            // Ensure that the user actually has the ability to append to the current message.
            if speaker == current_user {
                // Append the message to self.currentMessage
                if let Some(ref mut message) = self.current_message {
                    message.push_str(text);
                } else {
                    self.current_message = Some(text.to_string());
                }

                // Announce to all users that the speaker has appended to the current message.
                for user in &self.users {
                    user.sendMessage(Message::Append(text.to_string()));
                }
                Ok(())
                
            } else {
                Err("User not in control of typewriter")
            }
        } else {
            Err("User not in control of typewriter, but typewriter is available.")
        }
    }

    fn requestTakeControl(&mut self, requesting_user: &User) -> Result<(), &str> {
        if let Some(_) = self.speaker {
            Err("Another user is in control of the typewriter")
        } else {
            self.speaker = Some(requesting_user.clone());
            Ok(())
        }
    }
}