// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Lösung bindet genau sie ein und läuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_01_muster_im_detail::{Event, Point, describe, drain_stack, first_click, kind};

fn klick(x: i32, y: i32, taste: char) -> Event {
    Event::Click {
        punkt: Point { x, y },
        taste,
    }
}

#[test]
fn describe_takes_the_guarded_arm_on_the_diagonal() {
    assert_eq!(describe(&klick(2, 2, 'L')), "L auf der Diagonalen bei 2");
    assert_eq!(describe(&klick(0, 0, 'R')), "R auf der Diagonalen bei 0");
}

#[test]
fn describe_takes_the_left_edge_arm() {
    // Deutsch: Der Punkt liegt links, aber nicht auf der Diagonalen, sonst
    // haette der Waechter darueber schon gegriffen.
    // English: the point is on the left but not on the diagonal, otherwise the
    // guard above would already have taken it.
    assert_eq!(describe(&klick(0, 5, 'L')), "am linken Rand, 5 tief");
}

#[test]
fn describe_takes_the_general_click_arm() {
    assert_eq!(describe(&klick(3, 4, 'L')), "bei 3 und 4");
}

#[test]
fn describe_binds_a_digit_with_the_at_sign() {
    assert_eq!(describe(&Event::Key('7')), "Ziffer 7");
    assert_eq!(describe(&Event::Key('0')), "Ziffer 0");
    assert_eq!(describe(&Event::Key('9')), "Ziffer 9");
}

#[test]
fn describe_takes_the_general_key_arm() {
    assert_eq!(describe(&Event::Key('a')), "Taste a");
}

#[test]
fn describe_takes_the_last_arm() {
    assert_eq!(describe(&Event::Nothing), "nichts");
}

#[test]
fn drain_stack_turns_the_list_around() {
    assert_eq!(drain_stack(vec![1, 2, 3]), vec![3, 2, 1]);
    assert_eq!(drain_stack(vec![7]), vec![7]);
}

#[test]
fn drain_stack_of_nothing_is_nothing() {
    assert_eq!(drain_stack(Vec::new()), Vec::<i32>::new());
}

#[test]
fn first_click_finds_the_first_one() {
    let ereignisse = vec![
        Event::Nothing,
        Event::Key('a'),
        klick(3, 4, 'L'),
        klick(9, 9, 'R'),
    ];

    assert_eq!(first_click(&ereignisse), Some(&Point { x: 3, y: 4 }));
}

#[test]
fn first_click_without_a_click_is_none() {
    assert_eq!(first_click(&[Event::Nothing, Event::Key('a')]), None);
    assert_eq!(first_click(&[]), None);
}

#[test]
fn the_finished_function_shows_the_same_shape() {
    assert_eq!(kind(&klick(2, 2, 'L')), "Klick");
    assert_eq!(kind(&Event::Key('7')), "Taste");
    assert_eq!(kind(&Event::Nothing), "nichts");
}
