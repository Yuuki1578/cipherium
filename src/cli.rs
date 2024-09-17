use std::env;

/// Argument handler using `Vec<String>`
/// ## Example:
/// ```rust
/// // The input is "program foo bar baz"
/// let args = Args::parse(3);
///
/// //comparing
/// assert_eq!(&vec[
///     "foo".to_string(),
///     "bar".to_string(),
///     "baz".to_string()], &args.list),
/// ```
#[derive(Debug, Clone)]
pub struct Args {
    /// A vector of arguments
    /// represented as `Vec<String>`
    pub list: Vec<String>,
}

impl Args {
    /// Get _n_ arguments and store it as a `Vec<String>`.
    /// The _n_ is a limiter for when to stop to getting the arguments.
    /// This function also excluding the first arguments, which usually the program name.
    /// ## Example:
    /// ```rust
    /// let args = Args::parse(1); // input will be "foo"
    /// assert_eq!(&args.list, &vec!["foo".to_string()]);
    /// ```
    pub fn parse(limit: usize) -> Self {
        let mut as_used: Vec<String> = Vec::new();

        for (index, string) in env::args().enumerate() {
            if index == 0 {
                continue;
            }

            as_used.push(string);

            if index == limit {
                break;
            }
        }

        Self { list: as_used }
    }

    /// A wrapper arround
    /// ```rust
    /// Vec<T>::get(&Self) -> Option<&T>
    /// ```
    pub fn get(&self, index: usize) -> Option<&String> {
        self.list.get(index)
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn compare<F>(&self, index: usize, with: &str, or: &str, f: F)
    where
        F: FnOnce(),
    {
        if let Some(args) = self.list.get(index) {
            if args == with || args == or {
                f();
            }

            return ();
        }
    }
}

pub fn help() -> &'static str {
    r#"
Cipherium: encrypt or decrypt file with caesar cipher
basic usage:

    --help | -h        print this help message

    --version | -v     print the current major version

    --encode | -e      encode file with the following patterns:
        cipherium --encode <FILE> <KEY> where KEY is number of shift right

    --decode | -d      decode file with the following patterns:
        cipherium --decode <FILE> <KEY> where KEY is number of shift left

All key must be valid within u8 boundaries
starting from 0 to 255. If not, integer overflow
will occurs, and this program will handle it for you

Have a trouble or found a bug? create an issue at my repo
https://github.com/Yuuki1578/cipherium
"#
}

pub fn version(static_ver: &str) -> String {
    format!("cipherium version {}", static_ver)
}
