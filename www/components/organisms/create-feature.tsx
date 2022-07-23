import { CodeIcon } from "@heroicons/react/outline";
import clsx from "clsx";
import { useState } from "react";
import { FeatureInput } from "../../../lib/flagger-client";
import { Button } from "../atoms";
import { CreateFeatureEditor } from "../moleculas/create-feature-editor";
import { CreateFeatureForm } from "./create-feature-form";

interface Props {
  onSubmit: (input: FeatureInput) => void | Promise<void>;
}

export function CreateFeature({ onSubmit }: Props): JSX.Element {
  const [showEditor, toggleEditor] = useState(false);
  const [input, setInput] = useState<Partial<FeatureInput>>({});

  return (
    <form
      onSubmit={async (event) => {
        event.preventDefault();

        await onSubmit(input as FeatureInput);
      }}
    >
      <div className="flex flex-cols items-center gap-x-2 py-3 px-4 text-sm font-medium bg-white border text-gray-800 -mt-px first:rounded-t-lg first:mt-0 last:rounded-b-lg dark:bg-gray-800 dark:border-gray-700 dark:text-white">
        <button
          type="button"
          className={clsx(
            "inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800",
            !showEditor &&
              "text-gray-500 hover:text-gray-700 dark:text-gray-500 dark:hover:text-gray-400",
            showEditor &&
              "text-gray-300 hover:text-gray-500 dark:text-gray-300 dark:hover:text-gray-200",
          )}
          onClick={() => toggleEditor(!showEditor)}
        >
          <CodeIcon className="h-4 w-4" />
        </button>
      </div>

      <div className="flex flex-cols items-center gap-x-2 py-3 px-4 text-sm font-medium bg-white border text-gray-800 -mt-px first:rounded-t-lg first:mt-0 last:rounded-b-lg dark:bg-gray-800 dark:border-gray-700 dark:text-white">
        {showEditor && (
          <CreateFeatureEditor
            value={input}
            onChange={setInput}
          />
        )}
        {!showEditor && (
          <CreateFeatureForm
            value={input}
            onChange={setInput}
          />
        )}
      </div>

      <Button>
        Create feature
      </Button>
    </form>
  );
}
