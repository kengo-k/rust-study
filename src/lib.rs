#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Swap the two vectors.
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);

            // Reverse the older vector.
            self.older.reverse();
        }

        // Now older is not empty.
        self.older.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::new();

        q.push('0');
        println!("1: {:?}", q);
        q.push('1');
        println!("2: {:?}", q);
        assert_eq!(q.pop(), Some('0'));

        q.push('∞');
        println!("3: {:?}", q);
        assert_eq!(q.pop(), Some('1'));
        println!("4: {:?}", q);
        assert_eq!(q.pop(), Some('∞'));
        println!("5: {:?}", q);
        assert_eq!(q.pop(), None);
        println!("6: {:?}", q);
    }
}
