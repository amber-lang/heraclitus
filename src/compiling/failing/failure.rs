//! This module contains Failure enum that is used to return errors from parser

use crate::compiling::failing::message::Message;
use crate::compiling::failing::position_info::PositionInfo;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// Failure enum
/// 
/// This enum returns two types of errors - `Quiet` and `Loud`.
/// 
/// The Quiet failure is used when some minor error occurs, but the parser can continue.
/// It contains detailed information about the error such as token position and length.
/// The example for that can be a syntax mismatch - the current token could be an actual
/// valid token for the current context, but it's not the one that is expected.
/// 
/// The Loud failure is used when the parser cannot continue. It contains detailed information
/// about the error such as token position and length, but also a message, comment and a full traceback.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Failure {
    /// Failure that is not important
    Quiet(PositionInfo),
    /// Failure that is important
    Loud(Message)
}

impl Failure {
    /// Returns true if this failure is quiet
    pub fn is_quiet(&self) -> bool {
        matches!(self, Failure::Quiet(_))
    }

    /// Returns true if this failure is loud
    pub fn is_loud(&self) -> bool {
        matches!(self, Failure::Loud(_))
    }

    /// Unwraps this failure into quiet failure
    pub fn unwrap_quiet(self) -> PositionInfo {
        match self {
            Failure::Quiet(info) => info,
            Failure::Loud(_) => panic!("Cannot quietly unwrap loud failure")
        }
    }

    /// Unwraps this failure into loud failure
    pub fn unwrap_loud(self) -> Message {
        match self {
            Failure::Quiet(_) => panic!("Cannot loudly unwrap quiet failure"),
            Failure::Loud(message) => message
        }
    }
}