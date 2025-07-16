#let create-title-page = (
  title: none,
  subtitle: none,
  author: none,
  date: none,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let layout = config.layout.title-page

  set page(margin: layout.margin)

  align(center)[
    #v(layout.logo-spacing)

    #image("../images/logo.svg", width: 15%)

    #v(layout.title-subtitle-spacing)

    #text(size: get-font-size("title"), weight: get-font-weight("bold"), fill: colors.primary, font: get-font(
      "display",
    ))[#title]

    #v(layout.title-subtitle-spacing)

    #text(size: get-font-size("subtitle"), weight: get-font-weight("medium"), fill: colors.secondary, font: get-font(
      "display",
    ))[#subtitle]

    #v(layout.decorative-spacing)

    #grid(
      columns: (1fr, 1fr, 1fr),
      gutter: layout.grid-gutter,
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