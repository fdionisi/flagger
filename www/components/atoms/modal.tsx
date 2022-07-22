import { Dialog, Transition } from "@headlessui/react";
import { PropsWithChildren } from "react";

interface Props {
  isOpen: boolean;
  onClose: () => void;
}

export function Modal(
  { children, isOpen, onClose }: PropsWithChildren<Props>,
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
        className="fixed inset-0 p-4 pt-[15hv] overflow-y-auto z-10"
      >
        <Dialog.Overlay className="fixed inset-0" />
        <div className="bg-white rounded-md dark:bg-slate-900 max-w-2xl mx-auto ">
          {children}
        </div>
      </Dialog>
    </Transition>
  );
}
