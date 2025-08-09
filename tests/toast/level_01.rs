use ladder::toast::{api::*, levels::level_01};

#[cfg(test)]
mod toast_level_01 {
    use super::*;

    #[test]
    fn valid_toast_sequence() {
        let _ = Toaster::<Unplugged>::new().plug_in().toast();
    }

    // This should not compile, so it's commented out as a teaching tool:
    // #[test]
    // fn invalid_toast_without_plugin() {
    //     let _ = Toaster::<Unplugged>::new().toast(); // should not compile
    // }

    #[test]
    fn can_create_unplugged_toaster() {
        let _toaster = Toaster::<Unplugged>::new();
    }

    #[test]
    fn can_plug_in_toaster() {
        let _plugged = Toaster::<Unplugged>::new().plug_in();
    }

    #[test]
    fn level_01_completed() {
        level_01::target();
    }
}
