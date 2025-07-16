// ======================================
// USAGE GUIDE
// ======================================
//
// Usage Example:
// #import "template.typ": *
// #show: template.with(
//   title: "My Awesome Document",
//   author: "John Doe",
//   config-override: (
//     layout: (show-title-page: true,),
//     theme: "dark", // or "light"
//   ),
// )

// ======================================
// CENTRALIZED CONFIGURATION SYSTEM
// ======================================
#let project = (
  metadata: (
    name: "datum",
    title: "Datum",
    subtitle: "From Data Enthusiast to Guru, One Datum at a Time",
    author: "Craig 'Craole' Cole",
    tagline: "Every data journey begins with a single datum.",
    version: "1.0.0",
    date: datetime.today(),
    keywords: ("data", "analysis", "documentation"),
    language: "en",
  ),
  layout: (
    paper: "us-letter",
    margin: (x: 1.8cm, top: 2.8cm, bottom: 2cm),
    show-title-page: true,
    show-toc: true,
    show-page-numbers: true,
    show-footer: true,
    show-header: false,
    line-height: 1.5,
    paragraph-spacing: 1.2em,
    paragraph-indent: 0em,
    paragraph-justify: true,
    list-indent: 1.5em,
    list-spacing: 0.6em,
    enum-indent: 1.5em,
    enum-spacing: 0.6em,
    enum-numbering: "1.a.i.",
    heading-numbering: "1.1",
    border-radius: 8pt,
    gradient-angle: 45deg,
    gradient-angle-secondary: 90deg,
    spacing: (
      xs: 0.4em,
      sm: 0.8em,
      md: 1.2em,
      lg: 1.8em,
      xl: 2.4em,
      xxl: 3.2em,
    ),
    shadow: (
      enabled: true,
      color: rgb("#00000015"),
      offset: (x: 0pt, y: 2pt),
      blur: 4pt,
    ),
    title-page: (
      margin: (x: 2cm, y: 3cm),
      logo-spacing: 2em,
      title-subtitle-spacing: 0.8em,
      decorative-spacing: 1.5em,
      footer-spacing: 1cm,
      grid-gutter: 2em,
    ),
    toc: (
      header-inset: 1.5em,
      header-spacing: 1.5em,
      entry-spacing: (
        level1: 0.8em,
        level2: 0.4em,
        level3: 0.2em,
      ),
      depth: 3,
      indent: 0em,
      entry-indent: (
        level1: 0em,
        level2: 1em,
        level3: 2em,
      ),
    ),
    callout: (
      inset: 1.5em,
      icon-spacing: 0.8em,
      title-spacing: 0.8em,
      border-width: (left: 4pt, rest: 1pt),
      bg-lighten-amount: 3%,
    ),
    code: (
      inset: 1.5em,
      header-inset: (x: 1.5em, y: 0.8em),
      content-inset: 1.5em,
      border-width: 1pt,
    ),
    table: (
      border-width: 1pt,
      cell-padding: (x: 1em, y: 0.6em),
      header-padding: (x: 1em, y: 0.8em),
      zebra-enabled: true,
    ),
    heading: (
      spacing: (
        before: (h1: "lg", h2: "lg", h3: "md", h4: "md"),
        after: (h1: "md", h2: "md", h3: "sm", h4: "sm"),
      ),
      inset: 1.5em,
      underline-width: 2pt,
      underline-spacing: 0.5em,
    ),
    footer: (
      column-count: 3,
      column-gutter: 2em,
      alignment: (left: left, center: center, right: right),
    ),
    page-numbering: (
      format: "1",
      start: 1,
    ),
    link: (
      decoration: none,
      weight: "medium",
    ),
    emphasis: (
      style: "italic",
    ),
    strong: (
      weight: "bold",
    ),
    list-markers: ([•], [◦], [▪]),
    text-hyphenate: true,
    decorative-line: (
      widths: (40%, 60%, 30%),
      strokes: (3pt, 1pt, 2pt),
      spacing: 0.5em,
    ),
  ),
  font: (
    family: (
      serif: ("New Computer Modern", "Times New Roman"),
      sans: ("Dank Mono", "Arial"),
      mono: ("Maple Mono NF", "Monaspace Radon Var", "Consolas"),
      display: ("Monaspace Radon Var", "Arial"),
    ),
    size: (
      base: 11pt,
      title: 2.6em,
      subtitle: 1.4em,
      h1: 2.0em,
      h2: 1.6em,
      h3: 1.3em,
      h4: 1.1em,
      h5: 1.0em,
      small: 0.9em,
      footnote: 0.8em,
      caption: 0.85em,
      copyright: 0.7em,
      icon: 1.2em,
      code-header: 0.9em,
    ),
    weight: (
      thin: "thin",
      light: "light",
      normal: "regular",
      medium: "medium",
      semibold: "semibold",
      bold: "bold",
      extrabold: "extrabold",
      black: "black",
    ),
  ),
  themes: (
    light: (
      primary: rgb("#2563eb"),
      secondary: rgb("#7c3aed"),
      accent: rgb("#059669"),
      text: rgb("#1f2937"),
      text-light: rgb("#6b7280"),
      text-muted: rgb("#9ca3af"),
      bg: rgb("#ffffff"),
      bg-alt: rgb("#f9fafb"),
      bg-code: rgb("#f3f4f6"),
      border: rgb("#e5e7eb"),
      border-light: rgb("#f3f4f6"),
      success: rgb("#059669"),
      warning: rgb("#d97706"),
      error: rgb("#dc2626"),
      info: rgb("#2563eb"),
    ),
    dark: (
      primary: rgb("#60a5fa"),
      secondary: rgb("#a78bfa"),
      accent: rgb("#34d399"),
      text: rgb("#f9fafb"),
      text-light: rgb("#d1d5db"),
      text-muted: rgb("#9ca3af"),
      bg: rgb("#111827"),
      bg-alt: rgb("#1f2937"),
      bg-code: rgb("#374151"),
      border: rgb("#374151"),
      border-light: rgb("#4b5563"),
      success: rgb("#10b981"),
      warning: rgb("#f59e0b"),
      error: rgb("#ef4444"),
      info: rgb("#3b82f6"),
    ),
  ),
  content: (
    toc-title: "Table of Contents",
    code-label: "CODE",
    date-format: "[month repr:long] [day], [year]",
    author-label: "Author",
    date-label: "Date",
  ),
  icons: (
    tech: (
      python: "🐍",
      rust: "🦀",
      database: "🗄️",
      cloud: "☁️",
      docker: "🐳",
      git: "📦",
      spark: "⚡",
      airflow: "🌊",
      kafka: "📡",
      kubernetes: "☸️",
      typescript: "📘",
      javascript: "📙",
      react: "⚛️",
      vue: "💚",
      angular: "🅰️",
    ),
    document: (
      roadmap: "🗺️",
      toolchain: "🔧",
      project: "📁",
      learning: "📚",
      architecture: "🏗️",
      api: "🔌",
      guide: "📖",
      reference: "📋",
      tutorial: "🎓",
    ),
    status: (
      todo: "⏳",
      doing: "🔄",
      done: "✅",
      blocked: "🚫",
      review: "👀",
      experimental: "🧪",
      stable: "✅",
      deprecated: "⚠️",
      archive: "📦",
    ),
    priority: (
      critical: "🔥",
      high: "🔴",
      medium: "🟡",
      low: "🟢",
      zero: "⚪",
    ),
    misc: (
      idea: "💡",
      note: "📝",
      tip: "💡",
      warning: "⚠️",
      error: "❌",
      success: "✅",
      info: "ℹ️",
      question: "❓",
      star: "⭐",
      heart: "❤️",
    ),
  ),
)

