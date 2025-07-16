// ======================================
// USAGE GUIDE
// ======================================
//
// Usage Example:
// #import "template.typ": *
// #show: template.with(
//   title: "My Awesome Document",
//   author: "John Doe",
//   config-override: (
//     layout: (show-title-page: true,),
//     theme: "dark", // or "light"
//   ),
// )

#import "config/project.typ": project

// ======================================
// CENTRALIZED CONFIGURATION SYSTEM
// ======================================
      text: rgb("#f9fafb"),
      text-light: rgb("#d1d5db"),
      text-muted: rgb("#9ca3af"),
      bg: rgb("#111827"),
      bg-alt: rgb("#1f2937"),
      bg-code: rgb("#374151"),
      border: rgb("#374151"),
      border-light: rgb("#4b5563"),
      success: rgb("#10b981"),
      warning: rgb("#f59e0b"),
      error: rgb("#ef4444"),
      info: rgb("#3b82f6"),
    ),
  ),
  content: (
    toc-title: "Table of Contents",
    code-label: "CODE",
    date-format: "[month repr:long] [day], [year]",
    author-label: "Author",
    date-label: "Date",
  ),
  icons: (
    tech: (
      python: "🐍",
      rust: "🦀",
    ),
    document: (
      roadmap: "🗺️",
      toolchain: "🔧",
    ),
  ),
)

#import "config/utils.typ": get-value, get-spacing, get-color, get-font, get-font-size, get-font-weight, get-content, deep-merge, validate-config

// ======================================
// ENHANCED COMPONENTS
// ======================================

#import "config/title_page.typ": create-title-page

#import "config/toc.typ": create-toc

#import "config/code_block.typ": create-code-block

#import "config/callout.typ": create-callout

#import "config/table.typ": create-table_OLD
#let create-table(headers: (), data: (), theme: "dark") = {
  // Theme-aware colors
  let header-fill = if theme == "light" {
    rgb("#f7fafc") // Light gray for light theme
  } else {
    rgb("#2d3748") // Dark gray for dark theme
  }

  let border-color = if theme == "light" {
    rgb("#4a5568") // Dark border
  } else {
    rgb("#4a5568") // Dark border
    // rgb("#e2e8f0") // Light border
  }

  let text-color = if theme == "light" {
    rgb("#1a202c") // Dark text on light background
  } else {
    rgb("#4a5568") // Dark border
  }

  table(
    columns: headers.len(),
    fill: (x, y) => if y == 0 { header-fill } else { none },
    stroke: border-color,
    ..headers.map(h => text(fill: text-color, h)),
    ..data.flatten().map(d => text(fill: text-color, d))
  )
}

// ======================================
// ENHANCED STYLING SYSTEM
// ======================================
#import "config/heading.typ": apply-heading-styles

#import "config/footer.typ": create-footer

// ======================================
// MAIN TEMPLATE FUNCTION
// ======================================
#let template = (
  title: none,
  subtitle: none,
  author: none,
  theme: none,
  show-title-page: none,
  show-toc: none,
  show-footer: none,
  show-page-numbers: none,
  config-override: (:),
  body,
) => {
  // Merge configurations
  let config = deep-merge(project, config-override)
  if theme != none { config.insert("theme", theme) }

  validate-config(config)

  // Update metadata
  if title != none { config.metadata.insert("title", title) }
  if subtitle != none { config.metadata.insert("subtitle", subtitle) }
  if author != none { config.metadata.insert("author", author) }

  // Update layout settings with parameter overrides
  let show-title-page-final = if show-title-page != none { show-title-page } else { config.layout.show-title-page }
  let show-toc-final = if show-toc != none { show-toc } else { config.layout.show-toc }
  let show-footer-final = if show-footer != none { show-footer } else { config.layout.show-footer }
  let show-page-numbers-final = if show-page-numbers != none { show-page-numbers } else {
    config.layout.show-page-numbers
  }

  let colors = get-value(config.themes, theme, default: config.themes.dark)

  // Document metadata
  set document(
    title: config.metadata.title,
    author: config.metadata.author,
    date: config.metadata.date,
    keywords: config.metadata.keywords,
  )

  // Enhanced page setup
  set page(
    paper: config.layout.paper,
    margin: config.layout.margin,
    numbering: none,
    footer: none,
    fill: colors.bg,
  )

  // Enhanced text settings
  set text(
    font: get-font("sans"),
    size: get-font-size("base"),
    fill: colors.text,
    hyphenate: config.layout.text-hyphenate,
    lang: config.metadata.language,
  )

  // Enhanced paragraph settings
  set par(
    leading: config.layout.line-height * 1em,
    spacing: config.layout.paragraph-spacing,
    justify: config.layout.paragraph-justify,
    first-line-indent: config.layout.paragraph-indent,
  )

  // Enhanced list styling
  set list(
    indent: config.layout.list-indent,
    spacing: config.layout.list-spacing,
    marker: config.layout.list-markers,
  )

  set enum(
    indent: config.layout.enum-indent,
    spacing: config.layout.enum-spacing,
    numbering: config.layout.enum-numbering,
  )

  // Link styling
  show link: it => {
    text(fill: colors.primary, weight: get-font-weight(config.layout.link.weight))[#it]
  }

  // Enhanced emphasis
  show emph: it => {
    text(
      style: config.layout.emphasis.style,
      fill: colors.text-light,
    )[#it]
  }

  show strong: it => {
    text(
      weight: get-font-weight(config.layout.strong.weight),
      fill: colors.text,
    )[#it]
  }

  // Heading configuration
  set heading(numbering: config.layout.heading-numbering)
  apply-heading-styles(config)

  // Front matter
  if show-title-page-final {
    create-title-page(
      title: config.metadata.title,
      subtitle: config.metadata.subtitle,
      author: config.metadata.author,
      date: config.metadata.date,
      config: config,
    )
  }

  if show-toc-final {
    create-toc(config: config)
  }

  // Main content setup
  set page(
    paper: config.layout.paper,
    margin: config.layout.margin,
    numbering: config.layout.page-numbering.format,
    footer: if show-footer-final {
      create-footer(show-page-numbers-final)
    } else { none },
    fill: colors.bg,
  )

  counter(page).update(config.layout.page-numbering.start)

  // Render content
  body
}
