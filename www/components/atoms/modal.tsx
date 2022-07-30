import { Dialog, Transition } from "@headlessui/react";
import { PropsWithChildren } from "react";

interface Props {
  className?: string;
  isOpen: boolean;
  onClose: () => void;
}

export function Modal(
  { children, className, isOpen, onClose }: PropsWithChildren<Props>,
): JSX.Element {
  return (
    <Transition
      show={isOpen}
      enter="transition duration-100 ease-out"
      enterFrom="transform scale-95 opacity-0"
      enterTo="transform scale-100 opacity-100"
      leave="transition duration-75 ease-out"
      leaveFrom="transform scale-100 opacity-100"
      leaveTo="transform scale-95 opacity-0"
    >
      <Dialog
        onClose={onClose}
        className="fixed inset-0 p-4 pt-[15hv] overflow-y-auto z-10 "
      >
        <Dialog.Overlay className="fixed inset-0" />
        <div
          className={"bg-white rounded-md dark:border-gray-700 dark:text-white border dark:bg-gray-800 max-w-2xl mx-auto " +
            (className || "")}
        >
          {children}
        </div>
      </Dialog>
    </Transition>
  );
}
