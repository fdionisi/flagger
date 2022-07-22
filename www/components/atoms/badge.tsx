import { FeatureKind } from "../../../lib/flagger-client";

interface Props {
  kind: FeatureKind;
}

export function Badge({ kind }: Props): JSX.Element {
  return (
    <span className="inline-flex items-center gap-1.5 py-1.5 px-3 rounded-md text-xs font-medium bg-blue-100 text-blue-800">
      {kind}
    </span>
  );
}
