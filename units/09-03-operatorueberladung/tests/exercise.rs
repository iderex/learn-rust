// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_09_03_operatorueberladung::{Point, Week, sum};

fn woche() -> Week {
    Week {
        tage: ["Mo", "Di", "Mi", "Do", "Fr", "Sa", "So"],
    }
}

#[test]
fn plus_adds_per_axis() {
    let links = Point { x: 1, y: 2 };
    let rechts = Point { x: 3, y: 4 };

    assert_eq!(links + rechts, Point { x: 4, y: 6 });
}

// Deutsch: Die beiden Achsen duerfen nicht vertauscht werden. Mit x und y ueber
// Kreuz kaeme hier ein anderer Punkt heraus.
// English: the two axes may not be swapped. With x and y crosswise a different
// point would come out here.
#[test]
fn plus_does_not_swap_the_axes() {
    let links = Point { x: 10, y: 1 };
    let rechts = Point { x: 100, y: 2 };

    assert_eq!(links + rechts, Point { x: 110, y: 3 });
}

#[test]
fn plus_counts_negative_numbers_along() {
    let links = Point { x: -5, y: 5 };
    let rechts = Point { x: 5, y: -5 };

    assert_eq!(links + rechts, Point { x: 0, y: 0 });
}

// Deutsch: `Point` ist `Copy`, also stehen beide Seiten nach dem Plus noch da.
// English: `Point` is `Copy`, so both sides are still there after the plus.
#[test]
fn plus_uses_neither_side_up() {
    let links = Point { x: 1, y: 2 };
    let rechts = Point { x: 3, y: 4 };

    let summe = links + rechts;

    assert_eq!(summe, Point { x: 4, y: 6 });
    assert_eq!(links, Point { x: 1, y: 2 });
    assert_eq!(rechts, Point { x: 3, y: 4 });
}

#[test]
fn the_brackets_reach_the_days() {
    let woche = woche();

    assert_eq!(&woche[0], "Mo");
    assert_eq!(&woche[3], "Do");
    assert_eq!(&woche[6], "So");
}

#[test]
#[should_panic]
fn the_brackets_abort_beyond_the_last_day() {
    let woche = woche();

    let _ = &woche[7];
}

#[test]
fn sum_of_nothing_is_the_point_at_zero() {
    assert_eq!(sum(&[]), Point { x: 0, y: 0 });
}

#[test]
fn sum_adds_every_point_of_the_list() {
    let punkte = [
        Point { x: 1, y: 2 },
        Point { x: 3, y: 4 },
        Point { x: -4, y: -6 },
    ];

    assert_eq!(sum(&punkte), Point { x: 0, y: 0 });
}

// Deutsch: Ein Rumpf, der nur den ersten oder nur den letzten Punkt nimmt,
// faellt hier auf.
// English: a body taking only the first or only the last point shows up here.
#[test]
fn sum_takes_more_than_one_point() {
    let punkte = [
        Point { x: 1, y: 0 },
        Point { x: 10, y: 0 },
        Point { x: 100, y: 0 },
    ];

    assert_eq!(sum(&punkte), Point { x: 111, y: 0 });
}

#[test]
fn the_finished_operator_turns_both_signs_around() {
    let punkt = Point { x: 1, y: -2 };

    assert_eq!(-punkt, Point { x: -1, y: 2 });
    assert_eq!(-Point { x: 0, y: 0 }, Point { x: 0, y: 0 });
    assert_eq!(punkt, Point { x: 1, y: -2 });
}
