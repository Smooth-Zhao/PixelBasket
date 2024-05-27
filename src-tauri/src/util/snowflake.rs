use snowflaked::Snowflake;
use snowflaked::sync::Generator;

static GENERATOR: Generator = Generator::new(0);

#[allow(dead_code)]
pub fn id<T>() -> T
    where
        T: Snowflake,
{
    GENERATOR.generate::<T>()
}

#[allow(dead_code)]
pub fn id_str() -> String {
    GENERATOR.generate::<u64>().to_string()
}

pub fn timestamp() -> u128 {
    if let Ok(time) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        return time.as_micros();
    }
    return 0;
}
