use ladder::toast::level_02::*;

#[cfg(test)]
mod toast_level_02 {
    use super::*;

    #[test]
    fn valid_toast_sequence() {
        let _ = Toaster::<Unplugged, NoBread>::new()
            .plug_in()
            .insert_bread()
            .toast();
    }

    // Uncommenting this will fail to compile as expected:
    // #[test]
    // fn invalid_without_bread() {
    //     let _ = Toaster::<Unplugged, NoBread>::new()
    //         .plug_in()
    //         .toast();
    // }
}
