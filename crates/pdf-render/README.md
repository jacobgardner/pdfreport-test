# PDF-Render

`pdf-render` is the core library for authoring pdfs, handling text formatting,
block formatting, image transformations, and ultimately writing the final PDF.

## Known things that we can optimize:

- Preserving a cache of text block calculations (since yoga will recompute
several times to find the right fit)
- Do NOT embed fonts into PDF that are not actually referenced by the doc
- HashMap is slow (cryptographically secure) - FxHashMap
- Use more references where we can
- Use COW for things that are cloned everywhere, but not written and cannot be referenced