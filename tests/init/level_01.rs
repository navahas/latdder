use ladder::init::level_01::*;

#[cfg(test)]
mod init_level_01 {
    use super::*;

    #[test]
    fn latdder_ready() {
        let _ = Latdder::<Ready>::start();
    }
}
