use std::{fs::{self, OpenOptions}, io::Write, path::Path};

use chrono::Local;

pub struct SignalMessage {
    pub timestamp: String,
    pub message: String,
}

pub struct SignalInterface {
    messages_folder: String,
    client_number: String,
}

impl SignalInterface {
    pub fn new(client_number: String, messages_folder: String) -> Self {
        SignalInterface {
            messages_folder,
            client_number,
        }
    }

    pub fn add_message_to_send(&self, message: String) {
        let timestamp = Local::now().timestamp_millis().to_string();
        let mut path = Path::new(&self.messages_folder)
            .join("to-send")
            .join(&self.client_number);
        fs::create_dir_all(&path).unwrap();
        path = path.join(timestamp.to_owned() + ".lock");
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        path.set_extension("signalmessage");
        let mut messagefile = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        messagefile.write_all(message.as_bytes()).unwrap();
        path.set_extension("lock");
        fs::remove_file(path).unwrap();
    }

    pub fn add_attachment_to_send(&self, file_path: &Path) {
        let timestamp = Local::now().timestamp_millis().to_string();
        let mut path = Path::new(&self.messages_folder)
            .join("to-send")
            .join(&self.client_number);
        fs::create_dir_all(&path).unwrap();
        path = path.join(timestamp.to_owned() + ".lock");
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        path.set_extension("signalattachment");
        let mut messagefile = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        messagefile
            .write_all(file_path.to_str().unwrap().as_bytes())
            .unwrap();
        path.set_extension("lock");
        fs::remove_file(path).unwrap();
    }

    pub fn add_reply_to_message(&self, reply: String, message_timestamp_to_quote: &str) {
        let timestamp = Local::now().timestamp_millis().to_string();
        let mut path = Path::new(&self.messages_folder)
            .join("to-send")
            .join(&self.client_number);
        fs::create_dir_all(&path).unwrap();
        path = path.join(timestamp.to_owned() + ".lock");
        OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        path.set_extension("signalreply");
        let mut messagefile = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .unwrap();
        let file_content = format!("{}\n{}", message_timestamp_to_quote, reply);
        messagefile.write_all(file_content.as_bytes()).unwrap();
        path.set_extension("lock");
        fs::remove_file(path).unwrap();
    }

    pub fn access_received_messages(&self, prefix_identifier_string: String) -> Vec<SignalMessage> {
        let mut messages = vec![];
        let path = Path::new(&self.messages_folder)
            .join("received")
            .join(&self.client_number);
        if !path.exists() {
            return messages;
        }
        for entry in path.read_dir().unwrap() {
            if let Ok(file_entry) = entry {
                let file_path = file_entry.path();
                let extension = file_path.extension().unwrap();
                let stem = file_path.file_stem().unwrap().to_str().unwrap();
                if extension != "signalmessage" {
                    continue;
                }
                if let Err(_) = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create_new(true)
                    .open(&path.join(stem.to_owned() + ".lock"))
                {
                    continue;
                }
                messages.push(SignalMessage {
                    message: fs::read_to_string(&file_path).unwrap(),
                    timestamp: stem.to_owned(),
                });
                if !prefix_identifier_string.is_empty() {
                    if let Some((first_line, _)) = messages.last().unwrap().message.split_once('\n') {
                        if first_line.trim() != prefix_identifier_string.as_str() {
                            messages.pop().unwrap();
                            fs::remove_file(&path.join(stem.to_owned() + ".lock")).unwrap();
                            continue;
                        }
                    }
                }
                fs::remove_file(&file_path).unwrap();
                fs::remove_file(&path.join(stem.to_owned() + ".lock")).unwrap();
            }
        }
        messages
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let _result = super::SignalInterface::new("test".to_string(),"".to_string());
        // define later
    }
}
