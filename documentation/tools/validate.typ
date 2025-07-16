#let validate = (
  /// Checks if a string is a valid email format.
  /// - email: The string to validate.
  /// Returns true if the string is a valid email format.
  "email": email => {
    str.find(email, "@") != none and str.find(email, ".") != none
  },
  /// Checks if a string is a valid URL.
  /// - url: The string to validate.
  /// Returns true if the string is a valid URL.
  "url": url => {
    str.starts-with(url, "http://") or str.starts-with(url, "https://")
  },
  /// Checks if a value is a number, or within a certain range.
  /// - value: The value to check.
  /// - min: Optional minimum value.
  /// - max: Optional maximum value.
  /// Returns true if the value is a number (and within range if specified).
  "number": (value, min: none, max: none) => {
    type(value) == number and (min == none or value >= min) and (max == none or value <= max)
  },
  /// Checks if a value is present and not empty/null.
  /// - value: The value to check.
  /// Returns true if the value is not none and not empty.
  "required": value => {
    value != none and (type(value) != string or str.length(value) > 0)
  },
  /// Checks the type of a variable.
  /// - value: The value to check.
  /// - expected: The expected type (as string).
  /// Returns true if the value matches the expected type.
  "type": (value, expected) => {
    type(value) == expected
  },
)
