import { Combobox } from "@headlessui/react";
import clsx from "clsx";
import Fuse from "fuse.js";
import React, {
  KeyboardEvent,
  useCallback,
  useEffect,
  useMemo,
  useState,
} from "react";
import { Modal } from "../atoms";

interface Command {
  group: string;
  name: string;
  shortcut?: JSX.Element;
  action: () => void;
}

interface ChildrenProps {
  setIsOpen: (isOpen: boolean) => void;
  isOpen: boolean;
}

interface Props {
  commands: Command[];
  children: (props: ChildrenProps) => JSX.Element;
  isOpen?: boolean;
}

interface PropsWithGroup extends Omit<Props, "children"> {
  group: string;
}

function CommandGroup({ commands, group }: PropsWithGroup): JSX.Element {
  const groupCommends = useMemo(
    () => commands.filter((command) => command.group === group),
    [commands, group],
  );
  return (
    <>
      {groupCommends.length >= 1 && (
        <div className="flex items-center h-6 flex-shrink-0 bg-accent/50">
          <span className="text-xs dark:text-slate-100 px-3.5 font-bold">
            {group}
          </span>
        </div>
      )}

      {groupCommends
        .map((command, idx) => (
          <Combobox.Option key={idx} value={command} className="p-4">
            {({ active }) => (
              <div
                className={clsx(
                  "w-full h-[46px] dark:text-white flex items-center hover:bg-slate-300/40 dark:hover:bg-slate-700/40 cursor-default transition-colors duration-100 ease-in rounded-md cursor-pointer",
                  active ? "bg-slate-300/40 dark:bg-slate-700/40" : "",
                )}
              >
                <div className="px-3.5 flex items-center w-full">
                  <div className="mr-3 flex items-center justify-center w-4">
                    {
                      /*mapCommandGroupToIcon(
              command.group.toLowerCase()
              )*/
                    }
                  </div>
                  <span className="text-sm text-left flex flex-auto">
                    {command.name}
                  </span>
                  {command.shortcut && (
                    <code className="mono text-xs ">
                      {command.shortcut}
                    </code>
                  )}
                </div>
              </div>
            )}
          </Combobox.Option>
        ))}
    </>
  );
}

export function CommandPalette(
  { commands, isOpen: defaultIsOpen = false, children }: Props,
): JSX.Element {
  const [isOpen, setIsOpen] = useState(defaultIsOpen);

  const [query, setQuery] = useState("");
  const fuse = useMemo(
    () =>
      new Fuse(commands, {
        threshold: 0.2,
        useExtendedSearch: true,
        isCaseSensitive: false,
        includeScore: true,
        keys: ["name", { weight: 2, name: "group" }],
      }),
    [commands],
  );

  const filteredCommands = useMemo(
    () =>
      query === ""
        ? commands
        : fuse.search(query).map((res) => ({ ...res.item })),
    [commands, fuse, query],
  );

  useEffect(() => {
    const onKeydown = (e) => {
      if (e.key === "k" && (e.metaKey || e.ctrlKey)) {
        e.preventDefault();
        setIsOpen(true);
      }
    };
    window.addEventListener("keydown", onKeydown);
    return () => {
      window.removeEventListener("keydown", onKeydown);
    };
  }, []);

  const close = useCallback(() => {
    setQuery("");
    setIsOpen(false);
  }, [setQuery, setIsOpen]);

  return (
    <div>
      {children({
        setIsOpen,
        isOpen,
      })}
      <Modal onClose={close} isOpen={isOpen}>
        <Combobox
          value={undefined}
          onChange={(command: Command) => {
            close();
            command.action();
          }}
          as="div"
          className="dark:bg-accent-dark max-w-2xl mx-auto rounded-lg shadow-2xl relative flex flex-col"
        >
          <div className="flex items-center text-lg font-medium border-b border-slate-500 ">
            <Combobox.Input
              onKeyDown={(event: KeyboardEvent) => {
                if (event.key === "Escape") {
                  if (query) {
                    setQuery("");
                  } else {
                    close();
                  }
                }
              }}
              className="p-5 dark:placeholder-gray-200 w-full bg-transparent border-0 outline-none"
              placeholder="Type a command or search..."
              value={query}
              onChange={(event) => {
                setQuery(event.target.value);
              }}
            />
          </div>

          <Combobox.Options
            className="max-h-72 overflow-y-auto flex flex-col"
            static
          >
            <CommandGroup commands={filteredCommands} group="Feature" />
          </Combobox.Options>
        </Combobox>
      </Modal>
    </div>
  );
}
