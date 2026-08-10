//! 07-10 Das Zustandsmuster / The state pattern
//!
//! Deutsch: Ein Zustand ist hier ein eigener Typ, und was in ihm erlaubt ist,
//! steht in seiner eigenen Implementierung. Ein Übergang verbraucht den alten
//! Zustand und gibt den neuen zurück.
//!
//! English: a state is a type of its own here, and what is allowed in it stands
//! in its own implementation. A transition uses the old state up and returns the
//! new one.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Was jeder Zustand können muss.
///
/// `self: Box<Self>` statt `&self` ist der Kern des Musters. Ein Übergang
/// verbraucht den alten Zustand, statt ihn zu verändern, und deshalb kann kein
/// Aufrufer versehentlich noch den alten in der Hand halten.
///
/// What every state has to be able to do.
///
/// `self: Box<Self>` instead of `&self` is the core of the pattern. A transition
/// uses the old state up rather than changing it, and that is why no caller can
/// accidentally still be holding the old one.
pub trait State {
    /// Von hier aus zur Prüfung.
    ///
    /// From here to review.
    fn review(self: Box<Self>) -> Box<dyn State>;

    /// Von hier aus freigeben.
    ///
    /// Approve from here.
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// Wie dieser Zustand heißt.
    ///
    /// What this state is called.
    fn name(&self) -> &'static str;

    /// Ob der Text in diesem Zustand nach außen sichtbar ist.
    ///
    /// Whether the text is visible to the outside in this state.
    fn visible(&self) -> bool;
}

/// Der freigegebene Zustand.
///
/// Dieser Zustand steht fertig da und ist das Muster für die beiden anderen. Er
/// ist der letzte: Beide Übergänge geben `self` zurück, also bleibt er, wo er
/// ist. Sichtbar ist der Text nur hier.
///
/// The approved state.
///
/// This state stands there finished and is the model for the other two. It is
/// the last one: both transitions return `self`, so it stays where it is. The
/// text is visible only here.
///
/// ```
/// use unit_07_10_zustandsmuster::{Approved, State};
///
/// let zustand: Box<dyn State> = Box::new(Approved);
///
/// assert_eq!(zustand.name(), "Approved");
/// assert!(zustand.visible());
///
/// // Deutsch: Beide Uebergaenge fuehren wieder hierher.
/// // English: both transitions lead back here.
/// let danach = zustand.approve();
/// assert_eq!(danach.name(), "Approved");
/// assert_eq!(danach.review().name(), "Approved");
/// ```
pub struct Approved;

impl State for Approved {
    fn review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "Approved"
    }

    fn visible(&self) -> bool {
        true
    }
}

/// Der Entwurf, der Anfangszustand.
///
/// The draft, the starting state.
pub struct Draft;

/// Aufgabe 1: Schreibe die Regeln des Entwurfs auf.
///
/// Aus dem Entwurf führt `review` in die Prüfung, also nach `Pending`.
/// `approve` führt aus dem Entwurf heraus nirgendwohin: Der Zustand bleibt, und
/// das heißt, `self` kommt zurück. `name` ist `"Draft"`, und sichtbar ist hier
/// nichts.
///
/// Exercise 1: write down the rules of the draft.
///
/// Out of the draft `review` leads into the review, meaning to `Pending`.
/// `approve` leads nowhere out of the draft: the state stays, and that means
/// `self` comes back. `name` is `"Draft"`, and nothing is visible here.
impl State for Draft {
    fn review(self: Box<Self>) -> Box<dyn State> {
        todo!("Aufgabe 1 / Exercise 1")
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        todo!("Aufgabe 1 / Exercise 1")
    }

    fn name(&self) -> &'static str {
        todo!("Aufgabe 1 / Exercise 1")
    }

    fn visible(&self) -> bool {
        todo!("Aufgabe 1 / Exercise 1")
    }
}

