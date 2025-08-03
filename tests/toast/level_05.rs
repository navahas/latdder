use ladder::toast::level_05::*;

#[cfg(test)]
mod toast_level_05 {
    use super::*;

    #[test]
    fn valid_eject_sequence() {
        let _ = Toaster::<Unplugged, NoBread>::new()
            .plug_in()
            .insert_bread()
            .set_timer(2)
            .toast()
            .eject();
    }

    // Uncommenting this should not compile:
    // #[test]
    // fn cannot_eject_before_toast() {
    //     let _ = Toaster::<Unplugged, NoBread>::new()
    //         .plug_in()
    //         .insert_bread()
    //         .eject();
    // }
}
