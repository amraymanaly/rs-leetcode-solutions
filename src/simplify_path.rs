// use crate::common_data_structures::stack::Stack;

pub fn simplify_path(path: String) -> String {
    let mut dirs = vec![];
    let mut bottom = true;

    for dir in path.split('/') {
        // println!("found {dir}, {}", dir.len());
        match dir {
            "" | "." => (),
            ".." => {
                if !bottom {
                    dirs.pop();
                }
            }
            other => {
                bottom = false;
                dirs.push(other);
            }
        }
    }
    println!("dirs is {:#?}", dirs);
    format!("/{}", dirs.join("/"))
}
