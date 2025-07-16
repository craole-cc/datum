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