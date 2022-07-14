import type {DomNode} from './DomNode';
import type {FontFamilyInfo} from './FontFamilyInfo';
import type {RequiredEdgeStyle} from './RequiredEdgeStyle';
import type {Style} from './Style';

export interface DocStructure {
  filename: string;
  documentTitle: string;
  pageSize: string;
  pageMargins: RequiredEdgeStyle;
  fonts: Array<FontFamilyInfo>;
  stylesheet: Record<string, Style>;
  root: DomNode;
}
