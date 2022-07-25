use std::sync::mpsc::Receiver;

use crate::command::Command;
use crate::message::Message;
use crate::user::User;

pub struct Channel {
    speaker: Option<User>,
    users: Vec<User>,
    current_message: Option<String>,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            speaker: None,
            users: Vec::new(),
            current_message: None,
        }
    }

    pub fn command_loop(&mut self, recv: Receiver<Command>) {
        loop {
            let command = recv.recv();
            if let Ok(command) = command {
                match command {
                    Command::RequestTakeControl(user) => {
                        let take_control_result = self.request_take_control(&user);
                        if let Err(err) = take_control_result {
                            let err = err.to_owned();
                            self.handle_command_error(&user, &err);
                        }
                    }
                    Command::Append((user, text)) => {
                        let append_result = self.append_to_message(&user, &text);
                        if let Err(err) = append_result {
                            let err = err.to_owned();
                            self.handle_command_error(&user, &err);
                        }
                    }
                    Command::Join(user) => {
                        let join_result = self.add_user(user.clone());
                        if let Err(err) = join_result {
                            let err = err.to_owned();
                            self.handle_command_error(&user, &err);
                        }
                    }
                    Command::RelinquishControl(user) => {
                        let relinquish_control_result = self.relinquish(&user, false);
                        if let Err(err) = relinquish_control_result {
                            let err = err.to_owned();
                            self.handle_command_error(&user, &err);
                        }
                    }
                }
            } else if let Err(recv_error) = command {
                eprintln!(
                    "Error receiving command. Exiting commandLoop: {:?}",
                    recv_error
                );
                break;
            }
        }
    }

    fn add_user(&mut self, new_user: User) -> Result<(), &str> {
        for user in &self.users {
            if user == &new_user {
                return Err("User already exists!");
            }
        }

        let username = new_user.name.clone();

        // If we manage to successfully send the state of the channel back to the user, we're all good to add them to the users list.
        let send_state_result = new_user.send_message(self.get_current_state());
        if let Ok(()) = send_state_result {
            self.users.push(new_user);
        }

        // Broadcast to everyone else that someone new has joined.
        self.broadcast_message(Message::Join(username), None);
        Ok(())
    }

    fn append_to_message(&mut self, current_user: &User, text: &str) -> Result<(), &str> {
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
                self.broadcast_message(Message::Append(text.to_owned()), None);

                Ok(())
            } else {
                Err("User not in control of typewriter")
            }
        } else {
            Err("User not in control of typewriter, but typewriter is available.")
        }
    }

    fn request_take_control(&mut self, requesting_user: &User) -> Result<(), &str> {
        if let Some(_) = self.speaker {
            Err("Another user is in control of the typewriter")
        } else {
            self.speaker = Some(requesting_user.clone());
            self.broadcast_message(
                Message::TypewriterControl(
                    Some(requesting_user.name.clone(),)
                ), 
                None
            );
            Ok(())
        }
    }

    fn relinquish(&mut self, user: &User, exempt_user: bool) -> Result<(), &str> {
        if let Some(ref speaker) = self.speaker {
            if speaker != user {
                Err("Cannot relinquish typewriter. User not in control.")
            } else {
                self.speaker = None;
                if exempt_user {
                    self.broadcast_message(Message::TypewriterControl(None), Some(user));
                } else {
                    self.broadcast_message(Message::TypewriterControl(None), None);
                }
                Ok(())
            }
        } else {
            Err("Cannot relinquish typewriter. No user is in control.")
        }
    }

    fn broadcast_message(&mut self, message: Message, exempt: Option<&User>) {
        for index in 0..self.users.len() {
            let user = self.users.get(index).unwrap();
            // Skip the broadcast if a user's exempted.
            if exempt == Some(user) {
                continue; 
            }
            let send_result = user.send_message(message.clone());

            // If the send failed for whatever reason, remove the user from the channel.
            if let Err(err) = send_result {
                eprintln!(
                    "Error sending message to {:?}: {:?}. Removing from channel.",
                    user, err
                );
                self.users.swap_remove(index);
            }
        }
    }

    fn remove_user(&mut self, user: &User) {
        for i in 0..self.users.len() {
            if self.users.get(i).unwrap() == user {
                // If the removed user is in control of the typewriter, attempt to relinquish. If it fails we don't really care.
                // Exempt the user from any echoing to prevent 
                let _ = self.relinquish(user, true);
                self.users.swap_remove(i);
            }
        }
    }

    fn handle_command_error(&mut self, user: &User, err: &str) {
        let err_message_result = user.send_message(Message::Error(err.to_owned()));
        if let Err(_) = err_message_result {
            self.remove_user(&user)
        }
    }

    fn get_current_state(&self) -> Message {
        Message::ChannelState(ChannelState {
            users: (&self.users)
                .into_iter()
                .map(|user| user.name.clone())
                .collect(),
            speaker: match &self.speaker {
                Some(speaker) => Some(speaker.name.clone()),
                None => None,
            },
            current_message: match &self.current_message {
                Some(message) => Some(message.clone()),
                None => None,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct ChannelState {
    pub users: Vec<String>,
    pub speaker: Option<String>,
    pub current_message: Option<String>,
}
