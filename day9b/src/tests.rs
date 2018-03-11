#[allow(unused_imports)]
use super::{count_groups, count_score, simplify_stream, count_garbage};
#[allow(unused_imports)]
use super::parser::parse_input;

#[test]
fn simplification() {
    assert_eq!(simplify_stream(&parse_input("{}")), "{}");
    assert_eq!(simplify_stream(&parse_input("{{{}}}")), "{{{}}}");
    assert_eq!(simplify_stream(&parse_input("{{},{}}")), "{{}{}}");
    assert_eq!(simplify_stream(&parse_input("{{{},{},{{}}}}")), "{{{}{}{{}}}}");
    assert_eq!(simplify_stream(&parse_input("{<{},{},{{}}>}")), "{}");
    assert_eq!(simplify_stream(&parse_input("{<a>,<a>,<a>,<a>}")), "{}");
    assert_eq!(simplify_stream(&parse_input("{{<a>},{<a>},{<a>},{<a>}}")), "{{}{}{}{}}");
    assert_eq!(simplify_stream(&parse_input("{{<!>},{<!>},{<!>},{<a>}}")), "{{}}");
    assert_eq!(simplify_stream(&parse_input("{<{o\"i!{}<!>a,<{i<a>}")), "{}");
}

#[test]
fn groups() {
    assert_eq!(count_groups(&parse_input("{}")), 1);
    assert_eq!(count_groups(&parse_input("{{{}}}")), 3);
    assert_eq!(count_groups(&parse_input("{{},{}}")), 3);
    assert_eq!(count_groups(&parse_input("{{{},{},{{}}}}")), 6);
    assert_eq!(count_groups(&parse_input("{<{},{},{{}}>}")), 1);
    assert_eq!(count_groups(&parse_input("{<a>,<a>,<a>,<a>}")), 1);
    assert_eq!(count_groups(&parse_input("{{<a>},{<a>},{<a>},{<a>}}")), 5);
    assert_eq!(count_groups(&parse_input("{{<!>},{<!>},{<!>},{<a>}}")), 2);
}

#[test]
fn score () {
    assert_eq!(count_score(&parse_input("{}")), 1);
    assert_eq!(count_score(&parse_input("{{{}}}")), 6);
    assert_eq!(count_score(&parse_input("{{},{}}")), 5);
    assert_eq!(count_score(&parse_input("{{{},{},{{}}}},")), 16);
    assert_eq!(count_score(&parse_input("{<a>,<a>,<a>,<a>}")), 1);
    assert_eq!(count_score(&parse_input("{{<ab>},{<ab>},{<ab>},{<ab>}}")), 9);
    assert_eq!(count_score(&parse_input("{{<!!>},{<!!>},{<!!>},{<!!>}}")), 9);
    assert_eq!(count_score(&parse_input("{{<a!>},{<a!>},{<a!>},{<ab>}}")), 3);
}

#[test]
fn countgarbage () {
    assert_eq!(count_garbage(&parse_input("{}")), 0);
    assert_eq!(count_garbage(&parse_input("{{{}}}")), 0);
    assert_eq!(count_garbage(&parse_input("{{},{}}")), 0);
    assert_eq!(count_garbage(&parse_input("{{{},{},{{}}}},")), 0);
    assert_eq!(count_garbage(&parse_input("{<a>,<a>,<a>,<a>}")), 4);
    assert_eq!(count_garbage(&parse_input("{{<ab>},{<ab>},{<ab>},{<ab>}}")), 8);
    assert_eq!(count_garbage(&parse_input("{{<!!>},{<!!>},{<!!>},{<!!>}}")), 0);
    assert_eq!(count_garbage(&parse_input("{{<a!>},{<a!>},{<a!>},{<ab>}}")), 17);
}
