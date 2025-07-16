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
    ),
    note: (
      border: colors.primary,
      bg: colors.bg.lighten(callout-config.bg-lighten-amount),
      icon: config.icons.misc.note,
    ),
  )

  let current-style = get-value(styles, type, default: styles.info)
  let current-icon = if icon != none { icon } else { current-style.icon }
  let current-title = if title != none { title } else { get-content("callout-" + type) }

  block(
    fill: current-style.bg,
    stroke: (left: callout-config.border-width.left + current-style.border, rest: callout-config.border-width.rest + colors.border),
    radius: config.layout.border-radius,
    inset: callout-config.inset,
    width: 100%,
  )[
    #grid(
      columns: (auto, 1fr),
      gutter: callout-config.icon-spacing,
      align: (top, left),
      text(size: get-font-size("icon"), fill: current-style.border)[#current-icon],
      block()[
        #text(size: get-font-size("h5"), weight: get-font-weight("bold"), fill: colors.text)[#current-title]
        #v(callout-config.title-spacing)
        #content
      ],
    )
  ]
}