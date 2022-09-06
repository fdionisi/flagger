export enum FeatureKind {
  KillSwitch = "KillSwitch",
}

export interface Feature {
  kind: FeatureKind;
  name: string;
  description?: string;
  enabled: boolean;
}

export interface FeatureInput {
  kind: FeatureKind;
  name: string;
  description?: string;
}
