use ladder::init::{api::*, levels::level_01};

#[test]
fn latdder_ready() {
    let _ = level_01::target();
    println!("ready");
}
