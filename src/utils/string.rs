const SPECIAL_CHARS: [char; 1] = ['\''];
/// Return [Vec<&str>] of length 2 which
/// command as a first value
/// args as second value
pub fn get_cmd_and_args(input: &str) -> (&str, Option<&str>) {
    let v = input.trim_end().splitn(2, " ").collect::<Vec<&str>>();
    (v[0], v.get(1).copied())
}

pub fn get_formatted_input(args: &str) -> Vec<String> {
    let mut r = Vec::new();
    let mut word_to_append = String::new();

    let mut single_quotes = false;
    let mut double_quotes = false;
    let mut is_literal = false;

    for ch in args.trim().chars() {
        if is_literal {
            word_to_append.push(ch);
            is_literal = false;
            continue;
        }

        if ch == '\\' {
            is_literal = true;
            continue;
        }

        if ch == '\'' && !double_quotes {
            single_quotes = !single_quotes;
            continue;
        }

        if ch == '"' && !single_quotes {
            double_quotes = !double_quotes;
            continue;
        }

        if ch == ' ' {
            if !(single_quotes || double_quotes) {
                if word_to_append.is_empty() {
                    continue;
                }

                r.push(word_to_append.clone());
                word_to_append.clear();
            } else {
                word_to_append.push(ch);
            }
        } else {
            word_to_append.push(ch);
        }
    }

    r.push(word_to_append);

    r
}

//
// pub fn get_formatted_input(args: &str) -> Vec<String> {
//     let mut r = Vec::new();
//     let mut word = String::new();
//     let mut s: Vec<char> = Vec::new();
//
//     for ch in args.trim().chars() {
//         if SPECIAL_CHARS.contains(&ch) {
//             match s.last() {
//                 Some(last_char) => {
//                     if *last_char == ch {
//                         s.pop();
//                     } else {
//                         s.push(ch);
//                     }
//                 }
//                 None => s.push(ch),
//             }
//         } else if ch == ' ' {
//             if s.is_empty() {
//                 if !word.is_empty() {
//                     r.push(word.clone());
//                     word.clear();
//                 }
//             } else {
//                 word.push(ch);
//             }
//         } else {
//             word.push(ch);
//         }
//     }
//
//     r.push(word);
//
//     r
// }
