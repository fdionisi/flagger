import { CodeIcon } from "@heroicons/react/outline";
import { JSONSchema7 } from "json-schema";
import { useCallback, useMemo, useRef, useState } from "react";
import {
  Button,
  Editor,
  JSONSchemaForm,
  JSONSchemaFormProps,
  JSONSchemaFormRef,
  Modal,
  Row,
} from "../atoms";

interface Props<T> {
  onSubmit: (formData: T) => Promise<void>;
  schema: JSONSchema7;
  uiSchema: JSONSchemaFormProps<T>["uiSchema"];
}

export function InspectableForm<T>(
  { onSubmit, schema, uiSchema }: Props<T>,
): JSX.Element {
  const [showEditor, setEditorVisibility] = useState(false);
  const toggleEditorVisibility = useCallback(
    () => setEditorVisibility(!showEditor),
    [showEditor, setEditorVisibility],
  );

  const ref = useRef<JSONSchemaFormRef<T>>();
  const [formData, setFormData] = useState<T>({} as any);

  const stringyfiedFormData = useMemo(
    () => JSON.stringify(formData, null, 2),
    [formData],
  );

  return (
    <div>
      <Row className="justify-end">
        <button
          type="button"
          className="inline-flex text-gray-500 hover:text-gray-700 dark:text-gray-500 dark:hover:text-gray-400 flex-shrink-0 justify-center items-center h-8 w-8 rounded-md focus:outline-none focus:ring-2 focus:ring-gray-400 focus:ring-offset-2 focus:ring-offset-white text-sm dark:focus:ring-gray-700 dark:focus:ring-offset-gray-800"
          onClick={toggleEditorVisibility}
        >
          <CodeIcon className="h-4 w-4" />
        </button>
      </Row>

      <Row>
        <JSONSchemaForm
          ref={ref as any}
          schema={schema}
          uiSchema={uiSchema}
          formData={formData}
          onSubmit={(event) => onSubmit(event.formData as T)}
          onChange={(event) => {
            setFormData(event.formData as T);
          }}
        />
        <Modal
          className="absolute left-[5%] right-[5%] top-[10%] bottom-[10%] max-w-none"
          isOpen={showEditor}
          onClose={toggleEditorVisibility}
        >
          <Editor
            schema={schema}
            value={stringyfiedFormData}
            onChange={(input) => {
              try {
                const parsedFormData = JSON.parse(input || "");
                if (ref.current) {
                  const { errorSchema } = ref.current.validate(parsedFormData);
                  ref.current.onChange(parsedFormData, errorSchema);
                }
              } catch {}
            }}
          />
        </Modal>
      </Row>

      <Row className="justify-end">
        <Button
          onClick={() => {
            onSubmit(formData);
          }}
        >
          Submit
        </Button>
      </Row>
    </div>
  );
}
