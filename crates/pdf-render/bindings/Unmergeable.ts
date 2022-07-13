import type { PageBreakRule } from "./PageBreakRule";
import type { TextTransformation } from "./TextTransformation";
import type { Unmergeable } from "./Unmergeable";

export interface Unmergeable { border: Unmergeable, font: Unmergeable, color: string, margin: Unmergeable, padding: Unmergeable, backgroundColor: string | null, flex: Unmergeable, width: string, height: string, debug: boolean, breakBefore: PageBreakRule, breakAfter: PageBreakRule, breakInside: PageBreakRule, textTransform: TextTransformation, lineHeight: string | number | null, }