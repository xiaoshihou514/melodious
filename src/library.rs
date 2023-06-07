use std::sync::mpsc;

pub struct Library {
    buffer: Vec<String>,

    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<String>,
}

impl Library {
    pub fn new(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<String>) -> Self {
        Library {
            buffer: Vec::new(),
            sender: sender,
            receiver: receiver,
        }
    }

    pub fn init(&mut self) {}

    pub fn exit(&self) {}

    pub fn receive_dirs(&self) {}
}
