import type { BorderStyle } from "./BorderStyle";
import type { EdgeStyle } from "./EdgeStyle";
import type { FlexStyle } from "./FlexStyle";
import type { FontStyles } from "./FontStyles";
import type { PageBreakRule } from "./PageBreakRule";
import type { TextTransformation } from "./TextTransformation";

export interface Style { border: BorderStyle | null, font: FontStyles | null, color: string | null, margin: EdgeStyle | null, padding: EdgeStyle | null, backgroundColor: string | null | null, flex: FlexStyle | null, width: string | null, height: string | null, debug: boolean | null, breakBefore: PageBreakRule | null, breakAfter: PageBreakRule | null, breakInside: PageBreakRule | null, textTransform: TextTransformation | null, lineHeight: string | number | null | null, }