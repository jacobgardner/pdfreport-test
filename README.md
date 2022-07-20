# PDF Report Generator

This project is organized into a few different pieces.

The root of the project is just the server and cli binary that utilizing the
underlying libraries. These, on their own, are pretty small and only exist (as
of this writing) as two files (`cli.rs` and `server.rs`)

The libraries that drive pdf generation can be found in the `crates` directory
with the primary one being `pdf-render`.

- prototype-implementation - This does not need to be reviewed. This will be
  removed once SVG images are integrated into the PDF generation library.
- optional-merge-derive & merges - These two crates are complementary. They both
  work to allow us to define a fully specified structure and apply a macros to
  it to allow us to partially specify and merge partially specified structures.

  This is primarily (read: only) used to support stylesheets in the pdf generation.
  
  i.e. You can specify a style structure:

  ```rust
  #[mergeable]
  struct SomeStyles {
    color: String,
    backgroundColor: String,
    size: Pt,
  }
  ```
  
  and it will generate a Mergeable and Unmergable version of that struct at:
  `SomeStyles::Mergeable` and `SomeStyles::Unmergeable` 

  Where we can do something like:

  ```rust
  let a = SomeStyles::Mergeable {
    color: Some("red"),
    backgroundColor: Some("pink"),
    ..Default::default()
  }
 
  let b = SomeStyles::Mergeable {
    backgroundColor: Some("blue"),
    ..Default::default()
  } 
  
  
  let merged: SomeStyles::Unmergeable = a.merges(b);
  ``` 

  which results in a fully-defined merged struct.
- pdf-render - This is where the bulk of the work happens. Go to the `README.md`
  in pdf-render for more details.
  

