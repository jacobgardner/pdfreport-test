
// struct BorderStyle {
  
// }

// struct MergeableBorderStyle {
//   unique: usize
// }


// pub trait Mergeable {
//   type MergeType;
// }

// pub trait Unmergeable {
//   type UnmergeableType;
// }

// impl Mergeable for BorderStyle {
//   type MergeType = MergeableBorderStyle;
// }

// impl Unmergeable for MergeableBorderStyle {
//   type UnmergeableType = BorderStyle;
// }


// struct Style {
//     pub border: <BorderStyle as Mergeable>::MergeType,
// }


// pub fn output() {
//   let s = Style {
//     border: MergeableBorderStyle { unique: 83 }
//   };
// }