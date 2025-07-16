#let component = (
  /// Creates a header element for the document.
  /// - content: The content to display in the header. Can be a single element or array for columns.
  /// - columns: Number of columns (auto-detected from content array length if not specified).
  /// - gutter: Space between columns (default: 1em).
  /// - alignment: Text alignment ("left", "center", "right") or array of alignments per column.
  /// - style: Additional styling options.
  /// - fill: Background fill color.
  /// - stroke: Border stroke options.
  /// - inset: Internal padding (default: 0.5em).
  /// - height: Fixed height for the header.
  /// - page-numbering: Page numbering options (none, "1", "1 of 1", "Page 1", "Page 1 of 1", custom function).
  /// - page-position: Where to place page numbers ("left", "center", "right", or column index).
  /// Returns a header element.
  ///
  /// USAGE EXAMPLES:
  ///
  /// // Simple single column header
  /// #component.header("Document Title")
  ///
  /// // Multi-column header with array of content
  /// #component.header(
  ///   ("Company Logo", "Document Title", "Date"),
  ///   alignment: ("left", "center", "right")
  /// )
  ///
  /// // Custom styling with underline
  /// #component.header(
  ///   ("Project Name", format.date(format: "long")),
  ///   columns: 2,
  ///   gutter: 2em,
  ///   style: (size: 12pt, weight: "bold"),
  ///   fill: rgb("#f8f8f8"),
  ///   stroke: (bottom: 2pt + blue),
  ///   inset: 1em
  /// )
  ///
  /// // Page numbering in header
  /// #component.header(
  ///   ("Document Title", "Chapter 1"),
  ///   page-numbering: "Page 1 of 1",
  ///   page-position: "right"
  /// )
  ///
  /// // Title with subtitle stacked vertically
  /// #component.header(
  ///   ("Company Name", "Department"),
  ///   page-numbering: () => {
  ///     align(center)[
  ///       #text(size: 14pt, weight: "bold")[Main Title] \
  ///       #text(size: 10pt, fill: gray)[Subtitle or tagline]
  ///     ]
  ///   },
  ///   page-position: "center"
  /// )
  ///
  /// // Three columns with logo, title, and date
  /// #component.header(
  ///   ("🏢 Company", "Annual Report 2025", format.date(format: "long")),
  ///   alignment: ("left", "center", "right"),
  ///   gutter: 1.5em,
  ///   style: (size: 11pt, weight: "semibold"),
  ///   stroke: (bottom: 1pt + gray.lighten(30%))
  /// )
  ///
  /// // Fixed height header with custom content
  /// #component.header(
  ///   "Department of Engineering",
  ///   height: 4em,
  ///   page-numbering: () => {
  ///     "Section " + str(here().page()) + " • " + format.date(format: "short")
  ///   },
  ///   page-position: "right",
  ///   fill: rgb("#e8f4fd"),
  ///   stroke: (bottom: 3pt + blue.lighten(20%))
  /// )
  ///
  /// // Insert page numbering at specific column index
  /// #component.header(
  ///   ("Left Title", "Right Info"),
  ///   page-numbering: "1 of 1",
  ///   page-position: 1,  // Insert at index 1 (between Left and Right)
  ///   alignment: ("left", "center", "right")
  /// )
  ///
  /// // Just centered title with custom spacing
  /// #component.header(
  ///   (),
  ///   page-numbering: () => {
  ///     align(center)[
  ///       #text(size: 16pt, weight: "bold")[Document Title] \
  ///       #v(0.3em) \
  ///       #text(size: 10pt, style: "italic")[Professional Report]
  ///     ]
  ///   },
  ///   page-position: "center",
  ///   inset: (top: 1em, bottom: 0.5em)
  /// )
  "header": (
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

    let header-content = if col-count == 1 {
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
      header-content,
    )
  },
)
