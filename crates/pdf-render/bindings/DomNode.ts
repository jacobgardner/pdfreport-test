import type { ImageNode } from "./ImageNode";
import type { StyledNode } from "./StyledNode";
import type { TextNode } from "./TextNode";

export type DomNode = { type: "Styled" } & StyledNode | { type: "Text" } & TextNode | { type: "Image" } & ImageNode;