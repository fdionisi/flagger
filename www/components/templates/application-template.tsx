import {
  BellIcon,
  ChevronRightIcon,
  ColorSwatchIcon,
  LightningBoltIcon,
  MenuIcon,
  SearchIcon,
} from "@heroicons/react/outline";
import clsx from "clsx";
import Link from "next/link";
import { useRouter } from "next/router";
import { PropsWithChildren } from "react";
import { CommandPalette } from "../organisms";

interface Props {
  description: string;
  group: string;
  title: string;
  breadcrumbs?: { text: string; url: string }[];
}

export function ApplicationTemplate(
  { children, description, group, title, breadcrumbs }: PropsWithChildren<
    Props
  >,
): JSX.Element {
  const router = useRouter();
  return (
    <div className="bg-gray-50 min-h-full dark:bg-slate-900">
      <header
        className={"sticky top-0 inset-x-0 flex flex-wrap w-full bg-white border-b text-sm py-2.5 " +
          "sm:justify-start sm:flex-nowrap sm:py-4 " +
          "lg:pl-64 " +
          "dark:bg-gray-800 dark:border-gray-700"}
      >
        <nav className="flex basis-full items-center w-full mx-auto px-4 sm:px-6 md:px-8">
          <div className="mr-5 lg:mr-0 lg:hidden">
            <a
              className="flex-none text-xl font-semibold dark:text-white"
              href="#"
              aria-label="Flagger"
            >
              Flagger
            </a>
          </div>

          <div className="w-full flex items-center justify-end ml-auto sm:justify-between sm:gap-x-3 sm:order-3">
            <CommandPalette
              commands={[
                {
                  group: "Feature",
                  name: "Go to features",
                  action: (): void => {
                    router?.push("/feature");
                  },
                },
                {
                  group: "Feature",
                  name: "Create feature",
                  action: (): void => {
                    router?.push("/feature/new");
                  },
                },
              ]}
            >
              {({ setIsOpen }) => (
                <>
                  <div className="sm:hidden">
                    <button
                      type="button"
                      className="inline-flex flex-shrink-0 justify-center items-center gap-2 h-[2.375rem] w-[2.375rem] rounded-full font-medium bg-white text-gray-700 align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-xs dark:bg-gray-800 dark:hover:bg-slate-800 dark:text-gray-400 dark:hover:text-white dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
                      onClick={() => setIsOpen(true)}
                    >
                      <SearchIcon className="h-4 w-4 text-gray-400" />
                    </button>
                  </div>
                  <div className="hidden sm:block">
                    <button
                      type="button"
                      className="w-full flex items-center text-sm leading-6 text-slate-400 rounded-md ring-1 ring-slate-900/10 dark:ring-slate-700 shadow-sm py-1.5 pl-2 pr-3 hover:ring-slate-300 dark:highlight-white/5 dark:hover:bg-slate-700/40"
                      onClick={() => setIsOpen(true)}
                    >
                      <SearchIcon className="h-4 w-4 mr-2 text-gray-400" />
                      Quick search...
                      <span className="ml-auto pl-3 flex-none text-xs font-semibold">
                        ⌘K
                      </span>
                    </button>
                  </div>
                </>
              )}
            </CommandPalette>

            <div className="flex flex-row items-center justify-end gap-2">
              <button
                type="button"
                className="inline-flex flex-shrink-0 justify-center items-center gap-2 h-[2.375rem] w-[2.375rem] rounded-full font-medium bg-white text-gray-700 align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-xs dark:bg-gray-800 dark:hover:bg-slate-800 dark:text-gray-400 dark:hover:text-white dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
              >
                <BellIcon className="h-4 w-4 text-gray-400" />
              </button>
              <button
                type="button"
                className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center gap-2 h-[2.375rem] w-[2.375rem] rounded-full font-medium bg-white text-gray-700 align-middle hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white transition-all text-xs dark:bg-gray-800 dark:hover:bg-slate-800 dark:text-gray-400 dark:hover:text-white dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
                data-hs-offcanvas="#hs-offcanvas-right"
              >
                <LightningBoltIcon className="h-4 w-4 text-gray-400" />
              </button>
              <ColorSwatchIcon />
            </div>
          </div>
        </nav>
      </header>

      <div className="sticky top-[59px] inset-x-0 z-10 bg-white border-y px-4 sm:px-6 sm:top-[71px] md:px-8 lg:hidden dark:bg-gray-800 dark:border-gray-700">
        <div className="flex items-center py-4">
          <button
            type="button"
            className="text-gray-500 hover:text-gray-600"
            data-hs-sidebar="#docs-sidebar"
            aria-controls="docs-sidebar"
            aria-label="Toggle navigation"
          >
            <span className="sr-only">Toggle Navigation</span>
            <MenuIcon className="h-4 w-4 text-gray-400" />
          </button>

          {!!breadcrumbs?.length && (
            <ol
              className="ml-3 flex items-center whitespace-nowrap min-w-0"
              aria-label="Breadcrumb"
            >
              {breadcrumbs.map((crumb, index, array) => (
                <li
                  key={crumb.url}
                  className={clsx(
                    "flex items-center text-sm text-gray-800 dark:text-gray-400",
                    index === array.length - 1 && "font-semibold",
                  )}
                >
                  {index === array.length - 1
                    ? crumb.text
                    : (
                      <Link href={crumb.url}>
                        {crumb.text}
                      </Link>
                    )}

                  {index < array.length - 1 && (
                    <ChevronRightIcon className="flex-shrink-0 mx-1 overflow-visible h-4 w-4 text-gray-400 dark:text-gray-600" />
                  )}
                </li>
              ))}
            </ol>
          )}
        </div>
      </div>

      {/* Sidebar */}
      <div
        id="docs-sidebar"
        className="hs-sidebar hs-sidebar-open:translate-x-0 -translate-x-full transition-all duration-300 transform hidden fixed top-0 left-0 bottom-0 z-10 w-64 bg-white border-r border-gray-200 pt-7 pb-10 overflow-y-auto scrollbar-y lg:block lg:translate-x-0 lg:right-auto lg:bottom-0 dark:scrollbar-y dark:bg-gray-800 dark:border-gray-700"
      >
        <div className="px-6">
          <a
            className="flex-none text-xl font-semibold dark:text-white"
            href="#"
            aria-label="Brand"
          >
            Flagger
          </a>
        </div>

        <nav className="p-6 w-full flex flex-col flex-wrap">
          <ul className="space-y-1.5">
            <li>
              <a
                className="flex items-center gap-x-3 py-2 px-2.5 bg-gray-100 text-sm text-slate-700 rounded-md dark:bg-slate-700 dark:text-white"
                href="#"
              >
                <svg
                  className="w-3.5 h-3.5"
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  fill="currentColor"
                  viewBox="0 0 16 16"
                >
                  <path
                    fillRule="evenodd"
                    d="M2 13.5V7h1v6.5a.5.5 0 0 0 .5.5h9a.5.5 0 0 0 .5-.5V7h1v6.5a1.5 1.5 0 0 1-1.5 1.5h-9A1.5 1.5 0 0 1 2 13.5zm11-11V6l-2-2V2.5a.5.5 0 0 1 .5-.5h1a.5.5 0 0 1 .5.5z"
                  />
                  <path
                    fillRule="evenodd"
                    d="M7.293 1.5a1 1 0 0 1 1.414 0l6.647 6.646a.5.5 0 0 1-.708.708L8 2.207 1.354 8.854a.5.5 0 1 1-.708-.708L7.293 1.5z"
                  />
                </svg>
                Dashboard
              </a>
            </li>
          </ul>
        </nav>
      </div>

      {/* Content */}
      <div className="w-full pt-10 px-4 pb-10 sm:px-6 md:px-8 lg:pl-72">
        <header>
          <p className="mb-2 text-sm font-semibold text-blue-600">
            {group}
          </p>
          <h1 className="block text-2xl font-bold text-gray-800 sm:text-3xl dark:text-white">
            {title}
          </h1>
          <p className="mt-2 text-lg text-gray-800 dark:text-gray-400">
            {description}
          </p>
        </header>

        <div className="mt-5 flex flex-col">
          {children}
        </div>
      </div>
      <div
        id="hs-offcanvas-right"
        className="hs-offcanvas hs-offcanvas-open:translate-x-0 translate-x-full fixed top-0 right-0 transition-all duration-300 transform h-full max-w-xs w-full w-full z-50 bg-white border-l dark:bg-gray-800 dark:border-gray-700"
        tabIndex={-1}
      >
        <div className="flex justify-between items-center py-3 px-4 border-b dark:border-gray-700">
          <h3 className="font-bold text-gray-800 dark:text-white">
            Offcanvas title
          </h3>
          <button
            type="button"
            className="hs-dropdown-toggle inline-flex flex-shrink-0 justify-center items-center h-8 w-8 rounded-md text-gray-500 hover:text-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:text-gray-500 dark:hover:text-gray-400 dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
            data-hs-offcanvas="#hs-offcanvas-right"
          >
            <span className="sr-only">Close modal</span>
            <svg
              className="w-3.5 h-3.5"
              width="8"
              height="8"
              viewBox="0 0 8 8"
              fill="none"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                d="M0.258206 1.00652C0.351976 0.912791 0.479126 0.860131 0.611706 0.860131C0.744296 0.860131 0.871447 0.912791 0.965207 1.00652L3.61171 3.65302L6.25822 1.00652C6.30432 0.958771 6.35952 0.920671 6.42052 0.894471C6.48152 0.868271 6.54712 0.854471 6.61352 0.853901C6.67992 0.853321 6.74572 0.865971 6.80722 0.891111C6.86862 0.916251 6.92442 0.953381 6.97142 1.00032C7.01832 1.04727 7.05552 1.1031 7.08062 1.16454C7.10572 1.22599 7.11842 1.29183 7.11782 1.35822C7.11722 1.42461 7.10342 1.49022 7.07722 1.55122C7.05102 1.61222 7.01292 1.6674 6.96522 1.71352L4.31871 4.36002L6.96522 7.00648C7.05632 7.10078 7.10672 7.22708 7.10552 7.35818C7.10442 7.48928 7.05182 7.61468 6.95912 7.70738C6.86642 7.80018 6.74102 7.85268 6.60992 7.85388C6.47882 7.85498 6.35252 7.80458 6.25822 7.71348L3.61171 5.06702L0.965207 7.71348C0.870907 7.80458 0.744606 7.85498 0.613506 7.85388C0.482406 7.85268 0.357007 7.80018 0.264297 7.70738C0.171597 7.61468 0.119017 7.48928 0.117877 7.35818C0.116737 7.22708 0.167126 7.10078 0.258206 7.00648L2.90471 4.36002L0.258206 1.71352C0.164476 1.61976 0.111816 1.4926 0.111816 1.36002C0.111816 1.22744 0.164476 1.10028 0.258206 1.00652Z"
                fill="currentColor"
              />
            </svg>
          </button>
        </div>
        <div className="p-4">
          <p className="text-gray-800 dark:text-gray-400">
            Some text as placeholder. In real life you can have the elements you
            have chosen. Like, text, images, lists, etc.
          </p>
        </div>
      </div>
    </div>
  );
}
