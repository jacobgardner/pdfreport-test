import type { Direction } from "./Direction";
import type { FlexAlign } from "./FlexAlign";
import type { FlexWrap } from "./FlexWrap";

export interface FlexStyle { direction: Direction | null, wrap: FlexWrap | null, align_items: FlexAlign | null, align_self: FlexAlign | null, grow: number | null, shrink: number | null, }