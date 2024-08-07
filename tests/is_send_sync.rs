//! Test that a setting a field on a `#[salsa::input]`
//! overwrites and returns the old value.

use test_log::test;

#[salsa::db]
trait Db: salsa::Database {}

#[salsa::db]
#[derive(Default)]
struct Database {
    storage: salsa::Storage<Self>,
}

#[salsa::db]
impl salsa::Database for Database {}

#[salsa::db]
impl Db for Database {}

#[salsa::input]
struct MyInput {
    field: String,
}

#[salsa::tracked]
struct MyTracked<'db> {
    field: MyInterned<'db>,
}

#[salsa::interned]
struct MyInterned<'db> {
    field: String,
}

#[salsa::tracked]
fn test(db: &dyn crate::Db, input: MyInput) {
    let input = is_send_sync(input);
    let interned = is_send_sync(MyInterned::new(db, input.field(db).clone()));
    let _tracked_struct = is_send_sync(MyTracked::new(db, interned));
}

fn is_send_sync<T: Send + Sync>(t: T) -> T {
    t
}

#[test]
fn execute() {
    let db = Database::default();
    let input = MyInput::new(&db, "Hello".to_string());
    test(&db, input);
}
