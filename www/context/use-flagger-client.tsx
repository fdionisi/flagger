import React, { createContext, PropsWithChildren, useContext } from "react";

import { FlaggerClient, FlaggerClientContext } from "../../lib/flagger-client";

interface State {
  flaggerClient: FlaggerClient;
  flaggerClientContext: FlaggerClientContext;
}

interface Props {
  baseUrl: string;
}

const Context = createContext<State>({} as any);

export function FlaggerClientProvider({ children }: PropsWithChildren<Props>) {
  return (
    <Context.Provider
      value={{
        flaggerClient: new FlaggerClient("http://localhost:3333"),
        flaggerClientContext: new FlaggerClientContext(),
      }}
    >
      {children}
    </Context.Provider>
  );
}

export function useFlaggerClient(): State {
  return useContext(Context);
}
