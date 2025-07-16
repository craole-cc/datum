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