#[macro_export]
macro_rules! dbg {
    // No args
    () => {
        eprintln!("[{}:{}]", file!(), line!());
    };

    // Single expression: print + return the value
    ($expr:expr $(,)?) => {{
        let __val = $expr;
        eprintln!(
            "[{}:{}] {} = {:?}",
            file!(),
            line!(),
            stringify!($expr),
            &__val
        );
        __val
    }};

    // Multiple expressions: print all on one line, return a tuple
    ($($expr:expr),+ $(,)?) => {{
        eprintln!(
            "[{}:{}] {}",
            file!(),
            line!(),
            format!(
                concat!($(stringify!($expr), " = {:?}; "),+),
                $(&$expr),+
            )
        );
        ( $($expr),+ )
    }};
}
