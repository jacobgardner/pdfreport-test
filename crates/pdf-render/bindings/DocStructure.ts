import type { DomNode } from "./DomNode";
import type { FontFamilyInfo } from "./FontFamilyInfo";

export interface DocStructure { filename: string, documentTitle: string, pageSize: string, pageMargins: EdgeStyle, fonts: Array<FontFamilyInfo>, stylesheet: Record<string, Style>, root: DomNode, }