#![allow(non_snake_case)]

#[cfg(feature = "wasm")]
use orca_whirlpools_macros::wasm_expose;

use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CoreError {
    Static(&'static str),
    InvalidTimestamp { current: u64, expected_min: u64 },
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::Static(msg) => write!(f, "{}", msg),
            CoreError::InvalidTimestamp { current, expected_min } => {
                write!(f, "Invalid timestamp: current_timestamp ({}) must be >= max(last_reference_update_timestamp, last_major_swap_timestamp) ({})", 
                       current, expected_min)
            }
        }
    }
}

impl From<&'static str> for CoreError {
    fn from(s: &'static str) -> Self {
        CoreError::Static(s)
    }
}

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const TICK_ARRAY_NOT_EVENLY_SPACED: &'static str = "Tick array not evenly spaced";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const TICK_INDEX_OUT_OF_BOUNDS: &'static str = "Tick index out of bounds";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_TICK_INDEX: &'static str = "Invalid tick index";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const ARITHMETIC_OVERFLOW: &'static str = "Arithmetic over- or underflow";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const AMOUNT_EXCEEDS_MAX_U64: &'static str = "Amount exceeds max u64";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const SQRT_PRICE_OUT_OF_BOUNDS: &'static str = "Sqrt price out of bounds";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const TICK_SEQUENCE_EMPTY: &'static str = "Tick sequence empty";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const SQRT_PRICE_LIMIT_OUT_OF_BOUNDS: &'static str = "Sqrt price limit out of bounds";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_SQRT_PRICE_LIMIT_DIRECTION: &'static str = "Invalid sqrt price limit direction";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const ZERO_TRADABLE_AMOUNT: &'static str = "Zero tradable amount";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_TIMESTAMP: &'static str = "Invalid timestamp: current_timestamp must be >= max(last_reference_update_timestamp, last_major_swap_timestamp)";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_TRANSFER_FEE: &'static str = "Invalid transfer fee";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_SLIPPAGE_TOLERANCE: &'static str = "Invalid slippage tolerance";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const TICK_INDEX_NOT_IN_ARRAY: &'static str = "Tick index not in array";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_TICK_ARRAY_SEQUENCE: &'static str = "Invalid tick array sequence";

#[cfg_attr(feature = "wasm", wasm_expose)]
pub const INVALID_ADAPTIVE_FEE_INFO: &'static str = "Invalid adaptive fee info";
