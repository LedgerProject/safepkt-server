mod application;
mod domain;
mod infrastructure;

pub mod app {
    use super::application;

    pub use application::command;
    pub use application::http::controller;
    pub use application::http::middleware;
    pub use application::http::router;

    pub mod domain {
        use super::super::domain;

        pub use domain::project::manifest;
        pub use domain::value_object;
        pub use domain::verification_runtime;
    }
}

pub mod infra {
    use super::infrastructure;

    pub use infrastructure::display;
    pub use infrastructure::scaffold;
    pub use infrastructure::service::*;
    pub use infrastructure::signal_handling;
    pub use infrastructure::sigpipe;
    pub use infrastructure::verification_runtime;
}

#[cfg(test)]
pub mod test {
    use rand::prelude::*;

    pub fn generate_random_letters() -> String {
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        const WORD_LENGTH: usize = 12;
        let mut rng = rand::thread_rng();

        let word: String = (0..WORD_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        word
    }

    #[test]
    fn it_generates_random_letters() {
        let word = generate_random_letters();

        assert_eq!(12, word.len());
    }
}
