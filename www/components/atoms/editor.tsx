import dynamic from "next/dynamic";

const MonacoEditor = dynamic(import("@monaco-editor/react"));

interface Props {
  schema?: any;
  value: string;
  onChange: (input?: string) => void;
}

export function Editor({ schema, value, onChange }: Props) {
  return (
    <div className="p-1 bg-white w-full not-prose dark:bg-gray-800 rounded-md border border-gray-200 dark:border-gray-700">
      <MonacoEditor
        height="23.5rem"
        width="100%"
        theme="vs-dark"
        language="json"
        options={{
          tabSize: 2,
        }}
        value={value}
        onChange={(input) => {
          onChange(input);
        }}
        beforeMount={async (editor) => {
          editor.languages.json.jsonDefaults.setDiagnosticsOptions({
            validate: true,
            schemas: [{
              uri: schema.$id,
              fileMatch: ["*"],
              schema,
            }],
          });
        }}
      />
    </div>
  );
}
