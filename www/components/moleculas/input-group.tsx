import { Input, InputProps } from "../atoms";

interface Props extends InputProps {
  label?: string;
}

export function InputGroup(
  { label, id, placeholder, ...rest }: Props,
): JSX.Element {
  return (
    <div>
      <label
        htmlFor={id}
        className="block text-sm font-medium mb-2 dark:text-white"
      >
        {label}
      </label>
      <div
        className="relative"
        placeholder={placeholder}
      >
        <Input id={id} placeholder={placeholder} {...rest} />
      </div>
    </div>
  );
}
