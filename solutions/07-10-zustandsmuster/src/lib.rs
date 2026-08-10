//! 07-10 Das Zustandsmuster / The state pattern, gelöst.
//!
//! Deutsch: Die Erklärung steht in `units/07-10-zustandsmuster/README.md`. Hier
//! stehen nur die Rümpfe, die die Tests der Einheit grün machen.
//!
//! English: the explanation lives in `units/07-10-zustandsmuster/README.md`.
//! What is here is only the bodies that turn the unit's tests green.

/// Was jeder Zustand können muss.
///
/// What every state has to be able to do.
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
/// The approved state.
///
/// ```
/// use unit_07_10_zustandsmuster::{Approved, State};
///
/// let zustand: Box<dyn State> = Box::new(Approved);
///
/// assert_eq!(zustand.name(), "Approved");
/// assert!(zustand.visible());
///
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

impl State for Draft {
    fn review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Pending)
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn name(&self) -> &'static str {
        "Draft"
    }

    fn visible(&self) -> bool {
        false
    }
}

/// Der Zustand während der Prüfung.
///
/// The state during the review.
pub struct Pending;

impl State for Pending {
    fn review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Approved)
    }

    fn name(&self) -> &'static str {
        "Pending"
    }

    fn visible(&self) -> bool {
        false
    }
}

/// Ein Beitrag, der einen Zustand hat.
///
/// A post that has a state.
pub struct Post {
    zustand: Option<Box<dyn State>>,
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

    /// Gibt den Text heraus, wenn der Zustand ihn zeigt.
    ///
    /// Hands the text out when the state shows it.
    pub fn content(&self) -> &str {
        if self.zustand.as_ref().expect("ein Zustand ist da").visible() {
            &self.text
        } else {
            ""
        }
    }
}
