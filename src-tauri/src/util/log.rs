#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if cfg!(debug_assertions) { 
            use chrono::{DateTime, Local};
            let now: DateTime<Local> = Local::now();
            println!(
                "{} [{}] |{}| {}",
                now.format("%Y-%m-%d %H:%M:%S"),
                "DEBUG",
                &format!($($arg)*)
            );   
        }
    }}
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use chrono::{DateTime, Local};
        let now: DateTime<Local> = Local::now();
        println!(
            "{} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            "INFO",
            &format!($($arg)*)
        );
    }}
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        use chrono::{DateTime, Local};
        let now: DateTime<Local> = Local::now();
        println!(
            "{} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            "WARN",
            &format!($($arg)*)
        );
    }}
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use chrono::{DateTime, Local};
        let now: DateTime<Local> = Local::now();
        println!(
            "{} [{}] {}",
            now.format("%Y-%m-%d %H:%M:%S"),
            "ERROR",
            &format!($($arg)*)
        );
    }}
}