use std::collections::HashSet;

pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    // idea: we make sets of connected devices
    let mut connected: Vec<HashSet<i32>> = Vec::new();
    let mut free_cables = 0;
    let mut computers = 0;

    for conn in connections {
        let mut a = None;
        let mut b = None;
        for (i, s) in connected.iter().enumerate() {
            if a.is_some() && b.is_some() {
                break;
            }
            if s.contains(&conn[0]) {
                a = Some(i);
            }
            if s.contains(&conn[1]) {
                b = Some(i);
            }
        }

        match (a, b) {
            (Some(x), Some(y)) => {
                if x == y {
                    // both are already connected through another path!
                    free_cables += 1
                } else {
                    // connect them!
                    let mut v = vec![x, y];
                    v.sort_by_key(|&i| connected[i].len());
                    for comp in connected[v[0]].clone() {
                        connected[v[1]].insert(comp);
                    }
                    connected.remove(v[0]);
                }
            }
            (Some(x), None) | (None, Some(x)) => {
                // add the new computer to the connected network!
                let new = if a.is_none() { conn[0] } else { conn[1] };
                connected[x].insert(new);
                computers += 1;
            }
            _ => {
                // both are new!
                let mut z = HashSet::new();
                z.insert(conn[0]);
                z.insert(conn[1]);
                connected.push(z);
                computers += 2;
            }
        }
    }

    let disconnected = connected.len() as i32 - 1 + n - computers;
    if disconnected > free_cables {
        -1
    } else {
        disconnected
    }
}
