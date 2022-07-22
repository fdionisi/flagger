import { useReducer } from "react";
import { Feature, FeatureInput } from "../../lib/flagger-client";
import { useFlaggerClient } from "../context";
import { ActionImpl } from "./types";

interface State {
  data: Data;
  actions: {
    listFeatures(): Promise<void>;
    createFeature(input: FeatureInput): Promise<void>;
    toggleFeature(feature: string, enabled: boolean): Promise<void>;
  };
}

interface Data {
  listed: Feature[];
}

enum ActionKind {
  List,
}

type Action = ActionImpl<ActionKind.List, Feature[]>;

function dataReducer(state: Data, action: Action) {
  switch (action.kind) {
    case ActionKind.List: {
      return {
        ...state,
        listed: action.payload,
      };
    }
  }
}

export function useManagement(): State {
  const { flaggerClient, flaggerClientContext } = useFlaggerClient();
  const [data, dispatch] = useReducer(dataReducer, {
    listed: [],
  });

  return {
    data,
    actions: {
      async createFeature(input: FeatureInput): Promise<void> {
        try {
          await flaggerClient.management.createFeature(
            flaggerClientContext,
            input,
          );

          dispatch({
            kind: ActionKind.List,
            payload: await flaggerClient.management.listFeatures(
              flaggerClientContext,
            ),
          });
        } catch (error) {
          console.error(error);
        }
      },
      async listFeatures(): Promise<void> {
        try {
          dispatch({
            kind: ActionKind.List,
            payload: await flaggerClient.management.listFeatures(
              flaggerClientContext,
            ),
          });
        } catch (error) {
          console.error(error);
        }
      },
      async toggleFeature(feature: string, enabled: boolean): Promise<void> {
        try {
          await flaggerClient.management.toggleFeature(
            flaggerClientContext,
            feature,
            enabled,
          );

          dispatch({
            kind: ActionKind.List,
            payload: await flaggerClient.management.listFeatures(
              flaggerClientContext,
            ),
          });
        } catch (error) {
          console.error(error);
        }
      },
    },
  };
}
