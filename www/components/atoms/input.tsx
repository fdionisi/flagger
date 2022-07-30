import clsx from "clsx";
import { InputHTMLAttributes } from "react";

export interface InputProps extends InputHTMLAttributes<HTMLInputElement> {
  value?: string;
}

export const inputBaseClasses =
  "py-3 px-4 border block w-full border-gray-200 rounded-md text-sm " +
  "focus:border-blue-500 focus:ring-blue-500 " +
  "dark:bg-gray-800 dark:border-gray-700 dark:text-gray-400";

export function Input({ className, ...rest }: InputProps): JSX.Element {
  return (
    <input
      className={clsx(inputBaseClasses, className)}
      {...rest}
    />
  );
}