// ======================================
// UTILITY FUNCTIONS
// ======================================
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

// ======================================
// ENHANCED COMPONENTS
// ======================================

#let create-title-page = (
  title: "",
  subtitle: "",
  author: none,
  date: none,
  logo: none,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let layout = config.layout.title-page
  let decorative = config.layout.decorative-line

  page(margin: layout.margin, background: rect(width: 100%, height: 100%, fill: gradient.linear(
    colors.bg,
    colors.bg-alt,
    angle: config.layout.gradient-angle,
  )))[
    #set text(fill: colors.text)

    #v(1fr)

    // Logo section with modern styling
    #if logo != none {
      align(center)[
        #box(
          fill: colors.bg,
          stroke: config.layout.table.border-width + colors.border,
          radius: config.layout.border-radius,
          inset: config.layout.callout.inset,
        )[#logo]
      ]
      v(layout.logo-spacing)
    }

    // Title with gradient effect
    #align(center)[
      #text(
        size: get-font-size("title"),
        weight: get-font-weight("black"),
        font: get-font("display"),
        fill: gradient.linear(
          colors.primary,
          colors.secondary,
          angle: config.layout.gradient-angle,
        ),
      )[#title]
    ]

    // Subtitle with elegant styling
    #if subtitle != "" {
      v(layout.title-subtitle-spacing)
      align(center)[
        #text(
          size: get-font-size("subtitle"),
          weight: get-font-weight("light"),
          font: get-font("sans"),
          fill: colors.text-light,
          style: config.layout.emphasis.style,
        )[#subtitle]
      ]
    }

    #v(2fr)

    // Modern decorative line
    #align(center)[
      #stack(
        spacing: decorative.spacing,
        line(length: decorative.widths.at(0), stroke: decorative.strokes.at(0) + colors.primary),
        line(length: decorative.widths.at(1), stroke: decorative.strokes.at(1) + colors.secondary),
        line(length: decorative.widths.at(2), stroke: decorative.strokes.at(2) + colors.accent),
      )
    ]

    #v(layout.decorative-spacing)

    // Enhanced footer with better typography
    #grid(
      columns: (1fr, 1fr),
      column-gutter: layout.grid-gutter,
      align: (left, right),

      // Author section
      box(
        fill: colors.bg-alt,
        stroke: config.layout.table.border-width + colors.border,
        radius: config.layout.border-radius,
        inset: 1em,
      )[
        #text(
          size: get-font-size("small"),
          weight: get-font-weight("medium"),
          fill: colors.text-muted,
        )[*#get-content("author-label")*]
        #v(0.3em)
        #text(
          size: get-font-size("base"),
          weight: get-font-weight("semibold"),
          fill: colors.text,
        )[#author]
      ],

      // Date section
      box(
        fill: colors.bg-alt,
        stroke: config.layout.table.border-width + colors.border,
        radius: config.layout.border-radius,
        inset: 1em,
      )[
        #text(
          size: get-font-size("small"),
          weight: get-font-weight("medium"),
          fill: colors.text-muted,
        )[*#get-content("date-label")*]
        #v(0.3em)
        #text(
          size: get-font-size("base"),
          weight: get-font-weight("semibold"),
          fill: colors.text,
        )[#date.display(get-content("date-format"))]
      ],
    )

    #v(layout.footer-spacing)
  ]

  pagebreak()
}

