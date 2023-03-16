use std::str::Chars;

fn all_mut<F>(iters: &mut Vec<Chars>, mut f: F) -> bool
where
    F: FnMut(&mut Chars) -> bool,
{
    for i in iters {
        if !f(i) {
            return false;
        }
    }
    true
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.iter().any(|s| s.len() == 0) {
        return "".into();
    }

    let mut iters: Vec<Chars> = strs.iter().map(|s| s.chars()).collect();

    let mut idx = 0;
    let mut c = None;

    loop {
        if all_mut(&mut iters, |s| match c {
            Some(d) => match s.next() {
                Some(e) => {
                    // println!("testing {d}");
                    d == e
                }
                None => false,
            },
            None => {
                c = s.next();
                true
            }
        }) {
            idx += 1;
            c = None;
        } else {
            break;
        }
    }
    strs[0][..idx].into()
}
