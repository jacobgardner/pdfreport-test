import type { FontSlant } from "./FontSlant";
import type { FontWeight } from "./FontWeight";

export interface FontStyles { family: string | null, size: string | number | null, style: FontSlant | null, weight: FontWeight | null, letterSpacing: string | number | null, }