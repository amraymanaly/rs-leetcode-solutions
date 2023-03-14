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
            return is_there_a_match_from_here(&s[idx_s..], p, false)
                || is_there_a_match(&s[idx_s + 1..], p);
        }
    }
    false
}

struct LoopVars {
    did_the_first_plus: bool,
    idx_s: usize,
    idx_p: usize,
}

fn is_there_a_match_from_here(s: &str, p: &str, loop_vars: LoopVars) -> bool {
    // a version that sees if p matches s from the start, i.e, does not s.find
    // also, this version assumes that:
    //      1. p is a valid pattern, i.e, doesn't start with + or *

    // let match_pc = |c: char| pc == '.' || c == pc;

    // let mut pi: char = p[0];
    // let mut si: char = s.chars();

    // let mut po = pi.next();
    // let mut so = si.next();

    if s.len() >= loop_vars.idx_s || p.len() >= loop_vars.idx_p {
        return true;
    }

    let c = s[loop_vars.idx_s] as char;
    let p = p[loop_vars.idx_p] as char;
    let p_look_ahead = if p.len() > loop_vars.idx_p + 1 {
        p[loop_vars.idx_p + 1] as char
    } else {
        '_'
    };

    let match_em = || p == '.' || c == p;
    let repeat = || {
        // string shifted, same pattern
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                did_the_first_plus: true,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p,
            },
        );
    };

    let consume = || {
        // string shifted, pattern shifted
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                did_the_first_plus: false,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p + 2,
            },
        );
    };

    let consume_letter = || {
        // string shifted, pattern shifted
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                did_the_first_plus: false,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p + 1,
            },
        );
    };

    let ignore = || {
        // same string, pattern shifted
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                did_the_first_plus: false,
                idx_s: loop_vars.idx_s,
                idx_p: loop_vars.idx_p + 2,
            },
        );
    };

    // let repeat = || is_there_a_match_from_here(/* same string shifted right */, /* same pattern */, true);
    // let consume = || is_there_a_match_from_here(/* same string shifted right */, /* pattern after pd */);

    if let ('*', _) = (p_look_ahead, did_the_first_plus) {
        // just ignore it
    } else if !match_em() {
        return false;
    }

    // adjust loop variables
    match (p_look_ahead, did_the_first_plus) {
        ('+', false) => repeat(),
        ('+', true) => repeat() || consume(),
        ('*', _) => repeat() || consume() || ignore(),
        (_, _) => consume_letter(),
    }

    // is_there_a_match_from_here(&s[1..], p, did_the_first_plus)
}
