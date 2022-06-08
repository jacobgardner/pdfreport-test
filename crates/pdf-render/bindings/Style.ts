import type { BorderStyle } from "./BorderStyle";
import type { EdgeStyle } from "./EdgeStyle";
import type { FlexStyle } from "./FlexStyle";
import type { FontStyles } from "./FontStyles";
import type { PageBreakRule } from "./PageBreakRule";

export interface Style { border?: BorderStyle, font?: FontStyles, color?: string, margin?: EdgeStyle, padding?: EdgeStyle, background_color?: string, flex?: FlexStyle, width?: string, height?: string, debug?: boolean, break_before?: PageBreakRule, break_after?: PageBreakRule, break_inside?: PageBreakRule, }