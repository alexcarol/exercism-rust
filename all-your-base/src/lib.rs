///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits. However, your function must be able to
///     process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    let mut value = 0;

    if from_base <= 1 || to_base <= 1 {
        return Result::Err(());
    }

    for digit in number {
        if *digit >= from_base {
            return Result::Err(());
        }
    }

    let mut multiplier = 1;
    if number.len() == 0 {
        return Result::Ok(vec!());
    }
    let mut i = number.len() - 1;
    loop {
        let digit = number[i];
        value += digit * multiplier;

        multiplier *= from_base;
        if i == 0 {
            break;
        }
        i -= 1
    }

    let mut beginning = vec!();
    while value > 0 {
        let digit = value % to_base;
        beginning.push(digit);

        value /= to_base
    }

    beginning.reverse();

    Result::Ok(beginning)
}
