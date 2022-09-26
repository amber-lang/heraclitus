mod syntax_module;
mod pattern;
mod preset;
mod metadata;

pub use syntax_module::*;
pub use pattern::*;
pub use metadata::*;
pub mod patterns {
    //! Utility functions that help you parse tokens
    //! 
    //! Functions in this module can help you handle tokens in the parsing phase.
    pub use super::pattern::*;
    pub use super::preset::*;
}

/// Macro that helps you capture failures thrown by inner parsing function calls
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// fn parse(meta: &mut DefaultMetadata) -> SyntaxResult {
///   token(meta, "let")?;
///   return context!({
///     let name = variable(meta, vec![])?;
///     token(meta, "=")?;
///     // ...
///     Ok(())
///   }, |quiet_failure| {
///     error!(meta, quiet_failure, "Undefined syntax at variable declaration")
///   });
/// }
/// ```
#[macro_export]
macro_rules! context {
    ($body:block, |$pos:ident| $error:block) => {
        {
            let ctx: SyntaxResult = (|| { $body })();
            if let Err(failure) = ctx {
                if let Failure::Quiet($pos) = failure {
                    $error
                } else {
                    Err(failure)
                }
            } else {
                ctx
            }
        }
    };
}