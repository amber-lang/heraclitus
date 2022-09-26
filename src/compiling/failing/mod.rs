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
/// let token = meta.get_current_token();
/// if let Err(failure) = token(meta, "keyword") {
///   return error!(meta, token, "Expected keyword");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! error {
    ($meta:expr, $token:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_err_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_err_at_token($meta, $token).message($message).comment($comment)))
    };
    ($meta:expr, $token:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_err_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_err_at_token($meta, $token).message($message).comment($comment)))
    };
}

/// Macro for sending errors by position (Quiet Failure)
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return error_pos!(meta, failure.unwrap_quiet(), "Expected keyword");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! error_pos {
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
/// let token = meta.get_current_token();
/// if let Err(failure) = token(meta, "keyword") {
///   return warn!(meta, token, "This keyword is deprecated");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! warn {
    ($meta:expr, $token:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_warn_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_warn_at_token($meta, $token).message($message).comment($comment)))
    };
    ($meta:expr, $token:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_warn_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_warn_at_token($meta, $token).message($message).comment($comment)))
    };
}

/// Macro for sending warnings by position (Quiet Failure)
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return warn_pos!(meta, failure.unwrap_quiet(), "This keyword is deprecated");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! warn_pos {
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
/// let token = meta.get_current_token();
/// if let Err(failure) = token(meta, "keyword") {
///   return info!(meta, token, "This keyword is here");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! info {
    ($meta:expr, $token:expr, $message:expr) => {
        Err(Failure::Loud(Message::new_info_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr, $message:expr, $comment:expr) => {
        Err(Failure::Loud(Message::new_info_at_token($meta, $token).message($message).comment($comment)))
    };
    ($meta:expr, $token:expr => { message : $message:expr }) => {
        Err(Failure::Loud(Message::new_info_at_token($meta, $token).message($message)))
    };
    ($meta:expr, $token:expr => { message : $message:expr, comment : $comment:expr }) => {
        Err(Failure::Loud(Message::new_info_at_token($meta, $token).message($message).comment($comment)))
    };
}

/// Macro for sending infos by position (Quiet Failure)
/// 
/// # Example
/// ```
/// # use heraclitus_compiler::prelude::*;
/// # fn parse() -> SyntaxResult {
/// # let meta = &mut DefaultMetadata::new(vec![], None, None);
/// if let Err(failure) = token(meta, "keyword") {
///   return info_pos!(meta, failure.unwrap_quiet(), "This keyword is here");
/// }
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! info_pos {
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