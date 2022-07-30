import dynamic from "next/dynamic";

const MonacoEditor = dynamic(import("@monaco-editor/react"));

interface Props {
  schema?: any;
  value: string;
  onChange: (input?: string) => void;
}

export function Editor({ schema, value, onChange }: Props) {
  return (
    <div className="w-full h-full not-prose">
      <MonacoEditor
        height="100%"
        width="100%"
        theme="vs-dark"
        language="json"
        options={{
          minimap: {
            enabled: false,
          },
          tabSize: 2,
          formatOnType: true,
          formatOnPaste: true,
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
