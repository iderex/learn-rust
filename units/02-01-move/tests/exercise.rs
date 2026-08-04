use unit_02_01_move::{duplicated, joined, length_borrowed};

#[test]
fn length_borrowed_leaves_the_original_usable() {
    let s = String::from("hallo");
    assert_eq!(length_borrowed(&s), 5);
    assert_eq!(s, "hallo");
}

#[test]
fn length_borrowed_counts_bytes() {
    let s = String::from("gruesse");
    assert_eq!(length_borrowed(&s), 7);
}

#[test]
fn duplicated_leaves_the_original_usable() {
    let s = String::from("hallo");
    let copy = duplicated(&s);
    assert_eq!(copy, "hallo");
    assert_eq!(s, "hallo");
}

#[test]
fn duplicated_is_independent_of_the_original() {
    let s = String::from("hallo");
    let mut copy = duplicated(&s);
    copy.push('!');
    assert_eq!(copy, "hallo!");
    assert_eq!(s, "hallo");
}

#[test]
fn joined_moves_both_and_returns_one() {
    let a = String::from("hallo ");
    let b = String::from("welt");
    assert_eq!(joined(a, b), "hallo welt");
}
