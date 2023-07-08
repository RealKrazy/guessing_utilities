//! # Guessing utilities
//! This crate is ideal for writing programs for guessing numbers in range (0..101).
//! It provides a handful of utilities made for various cases.

use rand::Rng;

/// Used for creating a guess. This is a good alternative to storing a guessed number,
/// which is much safer and comfortable to use compared to a primitive number.
/// 
/// # Usage
/// Use `Guess::new(val: i32)` to create a guess object. The object will store the number now.
/// An error might be returned, make sure to handle in appropriate situations.
/// Now, the object can be used to compare with other guesses, extract the value, etc.
/// 
/// # Safety
/// The constructor automatically checks if a provided input is in correct range.
/// The object provides other utilities, like parsing which provide a safe way to do error handling.
/// 
/// # Comfort
/// The object implements equality and comparing checks to use with other guesses,
/// therefore not having to rely on `value()` function every time.
#[derive(Eq, Debug)]
pub struct Guess {
    val: i32, // i32 instead of u32 for future capabilities
}

/// Custom-written error handling.
pub mod err {
    use std::fmt;

    /// Used when the provided argument is outside the required (0..101) range.
    /// Usually returned by `Guess::new(val: i32)` function when an invalid input is provided.
    #[derive(Debug, Clone)]
    pub struct GuessRangeError;

    impl fmt::Display for GuessRangeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "The guess value was out of 0-100 range")
        }
    }

    impl std::error::Error for GuessRangeError {}
}

impl Guess {
    /// Creates a new `Guess` object.
    /// An error will be returned if the provided number was outside (0..101) range.
    /// # Example
    /// ```
    /// use guessing_utils::Guess;
    /// 
    /// let value = 100; // our guess number
    /// let guess = Guess::new(value);
    /// 
    /// let guess = match guess {
    ///     Ok(val) => val,
    ///     Err(err) => panic!("Provided value was outside the range!"),
    /// };
    /// ```
    pub fn new(val: i32) -> Result<Guess, err::GuessRangeError> {
        if val < 0 || val > 100 {
            return Err(err::GuessRangeError);
        }

        Ok(Guess { val })
    }

    /// Creates a new object from parsing the provided string slice.
    /// Invalid input will produce either a parsing or a range error.
    /// # Example
    /// ```
    /// use guessing_utils::Guess;
    /// 
    /// let guess = "57"; // value to parse (input)
    /// 
    /// let guess = match Guess::parse(&guess) {
    ///     Ok(val) => val,
    ///     Err(err) => panic!("{}", err), // handle the input error
    /// };
    /// ```
    pub fn parse(val: &str) -> Result<Guess, Box<dyn std::error::Error>> {
        let parsed: i32 = match val.trim().parse() {
            Ok(val) => val,
            Err(err) => return Err(err)?,
        };

        match Guess::new(parsed) {
            Ok(val) => Ok(val),
            Err(err) => Err(err)?
        }
    }

    /// Gets the value stored in the object.
    /// # Example
    /// ```
    /// use guessing_utils::Guess;
    /// 
    /// let guess = Guess::new(15).unwrap();
    /// println!("{}", guess.value()); // 15
    /// ```
    pub fn value(&self) -> &i32 {
        &self.val
    }
}

impl Ord for Guess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(other.value())
    }
}

impl PartialOrd for Guess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Guess {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

/// Generates a `Guess` object together with a randomly generated number in (0..101) range put inside the object.
/// # Example
/// ```
/// use guessing_utils::Guess;
/// use guessing_utils::gen_random;
/// 
/// use std::cmp::Ordering;
/// 
/// let random_guess = gen_random();
/// let my_guess = Guess::new(50).unwrap();
/// 
/// match random_guess.cmp(&my_guess) {
///     Ordering::Equal => println!("I guessed the number!"),
///     _ => (),
/// }
/// ```
pub fn gen_random() -> Guess {
    Guess::new(rand::thread_rng().gen_range(0..101)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn eq_test() {
        let guess1 = Guess::new(13).unwrap();
        let guess2 = Guess::new(13).unwrap();

        assert_eq!(guess1, guess2);
    }

    #[test]
    fn cmp_test() {
        let guess1 = Guess::new(50).unwrap();
        let guess2 = Guess::new(13).unwrap();

        assert_eq!(guess1.cmp(&guess2), Ordering::Greater);
    }

    #[test]
    fn parse_test() {
        match Guess::parse("16") {
            Ok(_) => (),
            Err(err) => panic!("Should had no errors, got: {}", err),
        }

        match Guess::parse("val") {
            Ok(_) => panic!("Should have panicked but didn't."),
            Err(_) => (),
        }
    }
}