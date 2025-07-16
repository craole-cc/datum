#let create-table_OLD = (
  data,
  header: none,
  caption: none,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let table-config = config.layout.table

  figure(
    table(
      stroke: table-config.border-width + colors.border,
      fill: (row, col) => {
        if table-config.zebra-enabled and row % 2 == 1 {
          return colors.bg-alt
        }
        return colors.bg
      },
      inset: table-config.cell-padding,
      align: auto,
      auto,
      // Table header
      if header != none {
        table.header(
          stroke: table-config.border-width + colors.border,
          fill: colors.bg-alt,
          inset: table-config.header-padding,
          ..header.map(it => text(weight: get-font-weight("bold"))[#it])
        )
      },
      // Table body
      ..data
    ),
    caption: caption,
    kind: "Table",
  )
}