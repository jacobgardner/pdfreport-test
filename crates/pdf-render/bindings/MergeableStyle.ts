import type { MergeableBorderStyle } from "./MergeableBorderStyle";
import type { MergeableEdgeStyle } from "./MergeableEdgeStyle";
import type { MergeableFlexStyle } from "./MergeableFlexStyle";
import type { MergeableFontStyles } from "./MergeableFontStyles";

export interface MergeableStyle { border?: MergeableBorderStyle, font?: MergeableFontStyles, color?: string, margin?: MergeableEdgeStyle, padding?: MergeableEdgeStyle, background_color?: string, flex?: MergeableFlexStyle, width?: string, height?: string, }