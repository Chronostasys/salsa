//! Test that creating a tracked struct outside of a
//! tracked function panics with an assert message.

#[salsa::tracked]
struct MyTracked<'db> {
    field: u32,
}

#[salsa::db]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for Database {}

#[test]
#[should_panic(
    expected = "cannot create a tracked struct disambiguator outside of a tracked function"
)]
fn execute() {
    let db = Database::default();
    MyTracked::new(&db, 0);
}
