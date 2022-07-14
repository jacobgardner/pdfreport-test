import type { MergeableBorderStyle } from "./MergeableBorderStyle";

export interface MergeableStyle { border: MergeableBorderStyle | null, width: string | null, height: string | null, debug: boolean | null, }