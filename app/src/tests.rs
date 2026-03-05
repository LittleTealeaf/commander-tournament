use crate::{App, traits::HandleMessage};

impl App {
    pub fn test_update<T>(&mut self, msg: T) -> anyhow::Result<()>
    where
        Self: HandleMessage<T>,
    {
        self.update(msg).map(|_| ())
    }

    pub fn test_updates<I, T>(&mut self, messages: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item = T>,
        Self: HandleMessage<T>,
    {
        for message in messages {
            self.test_update(message)?;
        }
        Ok(())
    }
}
