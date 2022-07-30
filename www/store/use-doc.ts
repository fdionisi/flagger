import { Reducer, useReducer } from "react";
import { JSONSchema7 } from "json-schema";
import get from "lodash/get";
import { useFlaggerClient } from "../context";
import { ActionImpl } from "./types";

import toJsonSchema from "@openapi-contrib/openapi-schema-to-json-schema";
import $RefParser from "@apidevtools/json-schema-ref-parser";

interface State {
  data: Data;
  actions: {
    readApiDocs(): Promise<void>;
  };
}

interface Data {
  docs?: Record<string, JSONSchema7>;
}

enum ActionKind {
  Fetch,
}

type Action = ActionImpl<ActionKind.Fetch, Record<string, JSONSchema7>>;

function dataReducer(state: Data, action: Action) {
  switch (action.kind) {
    case ActionKind.Fetch: {
      return {
        ...state,
        docs: action.payload,
      };
    }
  }
}

export function useDoc(): State {
  const { flaggerClient, flaggerClientContext } = useFlaggerClient();
  const [data, dispatch] = useReducer<Reducer<Data, Action>>(
    dataReducer,
    {},
  );

  return {
    data,
    actions: {
      async readApiDocs(): Promise<void> {
        try {
          const parsed = await $RefParser.dereference(toJsonSchema(
            await flaggerClient.apiDoc.get(
              flaggerClientContext,
            ),
          )) as any;

          const schemas: Record<string, JSONSchema7> = get(parsed, [
            "components",
            "schemas",
          ]);

          dispatch({
            kind: ActionKind.Fetch,
            payload: schemas,
          });
        } catch (error) {
          console.error(error);
        }
      },
    },
  };
}
