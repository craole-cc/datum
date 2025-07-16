#let component = (
  /// Creates a footer element for the document.
  /// - content: The content to display in the footer. Can be a single element or array for columns.
  /// - columns: Number of columns (auto-detected from content array length if not specified).
  /// - gutter: Space between columns (default: 1em).
  /// - alignment: Text alignment ("left", "center", "right") or array of alignments per column.
  /// - style: Additional styling options.
  /// - fill: Background fill color.
  /// - stroke: Border stroke options.
  /// - inset: Internal padding (default: 0.5em).
  /// - height: Fixed height for the footer.
  /// - page-numbering: Page numbering options (none, "1", "1 of 1", "Page 1", "Page 1 of 1", custom function).
  /// - page-position: Where to place page numbers ("left", "center", "right", or column index).
  /// Returns a footer element.
  ///
  /// USAGE EXAMPLES:
  ///
  /// // Simple single column footer
  /// #component.footer("Simple footer")
  ///
  /// // Multi-column footer with array of content
  /// #component.footer(
  ///   ("Left content", "Center content", "Right content"),
  ///   alignment: ("left", "center", "right")
  /// )
  ///
  /// // Custom styling and spacing
  /// #component.footer(
  ///   ("Author: John Doe", format.date(format: "standard")),
  ///   columns: 2,
  ///   gutter: 2em,
  ///   style: (size: 9pt, fill: gray),
  ///   fill: rgb("#f0f0f0"),
  ///   stroke: (top: 1pt + black),
  ///   inset: 1em
  /// )
  ///
  /// // Page numbering in center
  /// #component.footer(
  ///   ("© 2025", "Company Name"),
  ///   page-numbering: "Page 1 of 1",
  ///   page-position: "center"
  /// )
  ///
  /// // Page number with tagline stacked vertically
  /// #component.footer(
  ///   ("Left content", "Right content"),
  ///   page-numbering: () => {
  ///     align(center)[
  ///       #str(here().page()) \
  ///       #text(size: 8pt, fill: gray)[Your tagline here]
  ///     ]
  ///   },
  ///   page-position: "center"
  /// )
  ///
  /// // Three columns with different alignments and borders
  /// #component.footer(
  ///   ("© 2025", "Page " + str(here().page()), "Company Name"),
  ///   alignment: ("left", "center", "right"),
  ///   gutter: 1.5em,
  ///   stroke: (top: 0.5pt + gray.lighten(40%))
  /// )
  ///
  /// // Fixed height footer with custom page numbering
  /// #component.footer(
  ///   "Document Title",
  ///   height: 3em,
  ///   page-numbering: () => {
  ///     "Section " + str(here().page()) + " • " + format.date(format: "short")
  ///   },
  ///   page-position: "right",
  ///   fill: rgb("#e6e6e6"),
  ///   stroke: (top: 2pt + blue)
  /// )
  ///
  /// // Insert page numbering at specific column index
  /// #component.footer(
  ///   ("Left", "Right"),
  ///   page-numbering: "1 of 1",
  ///   page-position: 1,  // Insert at index 1 (between Left and Right)
  ///   alignment: ("left", "center", "right")
  /// )
  ///
  /// // Just page numbering with vertical spacing
  /// #component.footer(
  ///   (),
  ///   page-numbering: () => {
  ///     align(center)[
  ///       #text(weight: "bold")[#str(here().page())] \
  ///       #v(0.2em) \
  ///       #text(size: 8pt, style: "italic")[Excellence in Everything]
  ///     ]
  ///   },
  ///   page-position: "center"
  /// )
  "footer": (
    content,
    columns: auto,
    gutter: 1em,
    alignment: "center",
    style: none,
    fill: none,
    stroke: none,
    inset: 0.5em,
    height: auto,
    page-numbering: none,
    page-position: "center",
  ) => {
    let content-array = if type(content) == array { content } else { (content,) }

    // Handle page numbering
    let page-content = if page-numbering != none {
      if type(page-numbering) == function {
        page-numbering()
      } else if page-numbering == "1" {
        context str(here().page())
      } else if page-numbering == "1 of 1" {
        context str(here().page()) + " of " + str(counter(page).final().first())
      } else if page-numbering == "Page 1" {
        context "Page " + str(here().page())
      } else if page-numbering == "Page 1 of 1" {
        context "Page " + str(here().page()) + " of " + str(counter(page).final().first())
      } else {
        context str(here().page())
      }
    } else { none }

    // Insert page numbering into content array
    if page-content != none {
      if type(page-position) == int {
        // Insert at specific column index
        content-array.insert(page-position, page-content)
      } else if page-position == "left" {
        content-array.insert(0, page-content)
      } else if page-position == "right" {
        content-array.push(page-content)
      } else {
        // Default to center - add to middle or replace middle
        let mid = calc.floor(content-array.len() / 2)
        if calc.rem(content-array.len(), 2) == 1 {
          content-array.at(mid) = page-content
        } else {
          content-array.insert(mid, page-content)
        }
      }
    }

    let col-count = if columns == auto { content-array.len() } else { columns }
    let align-array = if type(alignment) == array { alignment } else { (alignment,) * col-count }

    let footer-content = if col-count == 1 {
      align(if align-array.at(0) == "left" { left } else if align-array.at(0) == "right" { right } else { center })[
        #if style != none { set text(..style) }
        #content-array.at(0)
      ]
    } else {
      grid(
        columns: (1fr,) * col-count,
        column-gutter: gutter,
        ..content-array
          .enumerate()
          .map(((i, item)) => {
            align(if align-array.at(i, default: "center") == "left" { left } else if align-array.at(
              i,
              default: "center",
            )
              == "right" { right } else { center })[
              #if style != none { set text(..style) }
              #item
            ]
          })
      )
    }

    block(
      width: 100%,
      height: height,
      fill: fill,
      stroke: stroke,
      inset: inset,
      footer-content,
    )
  },
)