#let create-toc = (
  title: none,
  depth: none,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let toc-config = config.layout.toc
  let toc-title = if title != none { title } else { get-content("toc-title") }
  let toc-depth = if depth != none { depth } else { config.layout.toc.depth }

  // Modern TOC header
  block(
    fill: gradient.linear(
      colors.primary,
      colors.secondary,
      angle: config.layout.gradient-angle-secondary,
    ),
    stroke: none,
    radius: config.layout.border-radius,
    inset: toc-config.header-inset,
    width: 100%,
  )[
    #text(size: get-font-size("h1"), weight: get-font-weight("bold"), fill: colors.bg, font: get-font(
      "display",
    ))[#toc-title]
  ]

  v(toc-config.header-spacing)

  // Enhanced outline styling
  show outline.entry.where(level: 1): it => {
    v(toc-config.entry-spacing.level1, weak: true)
    box(
      fill: colors.bg-alt,
      stroke: config.layout.table.border-width + colors.border,
      radius: config.layout.border-radius,
      inset: (x: 1em, y: 0.6em),
      width: 100%,
    )[
      #text(weight: get-font-weight("semibold"), fill: colors.text, font: get-font("sans"))[#it]
    ]
  }

  show outline.entry.where(level: 2): it => {
    v(toc-config.entry-spacing.level2, weak: true)
    pad(left: toc-config.entry-indent.level2, text(fill: colors.text-light, font: get-font("sans"))[#it])
  }

  show outline.entry.where(level: 3): it => {
    v(toc-config.entry-spacing.level3, weak: true)
    pad(left: toc-config.entry-indent.level3, text(fill: colors.text-muted, font: get-font("sans"))[#it])
  }

  outline(
    title: none,
    depth: toc-depth,
    indent: toc-config.indent,
  )

  pagebreak(weak: true)
}

#let create-code-block = (
  code,
  language: none,
  caption: none,
  line-numbers: false,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let code-config = config.layout.code

  figure(
    block(
      fill: colors.bg-code,
      stroke: code-config.border-width + colors.border,
      radius: config.layout.border-radius,
      width: 100%,
      clip: true,
    )[
      #if language != none {
        // Enhanced code header
        block(
          fill: colors.border,
          inset: code-config.header-inset,
          width: 100%,
        )[
          #grid(
            columns: (1fr, auto),
            align: (left, right),

            text(size: get-font-size("small"), weight: get-font-weight("medium"), fill: colors.text, font: get-font(
              "mono",
            ))[#language.upper()],

            text(size: get-font-size("small"), fill: colors.text-muted, font: get-font(
              "sans",
              config,
            ))[#get-content("code-label")],
          )
        ]
      }

      // Code content with improved styling
      block(
      inset: code-config.content-inset,
      width: 100%,
      )[
      #set text(
        font: get-font("mono"),
        size: get-font-size("small"),
        fill: colors.text,
      )
      #if line-numbers {
        // TODO: Implement line numbers
        raw(code, block: true)
      } else {
        raw(code, block: true)
      }
      ]
    ],
    caption: caption,
    kind: raw,
  )
}

