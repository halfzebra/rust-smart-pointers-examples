use crate::messenger::Messenger;

#[derive(Debug)]
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;
        let msg = if percentage_of_max >= 1.0 {
            Some("Error: You are over the quota!")
        } else if percentage_of_max >= 0.9 {
            Some("Urgent warning: You've used up over 90% of your quota!")
        } else if percentage_of_max >= 0.75 {
            Some("Warning: You've used up over 75% of your quota!")
        } else {
            None
        };

        if let Some(text) = msg {
            self.messenger.send(text);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            &self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn name() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(10);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 0);

        limit_tracker.set_value(76);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

        println!("{:?}", limit_tracker);
    }
}
