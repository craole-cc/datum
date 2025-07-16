#let format = (
  /// Formats a given date into a specified string format.
  /// - date: The datetime object to format. Defaults to today's date.
  /// - format: The desired format for the date string ("standard", "long", "short", "day", "month", "year").
  /// Returns the formatted date string.
  "date": (date: datetime.today(), format: "standard") => {
    let formats = (
      standard: date.display("[year]-[month]-[day]"),
      long: date.display("[month repr:long] [day], [year]"),
      short: date.display("[month repr:short] [day], [year]"),
      time: date.display("[hour]:[minute]:[second]"),
      datetime: date.display("[year]-[month]-[day] [hour]:[minute]:[second]"),
      iso: date.display("[year]-[month]-[day]T[hour]:[minute]:[second]"),
      full: date.display("[weekday repr:long], [month repr:long] [day], [year] [hour]:[minute]:[second]"),
      day: date.display("[day]"),
      day-padded: date.display("[day padding:zero]"),
      month: date.display("[month]"),
      month-padded: date.display("[month padding:zero]"),
      year: date.display("[year]"),
      year-last-two: date.display("[year repr:last_two]"),
    )
    formats.at(format, default: formats.standard)
  },
  /// Formats a given number with a specified number of decimal places and optional thousands separator.
  /// - value: The number to format.
  /// - decimals: The number of decimal places to include. Defaults to 0.
  /// - thousands: Whether to include a thousands separator. Defaults to false.
  /// Returns the formatted number string.
  "number": (value, decimals: 0, thousands: false) => {
    let s = "" + calc.round(value, digits: decimals)
    if thousands {
      // Add thousands separator (e.g., 1,000,000)
      let parts = str.split(s, ".")
      let integer = parts.at(0)
      let formatted_integer = ""
      for i in range(integer.len() - 1, -1, step: -1) {
        formatted_integer = integer.at(i) + formatted_integer
        if i > 0 and calc.rem(integer.len() - i, 3) == 0 {
          formatted_integer = "," + formatted_integer
        }
      }
      s = formatted_integer
      if parts.len() > 1 {
        s += "." + parts.at(1)
      }
    }
    s
  },
  /// Formats a given number as currency.
  /// - value: The number to format.
  /// - symbol: The currency symbol (e.g., "$", "€"). Defaults to "$".
  /// - decimals: The number of decimal places. Defaults to 2.
  /// - thousands: Whether to include thousands separator. Defaults to true.
  /// Returns the formatted currency string.
  "currency": (value, symbol: "$", decimals: 2, thousands: true) => {
    symbol + format.number(value, decimals: decimals, thousands: thousands)
  },
  /// Formats a given number as a percentage.
  /// - value: The number to format (e.g., 0.5 for 50%).
  /// - decimals: The number of decimal places. Defaults to 0.
  /// Returns the formatted percentage string.
  "percentage": (value, decimals: 0) => {
    format.number(value * 100, decimals: decimals) + "%"
  },
  /// Trims a string to a specified length, adding an ellipsis if truncated.
  /// - text: The string to trim.
  /// - length: The maximum length of the string.
  /// Returns the trimmed string.
  "trim": (text, length) => {
    if text.len() <= length {
      text
    } else {
      text.slice(0, length - 3) + "..."
    }
  },
  /// Capitalizes the first letter of a string.
  /// - text: The string to capitalize.
  /// Returns the capitalized string.
  "capitalize": text => {
    if text.len() == 0 {
      ""
    } else {
      str.upper(text.slice(0, 1)) + text.slice(1)
    }
  },
  /// Converts a string to lowercase.
  /// - text: The string to convert.
  /// Returns the lowercase string.
  "lowercase": text => {
    str.lower(text)
  },
  /// Converts a string to uppercase.
  /// - text: The string to convert.
  /// Returns the uppercase string.
  "uppercase": text => {
    str.upper(text)
  },
  /// Converts a string to title case (first letter of each word capitalized).
  /// - text: The string to convert.
  /// Returns the title cased string.
  "titlecase": text => {
    let words = str.split(text, " ")
    let capitalized_words = ()
    for word in words {
      capitalized_words.push(format.capitalize(word))
    }
    str.join(capitalized_words, " ")
  },
  /// Converts a string to a URL-friendly slug.
  /// - text: The string to convert.
  /// Returns the slugified string.
  "slugify": text => {
    let s = str.lower(text)
    s = regex.replace(s, "[^a-z0-9\\s-]", "") // Remove non-alphanumeric characters
    s = regex.replace(s, "\\s+", "-") // Replace spaces with hyphens
    s = regex.replace(s, "-+", "-") // Replace multiple hyphens with a single hyphen
    s = str.trim(s, "-") // Trim hyphens from start/end
    s
  },
  /// Truncates a string to a specified length, adding an ellipsis if truncated.
  /// - text: The string to truncate.
  /// - length: The maximum length of the string.
  /// Returns the truncated string.
  "truncate": (text, length) => {
    if text.len() <= length {
      text
    } else {
      text.slice(0, length) + "..."
    }
  },
  /// Pads a string on the left with a specified character until it reaches a target length.
  /// - text: The string to pad.
  /// - length: The target length of the string.
  /// - char: The character to use for padding. Defaults to " ".
  /// Returns the padded string.
  "pad-left": (text, length, char: " ") => {
    if text.len() >= length {
      text
    } else {
      str.repeat(char, length - text.len()) + text
    }
  },
  /// Pads a string on the right with a specified character until it reaches a target length.
  /// - text: The string to pad.
  /// - length: The target length of the string.
  /// - char: The character to use for padding. Defaults to " ".
  /// Returns the padded string.
  "pad-right": (text, length, char: " ") => {
    if text.len() >= length {
      text
    } else {
      text + str.repeat(char, length - text.len())
    }
  },
  /// Pads a string on both sides with a specified character until it reaches a target length.
  /// - text: The string to pad.
  /// - length: The target length of the string.
  /// - char: The character to use for padding. Defaults to " ".
  /// Returns the padded string.
  "pad-center": (text, length, char: " ") => {
    if text.len() >= length {
      text
    } else {
      let pad_len = length - text.len()
      let pad_left = calc.floor(pad_len / 2)
      let pad_right = calc.ceil(pad_len / 2)
      str.repeat(char, pad_left) + text + str.repeat(char, pad_right)
    }
  },
  /// Converts a string to camelCase.
  /// - text: The string to convert.
  /// Returns the camelCased string.
  "camel-case": text => {
    let words = str.split(text, "-")
    let camel = words.at(0)
    for i in range(1, words.len()) {
      camel += format.capitalize(words.at(i))
    }
    camel
  },
  /// Converts a string to snake_case.
  /// - text: The string to convert.
  /// Returns the snake_cased string.
  "snake-case": text => {
    let s = str.lower(text)
    str.replace(s, " ", "_")
  },
  /// Converts a string to kebab-case.
  /// - text: The string to convert.
  /// Returns the kebab-cased string.
  "kebab-case": text => {
    let s = str.lower(text)
    str.replace(s, " ", "-")
  },
  /// Converts a string to PascalCase.
  /// - text: The string to convert.
  /// Returns the PascalCased string.
  "pascal-case": text => {
    let words = str.split(text, "-")
    let pascal = ""
    for word in words {
      pascal += format.capitalize(word)
    }
    pascal
  },
  /// Converts a string to sentence case (first letter of the first word capitalized, rest lowercase).
  /// - text: The string to convert.
  /// Returns the sentence cased string.
  "sentence-case": text => {
    if text.len() == 0 {
      ""
    } else {
      format.capitalize(str.lower(text))
    }
  },
  /// Converts a string to path/case.
  /// - text: The string to convert.
  /// Returns the path/cased string.
  "path-case": text => {
    let s = str.lower(text)
    str.replace(s, " ", "/")
  },
  /// Converts a string to dot.case.
  /// - text: The string to convert.
  /// Returns the dot.cased string.
  "dot-case": text => {
    let s = str.lower(text)
    str.replace(s, " ", ".")
  },
  /// Converts a string to Header-Case.
  /// - text: The string to convert.
  /// Returns the Header-Cased string.
  "header-case": text => {
    let words = str.split(text, " ")
    let header = ()
    for word in words {
      header.push(format.capitalize(word))
    }
    str.join(header, "-")
  },
  /// Converts a string to CONSTANT_CASE.
  /// - text: The string to convert.
  /// Returns the CONSTANT_CASED string.
  "constant-case": text => {
    let s = str.upper(text)
    str.replace(s, " ", "_")
  },
  /// Swaps the case of each character in a string.
  /// - text: The string to swap case.
  /// Returns the case-swapped string.
  "swap-case": text => {
    let result = ""
    for char in text {
      if char == str.upper(char) {
        result += str.lower(char)
      } else {
        result += str.upper(char)
      }
    }
    result
  },
  /// Removes HTML tags from a string.
  /// - text: The string to process.
  /// Returns the string with HTML tags removed.
  "strip-html": text => {
    regex.replace(text, "<[^>]*>", "")
  },
  /// Escapes HTML special characters in a string.
  /// - text: The string to escape.
  /// Returns the HTML-escaped string.
  "escape-html": text => {
    let s = text
    s = str.replace(s, "&", "&amp;")
    s = str.replace(s, "<", "&lt;")
    s = str.replace(s, ">", "&gt;")
    s = str.replace(s, "\"", "&quot;")
    s = str.replace(s, "'", "&#039;")
    s
  },
  /// Unescapes HTML entities in a string.
  /// - text: The string to unescape.
  /// Returns the HTML-unescaped string.
  "unescape-html": text => {
    let s = text
    s = str.replace(s, "&amp;", "&")
    s = str.replace(s, "&lt;", "<")
    s = str.replace(s, "&gt;", ">")
    s = str.replace(s, "&quot;", "\"")
    s = str.replace(s, "&#039;", "'")
    s
  },
  /// Removes all whitespace from a string.
  /// - text: The string to process.
  /// Returns the string with all whitespace removed.
  "strip-whitespace": text => {
    regex.replace(text, "\\s", "")
  },
  /// Replaces multiple whitespace characters with a single space and trims leading/trailing whitespace.
  /// - text: The string to process.
  /// Returns the string with normalized whitespace.
  "normalize-whitespace": text => {
    let s = regex.replace(text, "\\s+", " ")
    str.trim(s)
  },
  /// Reverses a string.
  /// - text: The string to reverse.
  /// Returns the reversed string.
  "reverse": text => {
    let result = ""
    for i in range(text.len() - 1, -1, step: -1) {
      result += text.at(i)
    }
    result
  },
  /// Pluralizes a word based on a count.
  /// - word: The word to pluralize.
  /// - count: The count to determine pluralization. Defaults to 0.
  /// - suffix: The suffix to add for pluralization. Defaults to "s".
  /// Returns the pluralized word.
  "pluralize": (word, count: 0, suffix: "s") => {
    if count == 1 {
      word
    } else {
      word + suffix
    }
  },
  /// Adds an ordinal suffix to a number (e.g., 1st, 2nd, 3rd).
  /// - n: The number to add the ordinal suffix to.
  /// Returns the number with the ordinal suffix.
  "ordinal": n => {
    if calc.rem(n, 10) == 1 and calc.rem(n, 100) != 11 {
      str(n) + "st"
    } else if calc.rem(n, 10) == 2 and calc.rem(n, 100) != 12 {
      str(n) + "nd"
    } else if calc.rem(n, 10) == 3 and calc.rem(n, 100) != 13 {
      str(n) + "rd"
    } else {
      str(n) + "th"
    }
  },
  /// Converts an integer to its Roman numeral representation.
  /// - num: The integer to convert (1-3999).
  /// Returns the Roman numeral string.
  "roman": num => {
    if num < 1 or num > 3999 {
      panic("Number out of range for Roman numerals (1-3999).")
    }

    let numerals = (
      (1000, "M"),
      (900, "CM"),
      (500, "D"),
      (400, "CD"),
      (100, "C"),
      (90, "XC"),
      (50, "L"),
      (40, "XL"),
      (10, "X"),
      (9, "IX"),
      (5, "V"),
      (4, "IV"),
      (1, "I"),
    )

    let result = ""
    let temp_num = num

    for (value, symbol) in numerals {
      while temp_num >= value {
        result += symbol
        temp_num -= value
      }
    }
    result
  },
)
