pub fn is_match(s: String, p: String) -> bool {
    is_there_a_match(&s, &p)
}

fn is_there_a_match(s: &str, p: &String) -> bool {
    // strategy:
    // 1. find the first occurrence of a character
    //    matching the first character in the pattern
    // 2. see if we can match the whole pattern from there
    // 3. yeah? return true
    //    no? pass on the string right after that first character

    // (1)
    if p.len() == 0 {
        return false;
    }
    let pc = p.as_bytes()[0] as char;
    let match_pc = |c| pc == '.' || c == pc;

    if let Some(idx_s) = s.find(match_pc) {
        // (2), (3)
        return is_there_a_match_from_here(
            &s[idx_s..],
            p,
            LoopVars {
                repeated_once: false,
                idx_s: 0,
                idx_p: 0,
            },
        ) || is_there_a_match(&s[idx_s + 1..], p);
    }
    s.len() == 0 || can_pattern_be_ignored(p)
}

struct LoopVars {
    repeated_once: bool,
    idx_s: usize,
    idx_p: usize,
}

fn can_pattern_be_ignored(p: &str) -> bool {
    if p.len() < 2 || p.len() % 2 == 1 {
        return false;
    }
    for pc in p.chars().skip(1).step_by(2) {
        println!("pc is {pc}");
        if pc != '*' {
            return false;
        }
    }
    return true;
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

    if p.len() <= loop_vars.idx_p {
        // pattern ended
        return true;
    }

    if s.len() <= loop_vars.idx_s {
        // string ended
        // can the rest of the pattern be ignored?
        return can_pattern_be_ignored(&p[loop_vars.idx_p..]);
    }

    let c = s.as_bytes()[loop_vars.idx_s] as char;
    let pc = p.as_bytes()[loop_vars.idx_p] as char;
    let p_look_ahead = if p.len() > loop_vars.idx_p + 1 {
        p.as_bytes()[loop_vars.idx_p + 1] as char
    } else {
        '_'
    };

    let match_em = || pc == '.' || c == pc;
    let repeat = || {
        println!("repeat");
        // string shifted, same pattern
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                repeated_once: true,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p,
            },
        )
    };

    let consume = || {
        // string shifted, pattern shifted
        println!("consume");
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                repeated_once: false,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p + 2,
            },
        )
    };

    let consume_letter = || {
        println!("consume_letter");

        // string shifted, pattern shifted
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                repeated_once: false,
                idx_s: loop_vars.idx_s + 1,
                idx_p: loop_vars.idx_p + 1,
            },
        )
    };

    let ignore = || {
        println!("ignore");

        // same string, pattern shifted
        is_there_a_match_from_here(
            s,
            p,
            LoopVars {
                repeated_once: false,
                idx_s: loop_vars.idx_s,
                idx_p: loop_vars.idx_p + 2,
            },
        )
    };

    // let repeat = || is_there_a_match_from_here(/* same string shifted right */, /* same pattern */, true);
    // let consume = || is_there_a_match_from_here(/* same string shifted right */, /* pattern after pd */);

    if !match_em() {
        if p_look_ahead == '*' || (p_look_ahead == '+' && loop_vars.repeated_once) {
            return ignore();
        }
        return false;
    }

    // adjust loop variables
    match (p_look_ahead, loop_vars.repeated_once) {
        ('+', false) => repeat(),
        ('+', true) => repeat() || consume(),
        ('*', _) => repeat() || consume(),
        (_, _) => consume_letter(),
    }

    // is_there_a_match_from_here(&s[1..], p, did_the_first_plus)
}
