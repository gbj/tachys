use parking_lot::RwLock;
use std::sync::Arc;
use tachy_reaccy::prelude::*;

#[test]
fn memo_calculates_value() {
    let a = Signal::new(1);
    let b = Signal::new(2);
    let c = Signal::new(3);

    let d = Memo::new(move |_| a.get() + b.get() + c.get());
    assert_eq!(d.get(), 6);
}

#[test]
fn memo_doesnt_repeat_calculation_per_get() {
    let calculations = Arc::new(RwLock::new(0));

    let a = Signal::new(1);
    let b = Signal::new(2);
    let c = Signal::new(3);

    let d = Memo::new({
        let calculations = Arc::clone(&calculations);
        move |_| {
            *calculations.write() += 1;
            a.get() + b.get() + c.get()
        }
    });
    assert_eq!(d.get(), 6);
    assert_eq!(d.get(), 6);
    assert_eq!(d.get(), 6);
    assert_eq!(*calculations.read(), 1);

    println!("\n\n**setting to 0**");
    a.set(0);
    assert_eq!(d.get(), 5);
    assert_eq!(*calculations.read(), 2);
}

#[test]
fn nested_memos() {
    let a = Signal::new(0); // 1
    let b = Signal::new(0); // 2
    let c = Memo::new(move |_| {
        println!("calculating C");
        a.get() + b.get()
    }); // 3
    c.debug_log_inner("c");
    let d = Memo::new(move |_| {
        println!("calculating D");
        c.get() * 2
    }); // 4
    d.debug_log_inner("d");
    /* let e = Memo::new(move |_| {
        println!("calculating E");
        d.get() + 1
    }); // 5
    e.debug_log_inner("e"); */
    //assert_eq!(e.get(), 1);
    assert_eq!(d.get(), 0);
    assert_eq!(c.get(), 0);
    println!("\n\nFirst Set\n\n");
    a.set(5);
    assert_eq!(c.get(), 5);
    assert_eq!(d.get(), 10);
    //assert_eq!(e.get(), 11);
    println!("\n\nSecond Set\n\n");
    b.set(1);
    //assert_eq!(e.get(), 13);
    assert_eq!(d.get(), 12);
    assert_eq!(c.get(), 6);
}
