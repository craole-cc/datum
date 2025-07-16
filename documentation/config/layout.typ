
#let paper = "us-letter"
#let margin = (x: 1.8cm, top: 2.8cm, bottom: 2cm)
#let show-title-page = true
#let show-toc = true
#let show-page-numbers = true
#let show-footer = true
#let show-header = false
#let line-height = 1.5
#let paragraph-spacing = 1.2em
#let paragraph-indent = 0em
#let paragraph-justify = true
#let list-indent = 1.5em
#let list-spacing = 0.6em
#let enum-indent = 1.5em
#let enum-spacing = 0.6em
#let enum-numbering = "1.a.i."
#let heading-numbering = "1.1"
#let border-radius = 8pt
#let gradient-angle = 45deg
#let gradient-angle-secondary = 90deg
#let spacing = (
  xs: 0.125em,
  sm: 0.3em,
  md: 0.8em,
  lg: 1.2em,
  xl: 2.4em,
  xxl: 3.2em,
)
#let shadow = (
  enabled: true,
  color: rgb("#00000015"),
  offset: (x: 0pt, y: 2pt),
  blur: 4pt,
)
#let title-page = (
  margin: (x: 2cm, y: 3cm),
  logo-spacing: 2em,
  title-subtitle-spacing: 0.8em,
  decorative-spacing: 1.5em,
  footer-spacing: 1cm,
  grid-gutter: 2em,
)

#let toc = (
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
)
#let callout = (
  inset: 1.5em,
  icon-spacing: 0.8em,
  title-spacing: 0.8em,
  border-width: (left: 4pt, rest: 1pt),
  bg-lighten-amount: 3%,
)
#let code = (
  inset: 1.5em,
  header-inset: (x: 1.5em, y: 0.8em),
  content-inset: 1.5em,
  border-width: 1pt,
)
#let table = (
  border-width: 1pt,
  cell-padding: (x: 1em, y: 0.6em),
  header-padding: (x: 1em, y: 0.8em),
  zebra-enabled: true,
)
#let heading = (
  spacing: (
    before: (h1: "lg", h2: "lg", h3: "md", h4: "md"),
    after: (h1: "md", h2: "md", h3: "sm", h4: "sm"),
  ),
  inset: 1.5em,
  underline-width: 2pt,
  underline-spacing: 0.5em,
)
#let footer = (
  column-count: 3,
  column-gutter: 2em,
  alignment: (left: left, center: center, right: right),
)
#let page-numbering = (
  format: "1",
  start: 1,
)
#let link = (
  decoration: none,
  weight: "medium",
)
#let emphasis = (
  style: "italic",
)
#let strong = (
  weight: "bold",
)
#let list-markers = ([•], [◦], [▪])
#let text-hyphenate = true
#let decorative-line = (
  widths: (40%, 60%, 30%),
  strokes: (3pt, 1pt, 2pt),
  spacing: 0.5em,
)
