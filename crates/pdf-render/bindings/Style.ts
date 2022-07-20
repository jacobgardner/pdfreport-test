import type { BorderStyle } from "./BorderStyle";
import type { EdgeStyle } from "./EdgeStyle";
import type { FlexStyle } from "./FlexStyle";
import type { FontStyles } from "./FontStyles";
import type { PageBreakRule } from "./PageBreakRule";
import type { TextTransformation } from "./TextTransformation";

export interface Style { border?: BorderStyle, font?: FontStyles, color?: string, margin?: EdgeStyle, padding?: EdgeStyle, backgroundColor?: string, flex?: FlexStyle, width?: string, height?: string, debug?: boolean, breakBefore?: PageBreakRule, breakAfter?: PageBreakRule, breakInside?: PageBreakRule, textTransform?: TextTransformation, lineHeight?: number | string, }