#let create-callout = (
  content,
  type: "info",
  title: none,
  icon: none,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let callout-config = config.layout.callout

  let styles = (
    info: (
      border: colors.info,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.info,
    ),
    warning: (
      border: colors.warning,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.warning,
    ),
    success: (
      border: colors.success,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.success,
    ),
    error: (
      border: colors.error,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.error,
    ),
    tip: (
      border: colors.accent,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.tip,
    ),
  )

  let style = styles.at(type, default: styles.info)
  let callout-icon = if icon != none { icon } else { style.icon }

  block(
    fill: style.bg,
    stroke: (
      left: callout-config.border-width.left + style.border,
      rest: callout-config.border-width.rest + colors.border,
    ),
    radius: (right: config.layout.border-radius, left: 2pt),
    inset: callout-config.inset,
    width: 100%,
  )[
    #if title != none or callout-icon != none {
      grid(
        columns: (auto, 1fr),
        column-gutter: callout-config.icon-spacing,
        align: (center, left),

        if callout-icon != none {
          text(size: get-font-size("icon"))[#callout-icon]
        },

        if title != none {
          text(size: get-font-size("base"), weight: get-font-weight("bold"), fill: style.border, font: get-font(
            "sans",
          ))[#title]
        },
      )

      v(callout-config.title-spacing)
    }

    #set text(fill: colors.text, font: get-font("sans"))
    #content
  ]
}

