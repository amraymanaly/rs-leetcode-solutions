struct List {
    next: Option<Box<List>>,
}

impl List {
    fn walk_the_list(&mut self) {
        let mut current = self;
        loop {
            match current.next {
                None => return,
                Some(ref mut inner) => current = inner,
            }
        }
    }
}

fn main() {}
