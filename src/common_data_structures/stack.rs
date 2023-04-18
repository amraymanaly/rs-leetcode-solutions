pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            vec: Vec::default(),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }

    pub fn push(&mut self, t: T) {
        self.vec.push(t);
    }
}
