//! 09-07 Deklarative Makros mit macro_rules! / Declarative macros with macro_rules!
//!
//! Deutsch: Ein Makro macht aus einem Muster eine Ersetzung, und zwar bevor der
//! Übersetzer den Rest der Arbeit macht. Deshalb kann es Dinge, die eine
//! Funktion nicht kann: beliebig viele Argumente annehmen und ganze Funktionen
//! erzeugen.
//!
//! English: a macro turns a pattern into a substitution, and it does so before
//! the compiler does the rest of its work. That is why it can do things a
//! function cannot: take any number of arguments and bring whole functions into
//! being.

// Deutsch: Die Aufgaben sind offen, ihre Rümpfe sind `todo!()`, und die
// Parameter bleiben deshalb ungenutzt, bis jemand sie löst.
// English: The exercises are open, their bodies are `todo!()`, and their
// parameters therefore stay unused until somebody solves them.
#![allow(unused_variables)]

/// Steht an der Stelle eines noch offenen Makro-Rumpfs und bricht ab.
///
/// Ein `todo!()` allein reicht in einem Makro nicht. Es hat den Typ `!`, und
/// ein Makro hat keinen angeschriebenen Rückgabetyp, an dem sich das aufheben
/// ließe, also verlangt schon `assert_eq!` einen Typ, den es nicht bekommt.
/// Diese Funktion hat einen angeschriebenen Rückgabetyp, und deshalb steht sie
/// hier. Sie gehört zum Gerüst der Aufgabe und nicht zum Stoff: Wer eine
/// Aufgabe löst, ersetzt die ganze Regel und damit auch den Aufruf.
///
/// Stands in the place of a macro body that is still open and aborts.
///
/// A `todo!()` on its own does not reach inside a macro. It has the type `!`,
/// and a macro has no written-down return type at which that could be taken up,
/// so `assert_eq!` alone asks for a type it does not get. This function has a
/// written-down return type, and that is why it stands here. It belongs to the
/// scaffolding of the exercise and not to the material: whoever solves an
/// exercise replaces the whole rule and with it the call.
#[doc(hidden)]
pub fn offen<T>(wert: T, aufgabe: &str) -> T {
    todo!("{aufgabe}")
}

/// Das Quadrat eines Ausdrucks.
///
/// Dieses Makro steht fertig da und zeigt die Form. `$x:expr` fängt einen
/// Ausdruck ein, und ein Ausdruck bleibt einer: `quadrat!(1 + 2)` ist 9 und
/// nicht 5, denn ersetzt wird kein Text.
///
/// The square of an expression.
///
/// This macro stands there finished and shows the shape. `$x:expr` catches an
/// expression, and an expression stays one: `quadrat!(1 + 2)` is 9 and not 5,
/// because no text is substituted.
///
/// ```
/// use unit_09_07_deklarative_makros::quadrat;
///
/// assert_eq!(quadrat!(4), 16);
///
/// // Deutsch: 9 und nicht 5. Das Fragment ist ein Ausdruck.
/// // English: 9 and not 5. The fragment is an expression.
/// assert_eq!(quadrat!(1 + 2), 9);
/// ```
#[macro_export]
macro_rules! quadrat {
    ($x:expr) => {
        $x * $x
    };
}

/// Aufgabe 1: Gib von beliebig vielen Werten den größten heraus.
///
/// Bei einem einzigen Wert ist das dieser Wert. Bei mehreren ist es der größte,
/// und bei Gleichstand ist es der Wert, der schon da war.
///
/// Zwei Regeln sind der Weg: eine für den einen Wert und eine, die das Makro mit
/// einem Wert weniger noch einmal aufruft. Innerhalb eines exportierten Makros
/// heißt der eigene Name `$crate::groesster!`, sonst findet ein anderes Paket
/// den Rekursionsschritt nicht.
///
/// Verglichen wird mit `>`, also geht das Makro mit jedem Typ, der das kann,
/// und nicht nur mit Zahlen.
///
/// Exercise 1: hand the largest of any number of values out.
///
/// At a single value that is this value. At several it is the largest one, and
/// at a tie it is the value that was already there.
///
/// Two rules are the way: one for the single value and one calling the macro
/// once more with one value fewer. Inside an exported macro its own name is
/// `$crate::groesster!`, otherwise another package does not find the recursion
/// step.
///
/// Comparing happens with `>`, so the macro goes with every type that can do
/// that and not only with numbers.
#[macro_export]
macro_rules! groesster {
    ($($wert:expr),+ $(,)?) => {{
        let mut werte = vec![$($wert),+];

        $crate::offen(werte.remove(0), "Aufgabe 1 / Exercise 1")
    }};
}

/// Aufgabe 2: Baue aus beliebig vielen Werten ein `Vec`.
///
/// Die Werte kommen in der Reihenfolge hinein, in der sie dastehen. Ohne Wert
/// kommt ein leeres `Vec` heraus, und ein abschließendes Komma ist erlaubt.
///
/// Das ist `vec!` nachgebaut, und genau das ist der Punkt: `vec!` ist selbst so
/// ein Makro und keine Funktion, weil eine Funktion die Zahl der Werte nicht
/// offen lassen kann.
///
/// Exercise 2: build a `Vec` out of any number of values.
///
/// The values go in in the order they stand in. Without a value an empty `Vec`
/// comes out, and a trailing comma is allowed.
///
/// This is `vec!` rebuilt, and that is exactly the point: `vec!` is itself such
/// a macro and not a function, because a function cannot leave the number of
/// values open.
#[macro_export]
macro_rules! vec_von {
    ($($wert:expr),* $(,)?) => {
        $crate::offen(vec![$($wert),*], "Aufgabe 2 / Exercise 2")
    };
}

/// Aufgabe 3: Erzeuge aus einem Namen und einer Grenze eine Funktion.
///
/// `mach_pruefer!(ueber_zehn, 10);` soll eine öffentliche Funktion
/// `ueber_zehn(wert: i32) -> bool` hinstellen, die genau dann `true` liefert,
/// wenn `wert` echt größer als die Grenze ist. Bei genau der Grenze ist die
/// Antwort `false`.
///
/// `$name:ident` fängt einen Namen ein und keinen Ausdruck. Das ist der
/// Unterschied, an dem diese Aufgabe hängt: Ein Ausdruck taugt nicht als Name
/// einer Funktion.
///
/// Hier ist auch zu sehen, wofür es keine Funktion gibt. Eine Funktion, die
/// eine Funktion hinstellt, gibt es in Rust nicht.
///
/// Exercise 3: make a function out of a name and a limit.
///
/// `mach_pruefer!(ueber_zehn, 10);` shall put a public function
/// `ueber_zehn(wert: i32) -> bool` there that returns `true` exactly when
/// `wert` is strictly larger than the limit. At exactly the limit the answer is
/// `false`.
///
/// `$name:ident` catches a name and not an expression. That is the difference
/// this exercise hangs on: an expression is no good as the name of a function.
///
/// Here you also see what there is no function for. A function putting a
/// function there does not exist in Rust.
#[macro_export]
macro_rules! mach_pruefer {
    ($name:ident, $grenze:expr) => {
        pub fn $name(wert: i32) -> bool {
            todo!("Aufgabe 3 / Exercise 3")
        }
    };
}

mach_pruefer!(ueber_zehn, 10);
mach_pruefer!(ueber_hundert, 100);