#let create-table_OLD = (
  headers: (),
  data: (),
  caption: none,
  align: left,
  zebra: none,
  config: project,
  theme: none,
) => {
  let theme = get-value(config, "theme", default: "dark")
  let colors = get-value(config.themes, theme, default: config.themes.dark)
  let table-config = config.layout.table
  let use-zebra = if zebra != none { zebra } else { table-config.zebra-enabled }

  assert(headers.len() > 0, message: "Headers cannot be empty")
  assert(data.len() > 0, message: "Data cannot be empty")

  let flattened = if type(data.at(0)) == array { data.flatten() } else { data }
  let rows = calc.floor(flattened.len() / headers.len())
  let normalized = flattened.slice(0, rows * headers.len())

  figure(
    block(
      stroke: table-config.border-width + colors.border,
      radius: config.layout.border-radius,
      clip: true,
      width: 100%,
    )[
      #table(
        columns: headers.len(),
        stroke: none,
        align: align,
        fill: (_, y) => {
          if y == 0 {
            gradient.linear(
              colors.primary,
              colors.secondary,
              angle: config.layout.gradient-angle-secondary,
            )
          } else if use-zebra and calc.odd(y) {
            colors.bg-alt
          } else {
            colors.bg-alt
            none
          }
        },

        // Enhanced headers
        ..headers.map(h => pad(x: table-config.header-padding.x, y: table-config.header-padding.y, text(
          weight: get-font-weight("bold"),
          fill: colors.bg,
          font: get-font("sans"),
        )[#h])),

        // Enhanced data cells
        ..normalized.map(cell => pad(x: table-config.cell-padding.x, y: table-config.cell-padding.y, text(
          fill: colors.text,
          font: get-font("sans"),
        )[#cell])),
      )
    ],
    caption: caption,
    kind: table,
  )
}
#let create-table(headers: (), data: (), theme: "dark") = {
  // Theme-aware colors
  let header-fill = if theme == "light" {
    rgb("#f7fafc") // Light gray for light theme
  } else {
    rgb("#2d3748") // Dark gray for dark theme
  }

  let border-color = if theme == "light" {
    rgb("#4a5568") // Dark border
  } else {
    rgb("#e2e8f0") // Light border
  }

  let text-color = if theme == "light" {
    rgb("#1a202c") // Dark text on light background
  } else {
    rgb("#4a5568") // Dark border
  }

  table(
    columns: headers.len(),
    fill: (x, y) => if y == 0 { header-fill } else { none },
    stroke: border-color,
    ..headers.map(h => text(fill: text-color, h)),
    ..data.flatten().map(d => text(fill: text-color, d))
  )
}

// ======================================
// ENHANCED STYLING SYSTEM
// ======================================
#let apply-heading-styles = config => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let heading-config = config.layout.heading

  show heading.where(level: 1): it => {
    pagebreak(weak: true)
    v(get-spacing(heading-config.spacing.before.h1))

    block(
      fill: gradient.linear(
        colors.primary,
        colors.secondary,
        angle: config.layout.gradient-angle-secondary,
      ),
      stroke: none,
      radius: config.layout.border-radius,
      inset: heading-config.inset,
      width: 100%,
    )[
      #text(size: get-font-size("h1"), weight: get-font-weight("bold"), fill: colors.bg, font: get-font(
        "display",
        config,
      ))[#it]
    ]

    v(get-spacing(heading-config.spacing.after.h1))
  }

  show heading.where(level: 2): it => {
    v(get-spacing(heading-config.spacing.before.h2))

    stack(
      spacing: heading-config.underline-spacing,
      text(size: get-font-size("h2"), weight: get-font-weight("semibold"), fill: colors.text, font: get-font(
        "sans",
      ))[#it],
      line(length: 100%, stroke: heading-config.underline-width + colors.primary),
    )

    v(get-spacing(heading-config.spacing.after.h2))
  }

  show heading.where(level: 3): it => {
    v(get-spacing(heading-config.spacing.before.h3))

    text(size: get-font-size("h3"), weight: get-font-weight("semibold"), fill: colors.primary, font: get-font(
      "sans",
    ))[#it]

    v(get-spacing(heading-config.spacing.after.h3))
  }

  show heading.where(level: 4): it => {
    v(get-spacing(heading-config.spacing.before.h4))

    text(size: get-font-size("h4"), weight: get-font-weight("medium"), fill: colors.text, font: get-font("sans"))[#it]

    v(get-spacing(heading-config.spacing.after.h4))
  }
}

#let create-footer = (show-numbers, config: project) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let footer-config = config.layout.footer

  context {
    let page-num = counter(page).at(here()).first()

    if show-numbers {
      grid(
        columns: (1fr, auto, 1fr),
        column-gutter: footer-config.column-gutter,
        align: (
          footer-config.alignment.left,
          footer-config.alignment.center,
          footer-config.alignment.right,
        ),

        text(
          size: get-font-size("footnote"),
          fill: colors.text-muted,
          style: config.layout.emphasis.style,
        )[#config.metadata.tagline],

        text(size: get-font-size("footnote"), fill: colors.text-muted, weight: get-font-weight(
          "medium",
        ))[#page-num],

        text(
          size: get-font-size("footnote"),
          fill: colors.text-muted,
        )[#config.metadata.title],
      )
    } else {
      align(center)[
        #text(
          size: get-font-size("footnote"),
          fill: colors.text-muted,
          style: config.layout.emphasis.style,
        )[#config.metadata.tagline]
      ]
    }
  }
}

// ======================================
// MAIN TEMPLATE FUNCTION
// ======================================
#let template = (
  title: none,
  subtitle: none,
  author: none,
  theme: none,
  show-title-page: none,
  show-toc: none,
  show-footer: none,
  show-page-numbers: none,
  config-override: (:),
  body,
) => {
  // Merge configurations
  let config = deep-merge(project, config-override)
  if theme != none { config.insert("theme", theme) }

  validate-config(config)

  // Update metadata
  if title != none { config.metadata.insert("title", title) }
  if subtitle != none { config.metadata.insert("subtitle", subtitle) }
  if author != none { config.metadata.insert("author", author) }

  // Update layout settings with parameter overrides
  let show-title-page-final = if show-title-page != none { show-title-page } else { config.layout.show-title-page }
  let show-toc-final = if show-toc != none { show-toc } else { config.layout.show-toc }
  let show-footer-final = if show-footer != none { show-footer } else { config.layout.show-footer }
  let show-page-numbers-final = if show-page-numbers != none { show-page-numbers } else {
    config.layout.show-page-numbers
  }

  let colors = get-value(config.themes, theme, default: config.themes.dark)

  // Document metadata
  set document(
    title: config.metadata.title,
    author: config.metadata.author,
    date: config.metadata.date,
    keywords: config.metadata.keywords,
  )

  // Enhanced page setup
  set page(
    paper: config.layout.paper,
    margin: config.layout.margin,
    numbering: none,
    footer: none,
    fill: colors.bg,
  )

  // Enhanced text settings
  set text(
    font: get-font("sans"),
    size: get-font-size("base"),
    fill: colors.text,
    hyphenate: config.layout.text-hyphenate,
    lang: config.metadata.language,
  )

  // Enhanced paragraph settings
  set par(
    leading: config.layout.line-height * 1em,
    spacing: config.layout.paragraph-spacing,
    justify: config.layout.paragraph-justify,
    first-line-indent: config.layout.paragraph-indent,
  )

  // Enhanced list styling
  set list(
    indent: config.layout.list-indent,
    spacing: config.layout.list-spacing,
    marker: config.layout.list-markers,
  )

  set enum(
    indent: config.layout.enum-indent,
    spacing: config.layout.enum-spacing,
    numbering: config.layout.enum-numbering,
  )

  // Link styling
  show link: it => {
    text(fill: colors.primary, weight: get-font-weight(config.layout.link.weight))[#it]
  }

  // Enhanced emphasis
  show emph: it => {
    text(
      style: config.layout.emphasis.style,
      fill: colors.text-light,
    )[#it]
  }

  show strong: it => {
    text(
      weight: get-font-weight(config.layout.strong.weight),
      fill: colors.text,
    )[#it]
  }

  // Heading configuration
  set heading(numbering: config.layout.heading-numbering)
  apply-heading-styles(config)

  // Front matter
  if show-title-page-final {
    create-title-page(
      title: config.metadata.title,
      subtitle: config.metadata.subtitle,
      author: config.metadata.author,
      date: config.metadata.date,
      config: config,
    )
  }

  if show-toc-final {
    create-toc(config: config)
  }

  // Main content setup
  set page(
    paper: config.layout.paper,
    margin: config.layout.margin,
    numbering: config.layout.page-numbering.format,
    footer: if show-footer-final {
      create-footer(show-page-numbers-final)
    } else { none },
    fill: colors.bg,
  )

  counter(page).update(config.layout.page-numbering.start)

  // Render content
  body
}
