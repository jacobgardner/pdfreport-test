# PDF-Render

`pdf-render` is the core library for authoring pdfs, handling text formatting,
block formatting, image transformations, and ultimately writing the final PDF.

The order of operations roughly is this:

- Parse and validate JSON into a DocStructure
- Load Fonts for future pipeline stuff
- Using flexbox, we build the block layout (using yoga).
  - Within this pipeline yoga will ask text nodes to lay out their text content
    (alignment, line-breaks, etc.)
- Once block layout and text-layout has been computed, compute paginated
  layout.
- Once we have the paginated layout, send those paginated nodes to the PDF writer

- At several times during this, we compute and merge styles from stylesheets
  and apply them to the appropriate contexts (layout, text layout, writing to
  the PDF)

## Known things that we can optimize:

- Preserving a cache of text block calculations (since yoga will recompute
  several times to find the right fit)
- Do NOT embed fonts into PDF that are not actually referenced by the doc
- HashMap is slow (cryptographically secure) - FxHashMap
- Use more references where we can
- Use COW for things that are cloned everywhere, but not written and cannot be referenced
