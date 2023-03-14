pub fn is_match(s: String, p: String) -> bool {
    is_there_a_match(&s, &s)
}

fn is_there_a_match(s: &str, p: &String) -> bool {
    // strategy:
    // 1. find the first occurrence of a character
    //    matching the first character in the pattern
    // 2. see if we can match the whole pattern from there
    // 3. yeah? return true
    //    no? pass on the string right after that first character

    // (1)
    let mut pi = p.chars();
    if let Some(mut pc) = pi.next() {
        let match_pc = |c| pc == '.' || c == pc;
        if let Some(idx_s) = s.find(match_pc) {
            // (2), (3)
            return is_there_a_match_from_here(&s[idx_s..], p)
                || is_there_a_match(&s[idx_s + 1..], p);
        }
    }
    false
}

fn is_there_a_match_from_here(s: &str, p: &str) -> bool {
    // a version that sees if p matches s from the start, i.e, does not s.find
    // also, this version assumes that:
    //      1. s and p are both non-empty
    //      2. p is a valid pattern, i.e, doesn't start with + or *

    // let match_pc = |c: char| pc == '.' || c == pc;

    // let mut pi: char = p[0];
    // let mut si: char = s.chars();

    // let mut po = pi.next();
    // let mut so = si.next();

    let mut look_ahead = '_';

    if look_ahead == '+' || look_ahead == '*' {
        if is_there_a_match_from_here(/* same string */, /* same pattern */) ||
           is_there_a_match_from_here(/* same string shifted right */, /* pattern after pd */) {
        return true;
           }
    }

    pc = look_ahead;
    look_ahead = pi.next().unwrap_or('_'); // :D
    so = si.next();
    if !match_pc(sc) {
        if look_ahead == '*' && so.is_some() {
        return is_there_a_match_from_here(/* same string shifting right */, /* pattern after pd */);
        }
        return false;
    }
    true
}
