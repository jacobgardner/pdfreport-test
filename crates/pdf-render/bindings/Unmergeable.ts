import type { FontSlant } from "./FontSlant";
import type { FontWeight } from "./FontWeight";

export interface Unmergeable { family: string, size: string | number, style: FontSlant, weight: FontWeight, letterSpacing: string | number, }