import { useState } from "react";
import { FeatureInput, FeatureKind } from "../../../lib/flagger-client";
import { Autocomplete, Button, Input, Label, Textarea } from "../atoms";

interface Props {
  onSubmit: (input: FeatureInput) => void | Promise<void>;
}

export function CreateFeatureForm({ onSubmit }: Props): JSX.Element {
  const [name, setName] = useState<string>("");
  const [description, setDescription] = useState<string>("");
  const [kind, setKind] = useState<FeatureKind | undefined>();
  return (
    <form
      onSubmit={async (event) => {
        event.preventDefault();

        await onSubmit({
          name,
          kind: kind as FeatureKind,
          description,
        });

        setName("");
        setDescription("");
        setKind(undefined);
      }}
    >
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
          value={name}
          onChange={(event) => setName(event.target.value)}
        />
      </div>

      <div>
        <Label htmlFor="kind">
          Kind
        </Label>
        <Autocomplete
          id="kind"
          name="kind"
          value={kind}
          options={[
            {
              text: FeatureKind.KillSwitch,
              value: FeatureKind.KillSwitch,
              key: FeatureKind.KillSwitch,
            },
          ]}
          onChange={(option) => setKind(option?.value as FeatureKind)}
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
          value={description}
          onChange={(event) => setDescription(event.target.value)}
        />
      </div>

      <Button>
        Create feature
      </Button>
    </form>
  );
}
