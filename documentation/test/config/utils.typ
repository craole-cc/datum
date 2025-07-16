#let get-value = (dict, key, default: none) => {
  if dict != none and key != none and key in dict { dict.at(key) } else { default } 
}

#let get-spacing = (size: "sm", config: project) => {
  if type(size) == str {
    get-value(config.layout.spacing, size, default: size)
  } else { size }
}

#let get-color = (name, config: project) => {
  let theme = get-value(config, "theme", default: "dark")
  let colors = get-value(config.themes, theme, default: config.themes.dark)
  get-value(colors, name, default: colors.primary)
}

#let get-font = (type, config: project) => {
  get-value(config.font.family, type, default: config.font.family.sans)
}

#let get-font-size = (size, config: project) => {
  get-value(config.font.size, size, default: config.font.size.base)
}

#let get-font-weight = (weight, config: project) => {
  get-value(config.font.weight, weight, default: config.font.weight.normal)
}

#let get-content = (key, config: project) => {
  get-value(config.content, key, default: "")
}

#let deep-merge = (base, override) => {
  let result = base
  for (key, value) in override {
    if key in result and type(result.at(key)) == dictionary and type(value) == dictionary {
      result.insert(key, deep-merge(result.at(key), value))
    } else {
      result.insert(key, value)
    }
  }
  result
}

#let validate-config = config => {
  let required = ("metadata", "layout", "font", "themes", "content", "icons")
  for key in required {
    assert(key in config, message: "Missing required config key: " + key)
  }
}