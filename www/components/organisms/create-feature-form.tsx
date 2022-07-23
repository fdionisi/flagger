import { FeatureInput, FeatureKind } from "../../../lib/flagger-client";
import { Autocomplete, Input, Label, Textarea } from "../atoms";

interface Props {
  value: Partial<FeatureInput>;
  onChange: (input: Partial<FeatureInput>) => void;
}

export function CreateFeatureForm(
  { value, onChange }: Props,
): JSX.Element {
  return (
    <div>
      <div>
        <Label htmlFor="name">
          Name
        </Label>
        <Input
          type="text"
          id="name"
          name="name"
          placeholder="myNewFeature"
          required
          value={value.name || ""}
          onChange={(event) =>
            onChange({
              ...value,
              name: event.target.value,
            })}
        />
      </div>

      <div>
        <Label htmlFor="kind">
          Kind
        </Label>
        <Autocomplete
          id="kind"
          name="kind"
          value={value.kind || ""}
          options={[
            {
              text: FeatureKind.KillSwitch,
              value: FeatureKind.KillSwitch,
              key: FeatureKind.KillSwitch,
            },
          ]}
          onChange={(option) =>
            onChange({
              ...value,
              kind: option?.value as FeatureKind,
            })}
        />
      </div>

      <div>
        <Label htmlFor="description">
          Description
        </Label>
        <Textarea
          id="description"
          name="description"
          rows={10}
          value={value.description || ""}
          onChange={(event) =>
            onChange({
              ...value,
              description: event.target.value,
            })}
        />
      </div>
    </div>
  );
}
