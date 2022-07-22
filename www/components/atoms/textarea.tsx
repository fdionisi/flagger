import clsx from "clsx";
import { TextareaHTMLAttributes } from "react";
import { inputBaseClasses } from "./input";

interface Props extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  value?: string;
}

export function Textarea({ className, ...rest }: Props): JSX.Element {
  return (
    <textarea
      className={clsx(inputBaseClasses, className)}
      {...rest}
    />
  );
}
