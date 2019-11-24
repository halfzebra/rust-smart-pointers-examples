pub trait Messenger {
    fn send(&self, msg: &str);
}