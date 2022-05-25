import type { BorderStyle } from "./BorderStyle";
import type { Color } from "..\\Color";
import type { EdgeStyle } from "./EdgeStyle";
import type { FlexStyle } from "./FlexStyle";
import type { FontStyles } from "./FontStyles";

export interface Style {
  border?: BorderStyle;
  font?: FontStyles;
  color?: Color;
  margin?: EdgeStyle;
  padding?: EdgeStyle;
  background_color?: Color;
  flex?: FlexStyle;
  width?: string;
  height?: string;
}
