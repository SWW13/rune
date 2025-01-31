use rune::runtime::{AnyObj, Shared};
use rune::Any;

#[global_allocator]
static ALLOCATOR: checkers::Allocator = checkers::Allocator::system();

#[derive(Any, Debug, PartialEq, Eq)]
struct Foo(isize);

#[checkers::test]
fn test_take() {
    let thing = Shared::new(AnyObj::new(Foo(0)));
    let _ = thing.take().unwrap();
}

#[checkers::test]
fn test_clone_take() {
    let thing = Shared::new(AnyObj::new(Foo(0)));
    let thing2 = thing.clone();
    assert_eq!(Foo(0), thing2.take_downcast::<Foo>().unwrap());
    assert!(thing.take().is_err());
}

#[checkers::test]
fn test_from_ref() {
    #[derive(Any)]
    struct Thing(u32);

    let value = Thing(10u32);

    unsafe {
        let (shared, guard) = Shared::from_ref(&value);
        assert!(shared.downcast_borrow_mut::<Thing>().is_err());
        assert_eq!(10u32, shared.downcast_borrow_ref::<Thing>().unwrap().0);

        drop(guard);

        assert!(shared.downcast_borrow_mut::<Thing>().is_err());
        assert!(shared.downcast_borrow_ref::<Thing>().is_err());
    }
}

#[checkers::test]
fn test_from_mut() {
    #[derive(Any)]
    struct Thing(u32);

    let mut value = Thing(10u32);

    unsafe {
        let (shared, guard) = Shared::from_mut(&mut value);
        shared.downcast_borrow_mut::<Thing>().unwrap().0 = 20;

        assert_eq!(20u32, shared.downcast_borrow_mut::<Thing>().unwrap().0);
        assert_eq!(20u32, shared.downcast_borrow_ref::<Thing>().unwrap().0);

        drop(guard);

        assert!(shared.downcast_borrow_mut::<Thing>().is_err());
        assert!(shared.downcast_borrow_ref::<Thing>().is_err());
    }
}
