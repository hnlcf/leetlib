#[derive(Debug, Default)]
pub struct MyQueue {
    data: Vec<i32>,
    tmp: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            tmp: Vec::new(),
        }
    }

    pub fn push(&mut self, x: i32) {
        self.tmp.clear();
        for &e in self.data.iter().rev() {
            self.tmp.push(e);
        }
        self.data.clear();

        self.tmp.push(x);

        for &e in self.tmp.iter().rev() {
            self.data.push(e);
        }
        self.tmp.clear();
    }

    pub fn pop(&mut self) -> i32 {
        self.data.pop().unwrap()
    }

    pub fn peek(&self) -> i32 {
        *self.data.last().unwrap()
    }

    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        q.push(3);
        q.push(4);
        println!("{:#?}", q);

        q.pop();
        q.push(5);
        println!("{:#?}", q);
    }
}
