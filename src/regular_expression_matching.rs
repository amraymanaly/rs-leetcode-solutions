struct S<'a> {
    s: &'a str,
    p: &'a str,
    t: usize,
}

// TODO no tweaking can be made... the rust compiler is smart enough
// algorithm logic optimization is needed...
// hmmm...
// I have an idea!
// Let's get the pattern into "normal form"!
// i.e...

//  (a,a)
//      a*a* => a*              0000 => 00
//      a*a+ => a+              0001 => 01
//      a+a* => a+              0100 => 01
//      a+a+ => a+              0101 => 01

//  (a,.)
//      a*.* => .*      #       0010 => 10
//      a*.+ => .+      #       0011 => 11
//      a+.* => a.*
//      a+.+ => a.+

//  (.,a)
//      .*a* => .*              1000 => 10
//      .*a+ => .*a
//      .+a* => .+      #       1100 => 11
//      .+a+ => .+a

//  (.,.)
//      .*.* => .*      #       1010 => 10
//      .*.+ => .+      #       1011 => 11
//      .+.* => .+      #       1110 => 11
//      .+.+ => .+              1111 => 11

// other less important patterns that I'm not implementing
// because it would require a tokenizer...

//      a*a => a+
//      aa* => a+
//      aa+ => a+

// enum RepetitionAnnotation {
//     None,
//     Star,
//     Plus,
// }

// #[derive(Clone, Copy)]
// struct PatternToken {
//     letter: char,
//     repeat: RepetitionAnnotation,
// }

// fn tokenize(p: &str) -> Vec<PatternToken> {
//     let mut pt = Vec::with_capacity(20);
//     let mut tk = PatternToken {
//         letter: '_',
//         repeat: RepetitionAnnotation::None,
//     };
//     for c in p.chars() {
//         match c {
//             '+' => tk.repeat = RepetitionAnnotation::Plus,
//             '*' => tk.repeat = RepetitionAnnotation::Star,
//             a => {
//                 // push the previous one
//                 if tk.letter != '_' {
//                     pt.push(tk);
//                 }
//                 tk.letter = a;
//                 tk.repeat = RepetitionAnnotation::None;
//             }
//         }
//     }
//     if tk.letter != '_' {
//         pt.push(tk);
//     }
//     pt
// }

pub fn is_match(s: String, mut p: String) -> (bool, usize) {
    println!("pattern [originalll] is {p}");
    normalize_pattern(&mut p, 0);
    println!("pattern [normalized] is {p}");

    let mut g = S::new(&s, &p);
    let res = g.is_there_a_match();
    (res, g.t)
}

#[allow(non_snake_case)]
fn normalize_pattern(p: &mut String, start: usize) {
    if p.len() - start < 4 {
        return;
    }

    let c = p.as_bytes();
    let (A, B, C, D) = (c[0] as char, c[1] as char, c[2] as char, c[3] as char);

    if !((B == '*' || B == '+') && (D == '*' || D == '+')) || !(A == C || A == '.' || C == '.') {
        normalize_pattern(p, start + 1);
        return;
    }
    // now it's one of the 16 patterns
    // we start with the don't cares
    // i.e., the unnumbered patterns

    let to = if A == '.' && C != '.' && D == '+' {
        format!(".{B}{C}")
    } else if A != '.' && B == '+' && C == '.' {
        format!("{A}.{D}")
    } else {
        // now, the digital patterns
        // I used an online Karnaugh map solver :D
        format!(
            "{}{}",
            if A == '.' || C == '.' { '.' } else { A },
            if B == '+' || D == '+' { '+' } else { '*' }
        )
    };

    p.replace_range(start..start + 4, &to);
    normalize_pattern(p, start);
}

impl S<'_> {
    pub fn new<'a>(s: &'a str, p: &'a str) -> S<'a> {
        S { s, p, t: 0usize }
    }

    fn repeat(&mut self, idx_s: usize, idx_p: usize) -> bool {
        // println!("repeat");
        self.is_there_a_match_from_here(true, idx_s + 1, idx_p)
    }

    fn consume(&mut self, idx_s: usize, idx_p: usize) -> bool {
        // println!("consume");
        self.is_there_a_match_from_here(false, idx_s + 1, idx_p + 2)
    }

    fn consume_letter(&mut self, idx_s: usize, idx_p: usize) -> bool {
        // println!("consume_letter");
        self.is_there_a_match_from_here(false, idx_s + 1, idx_p + 1)
    }

    fn ignore(&mut self, idx_s: usize, idx_p: usize) -> bool {
        // println!("ignore");
        self.is_there_a_match_from_here(false, idx_s, idx_p + 2)
    }

    pub fn is_there_a_match(&mut self) -> bool {
        self.is_there_a_match_from_here(false, 0, 0)
    }

    fn is_there_a_match_from_here(
        &mut self,
        repeated_once: bool,
        idx_s: usize,
        idx_p: usize,
    ) -> bool {
        // what does a normalized pattern tell me?
        // it tells me that if i can repeat a non-dot, i should,
        // until i can't (because it cannot be followed by a dot plus)
        // ughhhhh noooooo it doesn't....
        self.t += 1;
        // if self.t == 50 {
        //     panic!("too much");
        // }
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
