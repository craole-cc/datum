#import "config/_.typ": *
#import "tools/_.typ": *
#import "themes/_.typ": *
#import "components/_.typ": *


#set page(
  header: (component.header)(
    ("Left Title", "Right Info"),
    page-numbering: "1 of 1",
    page-position: 1,
    alignment: ("left", "center", "right"),
  ),
  footer: (component.footer)(
    ("Left Content", "Right content"),
    page-numbering: () => {
      context [
        #box(width: 100%)[
          #grid(
            columns: (1fr, auto, 1fr),
            align(left)[Left Content], str(here().page()), align(right)[Right content],
          )
          #v(spacing.sm)
          #box(width: 100%)[
            #text(size: 8pt, fill: gray)[#tagline]
          ]
        ]
      ]
    },
    page-position: "center",
  ),
)
