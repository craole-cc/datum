#let query = (
  /// Retrieves a value from a dictionary, with an optional default.
  /// - dict: The dictionary to retrieve the value from.
  /// - key: The key whose value to retrieve.
  /// - default: The default value to return if the key is not found or inputs are none.
  /// Returns the value associated with the key, or the default value.
  "value": (dict, key, default: none) => {
    if dict != none and key != none and key in dict { dict.at(key) } else { default }
  },
  /// Formats a given date into a specified string format.
  /// - date: The datetime object to format. Defaults to today's date.
  /// - format: The desired format for the date string ("standard", "long", "short").
  /// Returns the formatted date string.
  "date": (date: datetime.today(), format: "standard") => {
    let formats = (
      standard: date.display("[year]-[month]-[day]"),
      long: date.display("[month repr:long] [day], [year]"),
      short: date.display("[month repr:short] [day], [year]"),
    )
    formats.at(format, default: formats.standard)
  },
)
