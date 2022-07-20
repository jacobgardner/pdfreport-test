import type { Direction } from "./Direction";
import type { FlexAlign } from "./FlexAlign";
import type { FlexWrap } from "./FlexWrap";

export interface FlexStyle { direction?: Direction, wrap?: FlexWrap, align_items?: FlexAlign, align_self?: FlexAlign, grow?: number, shrink?: number, }