import { PropsWithChildren } from "react";

interface Props {
  enabled: boolean;
}

export function Indicator(
  { enabled, children }: PropsWithChildren<Props>,
): JSX.Element {
  const statusClass = enabled ? "bg-green-500" : "bg-red-500";
  return (
    <div className="inline-flex items-center">
      <span
        className={`w-2 h-2 inline-block rounded-full mr-2 ${statusClass}`}
      />
      <span className="text-gray-600 dark:text-gray-400">{children}</span>
    </div>
  );
}