/// Der Zustand während der Prüfung.
///
/// The state during the review.
pub struct Pending;

/// Aufgabe 2: Schreibe die Regeln der Prüfung auf.
///
/// `review` führt von hier nirgendwohin, es wird ja schon geprüft. `approve`
/// führt nach `Approved`. `name` ist `"Pending"`, und sichtbar ist noch nichts.
///
/// Exercise 2: write down the rules of the review.
///
/// `review` leads nowhere from here, the review is already on. `approve` leads
/// to `Approved`. `name` is `"Pending"`, and nothing is visible yet.
impl State for Pending {
    fn review(self: Box<Self>) -> Box<dyn State> {
        todo!("Aufgabe 2 / Exercise 2")
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        todo!("Aufgabe 2 / Exercise 2")
    }

    fn name(&self) -> &'static str {
        todo!("Aufgabe 2 / Exercise 2")
    }

    fn visible(&self) -> bool {
        todo!("Aufgabe 2 / Exercise 2")
    }
}

/// Ein Beitrag, der einen Zustand hat.
///
/// Das `Option` ist hier kein "vielleicht keiner". Es ist der Griff, mit dem der
/// alte Zustand aus dem Beitrag herausgenommen wird, damit der Übergang ihn
/// verbrauchen kann. Ohne dieses Herausnehmen weist der Übersetzer den Übergang
/// zurück, und die Meldung dazu steht in der README.
///
/// A post that has a state.
///
/// The `Option` here is not a "maybe none". It is the handle with which the old
/// state is taken out of the post so that the transition can use it up. Without
/// that taking out, the compiler refuses the transition, and the message for it
/// is in the README.
pub struct Post {
    zustand: Option<Box<dyn State>>,
    // Deutsch: Gelesen wird dieses Feld erst von Aufgabe 3. Solange deren Rumpf
    // `todo!()` ist, meldet clippy es als ungenutzt, und deshalb steht die
    // Ausnahme hier statt eines Feldes, das beim Loesen wieder gebraucht wird.
    // English: this field is only read by exercise 3. While its body is
    // `todo!()`, clippy reports it as unused, which is why the exception stands
    // here instead of a field that is needed again when solving.
    #[allow(dead_code)]
    text: String,
}

impl Post {
    /// Ein neuer Beitrag, im Entwurf.
    ///
    /// A new post, in the draft.
    pub fn new(text: &str) -> Post {
        Post {
            zustand: Some(Box::new(Draft)),
            text: text.to_string(),
        }
    }

    /// Wie der Zustand gerade heißt.
    ///
    /// What the state is called right now.
    pub fn state_name(&self) -> &'static str {
        self.zustand.as_ref().expect("ein Zustand ist da").name()
    }

    /// Schickt den Beitrag in die Prüfung.
    ///
    /// Sends the post into the review.
    pub fn review(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.review());
        }
    }

    /// Gibt den Beitrag frei.
    ///
    /// Approves the post.
    pub fn approve(&mut self) {
        if let Some(alt) = self.zustand.take() {
            self.zustand = Some(alt.approve());
        }
    }

    /// Aufgabe 3: Gib den Text heraus, aber nur wenn der Zustand ihn zeigt.
    ///
    /// Ist der Zustand sichtbar, kommt der Text zurück. Sonst kommt der leere
    /// Text zurück. Entschieden wird das nicht hier, sondern von `visible` am
    /// Zustand, und genau das ist der Sinn des Musters: `Post` fragt und weiß
    /// nicht, welche Zustände es gibt.
    ///
    /// Exercise 3: hand the text out, but only when the state shows it.
    ///
    /// If the state is visible, the text comes back. Otherwise the empty text
    /// comes back. That is not decided here but by `visible` on the state, and
    /// that is exactly the point of the pattern: `Post` asks and does not know
    /// which states there are.
    pub fn content(&self) -> &str {
        todo!("Aufgabe 3 / Exercise 3")
    }
}
