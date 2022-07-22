import { Combobox, Transition } from "@headlessui/react";
import { CheckIcon, SelectorIcon } from "@heroicons/react/solid";
import clsx from "clsx";
import React, { Fragment, useMemo, useState } from "react";

import { inputBaseClasses, InputProps } from "./input";

export interface Option {
  key: string | number;
  value: string | undefined;
  text: string | number;
}

interface Props extends Omit<InputProps, "onChange"> {
  onChange: (option?: Option) => void;
  options: Option[];
}

export function Autocomplete({
  className,
  options,
  onChange,
  value,
  placeholder = "",
  ...rest
}: Props) {
  const selected = useMemo(
    () => options.find((option) => option.value === value),
    [options, value],
  );

  const [query, setQuery] = useState("");

  const filteredOptions = query === ""
    ? options
    : options.filter((option) =>
      `${option.text}`
        .toLowerCase()
        .replace(/\s+/g, "")
        .includes(query.toLowerCase().replace(/\s+/g, ""))
    );

  return (
    <div className={clsx("w-72 z-20", className)} {...rest}>
      <Combobox value={selected} onChange={(o) => onChange?.(o)}>
        <div className="relative w-full">
          <div className="focus:outline-none relative w-full cursor-default overflow-hidden rounded-lg bg-white text-left shadow-md focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75 focus-visible:ring-offset-2 focus-visible:ring-offset-teal-300 sm:text-sm">
            <Combobox.Input
              className={inputBaseClasses}
              displayValue={(option: Option) =>
                `${option?.text || ""}` || placeholder}
              onChange={(event) => setQuery(event.target.value)}
              onFocus={(event) => event.target.select()}
            />
            <Combobox.Button className="absolute inset-y-0 right-0 flex items-center pr-2">
              <SelectorIcon
                className="h-5 w-5 text-gray-400"
                aria-hidden="true"
              />
            </Combobox.Button>
          </div>
          <Transition
            as={Fragment}
            leave="transition-[opacity,margin] ease-in duration-100"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
            afterLeave={() => setQuery("")}
          >
            <Combobox.Options className="w-72 z-10 mt-2 min-w-[15rem] bg-white shadow-md rounded-lg p-2 dark:bg-gray-800 dark:border dark:border-gray-700 dark:divide-gray-700">
              {filteredOptions.length === 0 && query !== ""
                ? (
                  <div className="relative cursor-default select-none py-2 px-4 text-gray-700">
                    Nothing found.
                  </div>
                )
                : (
                  filteredOptions.map((option) => (
                    <Combobox.Option
                      key={option.key}
                      className={({ active }) =>
                        clsx(
                          "flex items-center gap-x-3.5 py-2 px-3 rounded-md text-sm text-gray-800 hover:bg-gray-100 focus:ring-2 focus:ring-blue-500 dark:text-gray-400 dark:hover:bg-gray-700 dark:hover:text-gray-300",
                          active ? "bg-teal-600 text-white" : "text-gray-900",
                        )}
                      value={option}
                    >
                      {({ selected, active }) => (
                        <>
                          <span
                            className={`block truncate ${
                              selected ? "font-medium" : "font-normal"
                            }`}
                          >
                            {option.text}
                          </span>
                          {selected
                            ? (
                              <span
                                className={`absolute inset-y-0 left-0 flex items-center pl-3 ${
                                  active ? "text-white" : "text-teal-600"
                                }`}
                              >
                                <CheckIcon
                                  className="h-5 w-5"
                                  aria-hidden="true"
                                />
                              </span>
                            )
                            : null}
                        </>
                      )}
                    </Combobox.Option>
                  ))
                )}
            </Combobox.Options>
          </Transition>
        </div>
      </Combobox>
    </div>
  );
}
