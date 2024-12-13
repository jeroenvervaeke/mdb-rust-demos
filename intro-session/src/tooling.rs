/// Adds two unsigned 32-bit integers.
///
/// This function performs simple arithmetic addition of two `u32` values and returns their sum.
/// The operation is checked in debug builds but wraps in release mode if overflow occurs.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use intro_session::tooling::add;
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Using with variables:
///
/// ```
/// # use intro_session::tooling::add;
/// let a = 42;
/// let b = 100;
/// let sum = add(a, b);
/// assert_eq!(sum, 142);
/// ```
///
/// # Overflow Behavior
///
/// In debug builds, this function will panic if the result overflows. In release builds,
/// the operation will wrap around following standard two's complement behavior.
///
/// ```rust,no_run
/// # use intro_session::tooling::add;
/// // This will work in release mode (wrapping behavior)
/// // In debug mode, this would panic
/// let max = u32::MAX;
/// let result = add(max, 1); // In release: wraps to 0
/// ```
///
/// # Arguments
///
/// * `left` - The first unsigned 32-bit integer to add
/// * `right` - The second unsigned 32-bit integer to add
///
/// # Returns
///
/// Returns the sum of `left` and `right` as a `u32`.
///
/// # Panics
///
/// This function will panic in debug builds if the result would overflow.
/// For example:
///
/// ```should_panic
/// # use intro_session::tooling::add;
/// // This will panic in debug builds
/// let result = add(u32::MAX, 1);
/// ```
///
/// # Performance
///
/// This function is implemented using native CPU addition instructions and has
/// constant time complexity O(1).
///
/// # See Also
///
/// * [`u32::wrapping_add`] - For explicit wrapping addition
/// * [`u32::checked_add`] - For checked addition that returns `None` on overflow
/// * [`u32::saturating_add`] - For saturating addition
///
pub fn add(left: u32, right: u32) -> u32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{black_box, Bencher};

    #[test]
    fn test_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[bench]
    fn bench_add(b: &mut Bencher) {
        b.iter(|| {
            // black_box prevents the compiler from optimizing away the computation
            add(black_box(2), black_box(3))
        });
    }
}
