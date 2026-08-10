// Deutsch: Die Loesung hat keine eigene Testdatei. Sie bindet die der Einheit
// ein, damit beide Seiten gegen genau dieselben Tests laufen.
// English: the solution has no test file of its own. It includes the unit's, so
// that both sides run against exactly the same tests.
#[path = "../../../units/09-08-webserver-einfach-und-blockierend/tests/exercise.rs"]
mod exercise;
