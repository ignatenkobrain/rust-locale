//! Numeric category
//!
//! Handling of local variations in how numbers are written and read.
//!
//! Exact set of supported formats currently depends on the selected backend.

use std::fmt::{Display, LowerExp};
use std::any::Any;
use std::fmt;

/// Auxiliary trait for types formattable as floating point.
///
/// Simply an alias for `Display + LowerExp`, which can't be written inline argument declarations that way.
pub trait DispLowerExp: Display + LowerExp {}

impl<T: Display + LowerExp> DispLowerExp for T {}

/// Facet for formatting and reading numbers.
///
/// # TODO
pub trait Numeric: Any + Send + Sync {
    /// Format integer.
    ///
    /// Output will be written to a formatter. The padding specified on `out` may be ignored.
    ///
    /// This is a generic version that should work with any integeral type by localizing a
    /// neutral string representation produced by its `Display::fmt`.
    ///
    /// # Parameters
    ///  - `n`: The number to format.
    ///  - `fmt`: Format specification. Supported set depends on backend.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    fn format_int_to(&self, n: &Display, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result;

    /// Format integer.
    ///
    /// Output will be written to a formatter. The padding specified on `out` may be ignored.
    ///
    /// This is possibly optimized version for standard signed integeral types using `isize`.
    ///
    /// # Parameters
    ///  - `n`: The number to format.
    ///  - `fmt`: Format specification. Supported set depends on backend.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    fn format_isize_to(&self, n: isize, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result {
        self.format_int_to(&n, fmt, out)
    }

    /// Format integer.
    ///
    /// Output will be written to a formatter. The padding specified on `out` may be ignored.
    ///
    /// This is possibly optimized version for standard unsigned integeral types using `usize`.
    ///
    /// # Parameters
    ///  - `n`: The number to format.
    ///  - `fmt`: Format specification. Supported set depends on backend.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    fn format_usize_to(&self, n: usize, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result {
        self.format_int_to(&n, fmt, out)
    }

    /// Format floating point number.
    ///
    /// Output will be written to a formatter. The padding specified on `out` may be ignored.
    ///
    /// This is a generic version that should work with any floating point type by localizing
    /// a neutral string representation produced by either its `Display::fmt` or its
    /// `LowerExp::fmt` depending on output format chosen.
    ///
    /// # Parameters
    ///  - `n`: The number to format.
    ///  - `fmt`: Format specification. Supported set depends on backend.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    fn format_float_to(&self, n: &DispLowerExp, fmt: &str, out: &mut fmt::Formatter)
        -> fmt::Result;

    /// Format floating point number.
    ///
    /// Output will be written to a formatter. The padding specified on `out` may be ignored.
    ///
    /// This is possibly optimized version for standard floating point types using `f64`.
    /// # Parameters
    ///  - `n`: The number to format.
    ///  - `fmt`: Format specification. Supported set depends on backend.
    ///  - `out`: Output sink.
    ///
    /// # Errors
    ///
    // FIXME - need to define suitable generic fail type
    fn format_f64_to(&self, n: f64, fmt: &str, out: &mut fmt::Formatter) -> fmt::Result {
        self.format_float_to(&n, fmt, out)
    }

    // TODO - parsing functions
}

// XXX - is there any point in implementing any kind of unit-tests here?
