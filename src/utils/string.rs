const SPECIAL_CHARS: [char; 1] = ['\''];
/// Return [Vec<&str>] of length 2 which
/// command as a first value
/// args as second value
pub fn get_cmd_and_args(input: &str) -> (&str, Option<&str>) {
    let v = input.trim_end().splitn(2, " ").collect::<Vec<&str>>();
    // println!("splitted: {:?}", v);
    (v[0], v.get(1).copied())
}

pub fn get_formatted_input(args: &str) -> Vec<String> {
    let mut r = Vec::new();
    let mut word = String::new();
    let mut s: Vec<char> = Vec::new();

    for ch in args.trim().chars() {
        if SPECIAL_CHARS.contains(&ch) {
            match s.last() {
                Some(last_char) => {
                    if *last_char == ch {
                        s.pop();
                    } else {
                        s.push(ch);
                    }
                }
                None => s.push(ch),
            }
        } else if ch == ' ' {
            if s.is_empty() {
                if !word.is_empty() {
                    r.push(word.clone());
                    word.clear();
                }
            } else {
                word.push(ch);
            }
        } else {
            word.push(ch);
        }
    }

    r.push(word);

    r
}
