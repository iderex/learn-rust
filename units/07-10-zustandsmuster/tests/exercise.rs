// Deutsch: Diese Datei ist die einzige Testdatei der Einheit. Die gleichnamige
// Loesung bindet genau sie ein und laeuft gegen dieselben Tests.
// English: this file is the unit's only test file. The solution of the same
// name includes exactly this file and runs against the same tests.
use unit_07_10_zustandsmuster::{Approved, Post, State};

#[test]
fn a_new_post_is_a_draft_and_shows_nothing() {
    let beitrag = Post::new("Heute gelernt");

    assert_eq!(beitrag.state_name(), "Draft");
    assert_eq!(beitrag.content(), "");
}

#[test]
fn the_way_through_goes_draft_pending_approved() {
    let mut beitrag = Post::new("Heute gelernt");

    beitrag.review();
    assert_eq!(beitrag.state_name(), "Pending");
    assert_eq!(beitrag.content(), "");

    beitrag.approve();
    assert_eq!(beitrag.state_name(), "Approved");
    assert_eq!(beitrag.content(), "Heute gelernt");
}

// Deutsch: Freigeben aus dem Entwurf heraus tut nichts. Ein Rumpf, der `approve`
// im Entwurf nach `Approved` fuehren laesst, faellt hier auf.
// English: approving out of the draft does nothing. A body letting `approve` in
// the draft lead to `Approved` shows up here.
#[test]
fn approving_a_draft_leaves_it_where_it_is() {
    let mut beitrag = Post::new("Heute gelernt");

    beitrag.approve();

    assert_eq!(beitrag.state_name(), "Draft");
    assert_eq!(beitrag.content(), "");
}

// Deutsch: Noch einmal pruefen, waehrend geprueft wird, tut nichts.
// English: reviewing again while the review is on does nothing.
#[test]
fn reviewing_twice_leaves_the_state_where_it_is() {
    let mut beitrag = Post::new("Heute gelernt");

    beitrag.review();
    beitrag.review();

    assert_eq!(beitrag.state_name(), "Pending");
    assert_eq!(beitrag.content(), "");
}

#[test]
fn an_approved_post_stays_approved() {
    let mut beitrag = Post::new("Heute gelernt");

    beitrag.review();
    beitrag.approve();
    beitrag.review();
    beitrag.approve();

    assert_eq!(beitrag.state_name(), "Approved");
    assert_eq!(beitrag.content(), "Heute gelernt");
}

// Deutsch: Der Text wird nicht veraendert, er wird nur gezeigt oder nicht.
// English: the text is not changed, it is only shown or not.
#[test]
fn content_hands_the_whole_text_out_once_it_is_visible() {
    let mut beitrag = Post::new("");

    beitrag.review();
    beitrag.approve();
    assert_eq!(beitrag.content(), "");

    let mut zweiter = Post::new("mit  zwei Leerzeichen");
    zweiter.review();
    zweiter.approve();
    assert_eq!(zweiter.content(), "mit  zwei Leerzeichen");
}

#[test]
fn the_finished_state_is_the_last_one() {
    let zustand: Box<dyn State> = Box::new(Approved);

    assert_eq!(zustand.name(), "Approved");
    assert!(zustand.visible());
    assert_eq!(zustand.review().name(), "Approved");
}
