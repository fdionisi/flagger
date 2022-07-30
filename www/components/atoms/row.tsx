import clsx from "clsx";
import { HTMLAttributes } from "react";

export function Row(
  { className, children, ...rest }: HTMLAttributes<HTMLDivElement>,
): JSX.Element {
  return (
    <div
      {...rest}
      className={clsx(
        "flex items-center gap-x-2 py-3 px-4 text-sm font-medium bg-white border text-gray-800 -mt-px first:rounded-t-lg first:mt-0 last:rounded-b-lg dark:bg-gray-800 dark:border-gray-700 dark:text-white",
        className,
      )}
    >
      {children}
    </div>
  );
}
