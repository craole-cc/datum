#let merge = (
  /// Recursively merges two dictionaries.
  /// If a key exists in both dictionaries and both values are dictionaries, they are merged recursively.
  /// Otherwise, the value from `override` takes precedence.
  /// - base: The base dictionary.
  /// - override: The dictionary with overriding values.
  /// Returns a new dictionary with merged values.
  "value": (base, override) => {
    let result = base
    for (key, value) in override {
      if key in result and type(result.at(key)) == dictionary and type(value) == dictionary {
        result.insert(key, merge.value(result.at(key), value))
      } else {
        result.insert(key, value)
      }
    }
    result
  },
  /// Merges two or more objects. If a key exists in multiple objects, the value from the later object takes precedence.
  /// - objects: A list of dictionaries to merge.
  /// Returns a new dictionary with merged values.
  "object": objects => {
    let result = (:)
    for obj in objects {
      for (key, value) in obj {
        result.insert(key, value)
      }
    }
    result
  },
  /// Merges two or more arrays. By default, concatenates arrays. Can be extended for unique elements or merging by key.
  /// - arrays: A list of arrays to merge.
  /// Returns a new array with merged values.
  "array": arrays => {
    let result = ()
    for arr in arrays {
      result += arr
    }
    result
  },
  /// Merges a configuration object with default values, applying defaults only for properties not already present.
  /// - config: The configuration dictionary.
  /// - defaults: The default values dictionary.
  /// Returns a new dictionary with defaults applied.
  "defaults": (config, defaults) => {
    let result = defaults
    for (key, value) in config {
      result.insert(key, value)
    }
    result
  },
)
