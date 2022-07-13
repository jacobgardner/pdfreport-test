import type { BorderRadiusStyle } from "./BorderRadiusStyle";
import type { EdgeStyle } from "./EdgeStyle";

export interface BorderStyle { width: EdgeStyle | null, color: string | null, radius: BorderRadiusStyle | null, }