// not self-balancing..

type Child = Option<Box<Trie>>;

struct Trie {
    val: Option<char>,
    left: Child,  // val is not next in the string
    right: Child, // val is next in the string
}

impl Trie {
    fn new() -> Self {
        Trie {
            val: None,
            left: None,
            right: None,
        }
    }

    fn create(node: &mut Trie, string: &impl Iterator<Item = char>) {
        let node = &mut Some(node);
        while let Some(c) = string.next() {
            
            node.val = Some(c);
            Trie::create()
        }

    }

    fn insert(&mut self, word: String) {
        // append to the existing path
        let node = self;
        let create = |nd, st| {
            match st.len() {
                0 => return,
                1 => nd.val = Some(st[0]),
            }
        }
        for (i, c) in word.chars().enumerate() {
            if let Some(val) = node.val {
                if val == c {
                    if let Some(ref mut right) = node.right {
                        node = &mut right;
                    }
                    node = &mut node.right;
                } else {
                    node = &mut node.left;
                }
            }
        }
    }

    fn search(&self, word: String) -> bool {}

    fn starts_with(&self, prefix: String) -> bool {}
}
