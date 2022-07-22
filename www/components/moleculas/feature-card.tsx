import type { Feature } from "../../../lib/flagger-client";

import { Badge, Indicator } from "../atoms";

interface Props {
  feature: Feature;
}

export function FeatureCard({ feature }: Props): JSX.Element {
  return (
    <div
      className={"flex flex-col bg-white border shadow-sm rounded-xl p-4 " +
        "md:p-5 " +
        "dark:bg-gray-800 dark:border-gray-700 dark:shadow-slate-700/[.7] dark:text-gray-400"}
    >
      <Indicator enabled={feature.enabled}>
        {feature.name} <Badge kind={feature.kind} />
      </Indicator>
      <p>{feature.description}</p>
    </div>
  );
}
