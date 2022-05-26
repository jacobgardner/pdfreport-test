import type { BorderStyle } from "./BorderStyle";
import type { EdgeStyle } from "./EdgeStyle";
import type { FlexStyle } from "./FlexStyle";
import type { FontStyles } from "./FontStyles";

export interface Style { border: BorderStyle, font: FontStyles, color: string, margin: EdgeStyle, padding: EdgeStyle, background_color: string, flex: FlexStyle, width: string, height: string, }