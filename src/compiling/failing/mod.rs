//! Module designed to help you send
//! - Errors
//! - Warnings
//! - Infos
//! 
//! It's recommmended to use macros for this case, but you can use this module directly.

pub mod failure;
pub mod message;
pub mod position_info;
pub mod logger;

/// Macro for sending errors
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return error!(meta, failure.unwrap_quiet(), "Expected keyword");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($meta:expr, $pos:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_err_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_err_at_position($meta, $pos).message($message).comment($comment)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_err_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_err_at_position($meta, $pos).message($message).comment($comment)))
    };
}

/// Macro for sending warnings
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return warn!(meta, failure.unwrap_quiet(), "Expected keyword");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($meta:expr, $pos:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_warn_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_warn_at_position($meta, $pos).message($message).comment($comment)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_warn_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_warn_at_position($meta, $pos).message($message).comment($comment)))
    };
}

/// Macro for sending infos
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return info!(meta, failure.unwrap_quiet(), "Expected keyword");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($meta:expr, $pos:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_info_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_info_at_position($meta, $pos).message($message).comment($comment)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_info_at_position($meta, $pos).message($message)))
    };
    ($meta:expr, $pos:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_info_at_position($meta, $pos).message($message).comment($comment)))
    };
}