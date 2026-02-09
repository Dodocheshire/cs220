//! Small problems.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::hash::Hash;

use crate::assignments::assignment03::custom_operators::MyOption;

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Sun => DayOfWeek::Mon,
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri => DayOfWeek::Mon,
        DayOfWeek::Sat => DayOfWeek::Mon,
    }
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```ignore
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```ignore
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    if values.is_empty() {
        None
    } else {
        let mut values = values; // no copy, just move
        values.sort_unstable();
        let mid = values.len() / 2;
        Some(values[mid])
    }
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash
/// map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let mut cnt: HashMap<isize, isize> = HashMap::new();
    for &v in values.iter() {
        *cnt.entry(v).or_insert(0) += 1;
    }
    cnt.into_iter()
        .max_by(|(val_a, count_a), (val_b, count_b)| {
            count_a.cmp(count_b).then_with(|| val_b.cmp(val_a))
        })
        .map(|(val, _count)| val)
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig
/// Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end
///    of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    let mut input = input;
    let mut prefix: String = String::new();
    for (i, c) in (0..).zip(input.chars()) {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                prefix = input.split_off(i);
                break;
            }
            _ => {}
        }
    }
    if input.is_empty() {
        prefix += "h";
    }
    prefix += &input;
    prefix += "ay";
    prefix
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```ignore
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```ignore
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add {person} to {department}", "Remove {person} from {department}",
///   and "Move {person} from {department} to {department}".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut depart_info: HashMap<String, HashSet<String>> = HashMap::new();
    use super::parse_shell::parse_shell_command;
    for cmd in commands.iter() {
        let parse_res = parse_shell_command(cmd);
        match (
            parse_res[0].to_ascii_lowercase().as_str(),
            parse_res[2].to_ascii_lowercase().as_str(),
        ) {
            ("add", "to") => {
                let person = parse_res[1].clone();
                let depart = parse_res[3].clone();
                let _unused = depart_info
                    .entry(depart)
                    .or_insert(HashSet::new())
                    .insert(person);
            }
            ("remove", "from") => {
                let person = parse_res[1].clone();
                let depart = parse_res[3].clone();
                match depart_info.get_mut(&depart) {
                    None => {}

                    Some(people) => {
                        if people.remove(&person) & people.is_empty() {
                            let _unused = depart_info.remove(&depart);
                        }
                    }
                }
            }
            ("move", "from") => {
                let person = parse_res[1].clone();
                let from_depart = parse_res[3].clone();
                let to_depart = parse_res[5].clone();
                let removed = if let Some(people) = depart_info.get_mut(&from_depart) {
                    let ok = people.remove(&person);
                    if people.is_empty() {
                        let _unused = depart_info.remove(&from_depart);
                    }
                    ok
                } else {
                    false
                };

                if removed {
                    let _unused = depart_info
                        .entry(to_depart)
                        .or_insert(HashSet::new())
                        .insert(person);
                }
            }
            (_, _) => {}
        }
    }
    depart_info
}

/// Events in a text editor.
#[derive(Debug)]
pub enum TypeEvent {
    /// A character is typed.
    Type(char),
    /// The last character is removed.
    Backspace,
    /// The whole string is copied to the clipboard.
    Copy,
    /// The string in the clipboard is appended.
    Paste,
}

/// Starting from an empty string and an empty clipboard,
/// processes the given `events` in order and returns the resulting string.
///
/// See the test function `test_editor` for examples.
pub fn use_editor(events: Vec<TypeEvent>) -> String {
    let mut buffer = String::new();
    let mut clipboard = String::new();
    for ev in events.iter() {
        match ev {
            TypeEvent::Type(c) => {
                buffer.push(c.clone());
            }
            TypeEvent::Backspace => {
                let _unused = buffer.pop();
            }
            TypeEvent::Copy => {
                clipboard = buffer.clone();
            }
            TypeEvent::Paste => {
                buffer += &clipboard;
            }
        }
    }
    buffer
}
