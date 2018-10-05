use crate::implementation::{TestContext, TestContextImpl};
use salsa::Database;

salsa::query_prototype! {
    crate trait MemoizedVolatileContext: TestContext {
        // Queries for testing a "volatile" value wrapped by
        // memoization.
        fn memoized2(key: ()) -> usize {
            type Memoized2;
        }
        fn memoized1(key: ()) -> usize {
            type Memoized1;
        }
        fn volatile(key: ()) -> usize {
            type Volatile;
            storage volatile;
        }
    }
}

impl<DB: MemoizedVolatileContext> salsa::QueryFunction<DB> for Memoized2 {
    fn execute(db: &DB, (): ()) -> usize {
        db.log().add("Memoized2 invoked");
        db.memoized1(())
    }
}

impl<DB: MemoizedVolatileContext> salsa::QueryFunction<DB> for Memoized1 {
    fn execute(db: &DB, (): ()) -> usize {
        db.log().add("Memoized1 invoked");
        let v = db.volatile(());
        v / 2
    }
}

impl<DB: MemoizedVolatileContext> salsa::QueryFunction<DB> for Volatile {
    fn execute(db: &DB, (): ()) -> usize {
        db.log().add("Volatile invoked");
        db.clock().increment()
    }
}

#[test]
fn volatile_x2() {
    let query = TestContextImpl::default();

    // Invoking volatile twice will simply execute twice.
    query.volatile(());
    query.volatile(());
    query.assert_log(&["Volatile invoked", "Volatile invoked"]);
}

/// Test that:
///
/// - On the first run of R0, we recompute everything.
/// - On the second run of R1, we recompute nothing.
/// - On the first run of R1, we recompute Memoized1 but not Memoized2 (since Memoized1 result
///   did not change).
/// - On the second run of R1, we recompute nothing.
/// - On the first run of R2, we recompute everything (since Memoized1 result *did* change).
#[test]
fn revalidate() {
    let query = TestContextImpl::default();

    query.memoized2(());
    query.assert_log(&["Memoized2 invoked", "Memoized1 invoked", "Volatile invoked"]);

    query.memoized2(());
    query.assert_log(&[]);

    // Second generation: volatile will change (to 1) but memoized1
    // will not (still 0, as 1/2 = 0)
    query.salsa_runtime().next_revision();

    query.memoized2(());
    query.assert_log(&["Memoized1 invoked", "Volatile invoked"]);

    query.memoized2(());
    query.assert_log(&[]);

    // Third generation: volatile will change (to 2) and memoized1
    // will too (to 1).  Therefore, after validating that Memoized1
    // changed, we now invoke Memoized2.
    query.salsa_runtime().next_revision();

    query.memoized2(());
    query.assert_log(&["Memoized1 invoked", "Volatile invoked", "Memoized2 invoked"]);

    query.memoized2(());
    query.assert_log(&[]);
}
