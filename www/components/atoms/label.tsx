import { LabelHTMLAttributes, PropsWithChildren } from "react";
import clsx from "clsx";

interface Props extends LabelHTMLAttributes<HTMLLabelElement> {}

export function Label(
  { children, className, ...rest }: PropsWithChildren<Props>,
): JSX.Element {
  return (
    <label
      className={clsx(
        "block text-sm font-medium mb-2 dark:text-white",
        className,
      )}
      {...rest}
    >
      {children}
    </label>
  );
}
