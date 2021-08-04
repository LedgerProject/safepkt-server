mod application;
mod domain;
mod infrastructure;

pub mod app {
    use super::application;

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

    pub use infrastructure::project_scaffold;
    pub use infrastructure::service::*;
    pub use infrastructure::signal_handling;
    pub use infrastructure::verification_runtime;
}
