import type { PageBreakRule } from "./PageBreakRule";
import type { Unmergeable } from "./Unmergeable";

export interface Unmergeable { border: Unmergeable, font: Unmergeable, color: string, margin: Unmergeable, padding: Unmergeable, background_color: string, flex: Unmergeable, width: string, height: string, debug: boolean, break_before: PageBreakRule, break_after: PageBreakRule, break_inside: PageBreakRule, }