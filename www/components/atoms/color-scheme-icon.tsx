import { MoonIcon, SunIcon } from "@heroicons/react/outline";
import React, { MouseEvent } from "react";

interface Props {
  onClick: (event: MouseEvent) => void;
}

export function ColorSchemeIcon({ onClick }: Props): JSX.Element {
  return (
    <>
      <button
        onClick={onClick}
        className="hs-dark-mode-active:hidden block hs-dark-mode group flex items-center text-gray-600 hover:text-blue-600 font-medium dark:text-gray-400 dark:hover:text-gray-500"
      >
        <MoonIcon className="w-4 h-4" />
      </button>
      <button
        onClick={onClick}
        className="hs-dark-mode-active:block hidden hs-dark-mode group flex items-center text-gray-600 hover:text-blue-600 font-medium dark:text-gray-400 dark:hover:text-gray-500"
      >
        <SunIcon className="w-4 h-4" />
      </button>
    </>
  );
}
