struct S<'a> {
    s: &'a str,
    p: &'a str,
}

pub fn is_match(s: String, p: String) -> bool {
    S::new(&s, &p).is_there_a_match()
}

impl S<'_> {
    pub fn new<'a>(s: &'a str, p: &'a str) -> S<'a> {
        S { s, p }
    }

    fn repeat(&self, idx_s: usize, idx_p: usize) -> bool {
        println!("repeat");
        self.is_there_a_match_from_here(true, idx_s + 1, idx_p)
    }

    fn consume(&self, idx_s: usize, idx_p: usize) -> bool {
        println!("consume");
        self.is_there_a_match_from_here(false, idx_s + 1, idx_p + 2)
    }

    fn consume_letter(&self, idx_s: usize, idx_p: usize) -> bool {
        println!("consume_letter");
        self.is_there_a_match_from_here(false, idx_s + 1, idx_p + 1)
    }

    fn ignore(&self, idx_s: usize, idx_p: usize) -> bool {
        println!("ignore");
        self.is_there_a_match_from_here(false, idx_s, idx_p + 2)
    }

    pub fn is_there_a_match(&self) -> bool {
        self.is_there_a_match_from_here(false, 0, 0)
    }

    fn is_there_a_match_from_here(&self, repeated_once: bool, idx_s: usize, idx_p: usize) -> bool {
        if self.p.len() <= idx_p {
            return self.s.len() <= idx_s;
        }

        if self.s.len() <= idx_s {
            return self.can_pattern_be_ignored(idx_p);
        }

        let c = self.s.as_bytes()[idx_s] as char;
        let pc = self.p.as_bytes()[idx_p] as char;
        let p_look_ahead = if self.p.len() > idx_p + 1 {
            self.p.as_bytes()[idx_p + 1] as char
        } else {
            '_'
        };

        if !(pc == '.' || c == pc) {
            if p_look_ahead == '*' || (p_look_ahead == '+' && repeated_once) {
                return self.ignore(idx_s, idx_p);
            }
            return false;
        }

        match (p_look_ahead, repeated_once) {
            ('+', false) => self.repeat(idx_s, idx_p),
            ('+', true) => self.repeat(idx_s, idx_p) || self.consume(idx_s, idx_p),
            ('*', _) => {
                self.repeat(idx_s, idx_p) || self.consume(idx_s, idx_p) || self.ignore(idx_s, idx_p)
            }
            (_, _) => self.consume_letter(idx_s, idx_p),
        }
    }

    fn can_pattern_be_ignored(&self, idx_p: usize) -> bool {
        let p = &self.p[idx_p..];
        if p.len() < 2 || p.len() % 2 == 1 {
            return false;
        }
        for pc in p.chars().skip(1).step_by(2) {
            if pc != '*' {
                return false;
            }
        }
        return true;
    }
}
