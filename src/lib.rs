#[cfg(all(feature = "full", test))]
mod tests;

macro_rules! wrap_mod {
    ($feat:literal $mod:ident) => {
        paste::paste! {
            #[cfg(feature = $feat)]
            pub mod $mod { pub use [<kt_ $feat>]::*; }
        }
    };
}
wrap_mod!("std" kt);
wrap_mod!("anyhow" anyhow);
wrap_mod!("log" log);
wrap_mod!("tracing" tracing);
