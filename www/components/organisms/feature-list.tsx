import {
  PencilIcon,
  PlusIcon,
  StatusOfflineIcon,
  StatusOnlineIcon,
} from "@heroicons/react/outline";
import Fuse from "fuse.js";
import Link from "next/link";
import { useMemo, useState } from "react";
import { Feature } from "../../../lib/flagger-client";
import { Indicator, Input } from "../atoms";

interface Props {
  features: Feature[];
  toggleFeature: (feature: string, enabled: boolean) => void | Promise<void>;
}

export function FeatureList(
  { features, toggleFeature }: Props,
): JSX.Element {
  const [query, setQuery] = useState("");

  const filteredEnabled = useMemo(() => {
    if (query.includes("enabled:true")) {
      return features.filter((feature) => feature.enabled);
    } else if (query.includes("enabled:false")) {
      return features.filter((feature) => !feature.enabled);
    } else {
      return features;
    }
  }, [features, query]);

  const fuse = useMemo(
    () =>
      new Fuse(filteredEnabled, {
        threshold: 0.2,
        useExtendedSearch: true,
        isCaseSensitive: false,
        includeScore: true,
        keys: ["name", { weight: 2, name: "description" }],
      }),
    [filteredEnabled],
  );

  const filteredFeatures = useMemo(
    () => {
      if (query === "") {
        return filteredEnabled;
      }

      const fuseResult = fuse.search(
        query.replace("enabled:true", "").replace("enabled:false", "").trim(),
      ).map((res) => ({
        ...res.item,
      }));
      if (fuseResult.length === 0) {
        return filteredEnabled;
      }

      return fuseResult;
    },
    [filteredEnabled, fuse, query],
  );

  return (
    <ul className="flex flex-col">
      <li className="inline-flex items-center justify-between gap-x-2 py-3 px-4 text-sm font-medium bg-white border text-gray-800 -mt-px first:rounded-t-lg first:mt-0 last:rounded-b-lg dark:bg-gray-800 dark:border-gray-700 dark:text-white">
        <Input
          type="text"
          value={query}
          placeholder="Filter..."
          onChange={(event) => setQuery(event.target.value)}
        />
        <Link
          href="/feature/new"
          className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:text-gray-500 dark:hover:text-gray-400 dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
        >
          <button
            type="button"
            className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:text-gray-500 dark:hover:text-gray-400 dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
          >
            <PlusIcon width={20} height={20} />
          </button>
        </Link>
      </li>

      {filteredFeatures.map((feature) => (
        <li
          key={feature._id}
          className="inline-flex items-center gap-x-2 py-3 px-4 text-sm font-medium bg-white border text-gray-800 -mt-px first:rounded-t-lg first:mt-0 last:rounded-b-lg dark:bg-gray-800 dark:border-gray-700 dark:text-white"
        >
          <div className="flex flex-col">
            <Indicator enabled={feature.enabled}>
              <p className="text-lg mr-4">{feature.name}</p>
            </Indicator>

            <div className="pl-4">
              <small>{feature.kind}</small>

              <p>{feature.description}</p>
            </div>
          </div>

          <div className="flex flex-1 items-center justify-end">
            <Link href={`/feature/${feature._id}`}>
              <button
                type="button"
                className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:text-gray-500 dark:hover:text-gray-400 dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
              >
                <PencilIcon width={20} height={20} />
              </button>
            </Link>
            <button
              type="button"
              className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:text-gray-500 dark:hover:text-gray-400 dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
              onClick={() => toggleFeature(feature._id, !feature.enabled)}
            >
              {feature.enabled
                ? <StatusOfflineIcon width={20} height={20} />
                : <StatusOnlineIcon width={20} height={20} />}
            </button>
          </div>
        </li>
      ))}
    </ul>
  );
}
