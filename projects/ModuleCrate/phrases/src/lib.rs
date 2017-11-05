pub mod english {
    pub mod greetings {
        pub fn hello() -> String {
            "Hello!".to_string()
        }
    }

    pub mod farewells {
        pub fn goodbye() -> String {
            "Goodbye.".to_string()
        }
    }
}

pub mod japanese {
    pub use self::greetings::hello;
    pub use self::farewells::goodbye as faaaaaa;

    pub mod greetings {
        pub fn hello() -> String {
            "こんにちは".to_string()
        }
    }

    pub mod farewells {
        pub fn goodbye() -> String {
            "さようなら".to_string()
        }
    }
}
