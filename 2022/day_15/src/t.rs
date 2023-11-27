#[macro_export]
macro_rules! timeit {
    ($fn:expr) => {{
        let timer = std::time::Instant::now();
        let output = $fn;
        let t = timer.elapsed();
        (output, t)
    }};
}

#[macro_export]
macro_rules! timelog {
    ($fn:expr, $name:expr) => {{
        let timer = std::time::Instant::now();
        let output = $fn;
        let t = timer.elapsed();
        eprintln!("{} took {t:?}", $name);
        output
    }};
}
