pub const DB: &str = if cfg!(debug_assertions) {
    "./db/main.db"
} else {
    "./db/release/main.db"
};
