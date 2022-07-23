import { FeatureInput } from "../../../lib/flagger-client";
import { Editor } from "../atoms";

interface Props {
  value: Partial<FeatureInput>;
  onChange: any;
}

export function CreateFeatureEditor({ value, onChange }: Props): JSX.Element {
  return (
    <Editor
      value={JSON.stringify(value, null, 2)}
      schema={{
        "$id": "http://localhost:3000/assets/schemas/create-asset-request.json",
        "$schema": "http://json-schema.org/draft-07/schema",
        "description":
          "A JSON representation of the payload required to create a new asset.",
        "required": [
          "name",
          "kind",
        ],
        "title": "Create asset requests Schema",
        "type": "object",
        "properties": {
          "name": {
            "type": "string",
            "description": "A unique name representative of the new feature",
          },
          "description": {
            "type": "string",
            "description": "A small description regarding the feature",
          },
          "kind": {
            "type": "string",
            "description": "A small description regarding the feature",
            "enum": ["KillSwitch"],
          },
        },
      }}
      onChange={(input) => {
        try {
          onChange(JSON.parse(input || ""));
        } catch {}
      }}
    />
  );
}
