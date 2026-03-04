use crate::{App, traits::HandleMessage};

impl App {
    pub fn test_update<T>(&mut self, msg: T) -> anyhow::Result<()>
    where
        Self: HandleMessage<T>,
    {
        self.update(msg).map(|_| ())
    }
}
