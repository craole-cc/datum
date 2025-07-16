#let create-footer = (
  show-page-numbers: true,
  config: project,
) => {
  let colors = get-value(config.themes, get-value(config, "theme", default: "dark"), default: config.themes.dark)
  let footer-config = config.layout.footer

  set text(size: get-font-size("small"), fill: colors.text-muted)

  grid(
    columns: footer-config.column-count,
    gutter: footer-config.column-gutter,
    align: (footer-config.alignment.left, footer-config.alignment.center, footer-config.alignment.right),
    // Left column: Project name and version
    [#text(weight: get-font-weight("bold"))[#config.metadata.name] #config.metadata.version],
    // Center column: Copyright
    [© #datetime.today().year #config.metadata.author],
    // Right column: Page number
    if show-page-numbers {
      [#text(weight: get-font-weight("bold"))[#get-content("page-label")] #counter(page).display(config.layout.page-numbering.format)]
    } else {
      []
    },
  )
}