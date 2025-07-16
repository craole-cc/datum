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