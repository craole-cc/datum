/// Alias so every `?` automatically converts into a miette-friendly report
pub type Result<T> = miette::Result<T, crate::Error>;
// pub type Result<T> = std::result::Result<T, crate::Error>;
