#[derive(Debug)]
pub enum Error {
    IO(String),
    Connection(String),
    Migration(String),
}
