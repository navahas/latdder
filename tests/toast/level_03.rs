use ladder::toast::level_03::*;

#[cfg(test)]
mod toast_level_03 {
    use super::*;

    #[test]
    fn valid_toast_with_timer() {
        let _ = Toaster::<Unplugged, NoBread>::new()
            .plug_in()
            .insert_bread()
            .set_timer(10)
            .toast();
    }

    // Uncommenting this will fail to compile:
    // #[test]
    // fn invalid_set_timer_before_plug() {
    //     let _ = Toaster::<Unplugged, NoBread>::new()
    //         .set_timer(5); // ❌ should not compile: not powered
    // }
}
