use crossbeam::channel::Sender;

#[derive(Debug)]
pub struct Journaler {
    tx: Sender<String>,
}

impl Journaler {
    pub fn new(tx: Sender<String>) -> Self {
        Self { tx }
    }

    pub fn log(&self, message: String) {
        let operation_result = self.tx.send(message.clone());
        if let Err(err) = operation_result {
            log::error!("Failed to send message. {}. Message: {}", err, message);
        }
    }
}
