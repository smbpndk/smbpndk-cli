use spinners::Spinner;

#[macro_export]
macro_rules! debug {
    ($($e:expr),+) => {
        {
            #[cfg(debug_assertions)]
            {
                dbg!($($e),+)
            }
            #[cfg(not(debug_assertions))]
            {
                ($($e),+)
            }
        }
    };
}

pub struct CommandResult {
    pub spinner: Spinner,
    pub symbol: String,
    pub msg: String,
}
