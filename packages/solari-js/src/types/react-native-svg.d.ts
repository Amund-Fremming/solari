declare module "react-native-svg" {
  import * as React from "react";

  export type NumberLike = number | string;

  export interface SvgUriProps {
    uri?: string;
    width?: NumberLike;
    height?: NumberLike;
    style?: unknown;
  }

  export const SvgUri: React.ComponentType<SvgUriProps>;
}
