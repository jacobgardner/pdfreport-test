import type { BorderRadiusStyle } from "./BorderRadiusStyle";
import type { Color } from "..\\Color";

export interface BorderStyle {
  width?: number;
  color?: Color;
  radius?: BorderRadiusStyle;
}
