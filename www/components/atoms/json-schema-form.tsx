import Form, { FormProps } from "@rjsf/core";
import { ForwardedRef, forwardRef } from "react";

export type JSONSchemaFormRef<T> = Form<T>;

export type JSONSchemaFormProps<T> = FormProps<T>;

export const JSONSchemaForm = forwardRef(
  function JSONSchemaForm<T,>(
    { uiSchema = {}, schema, ...rest }: JSONSchemaFormProps<T>,
    ref: ForwardedRef<Form<T>>,
  ): JSX.Element {
    return (
      <Form<T>
        {...rest}
        ref={ref as any}
        schema={schema}
        uiSchema={{
          "ui:submitButtonOptions": {
            "norender": true,
          },
          ...uiSchema,
        }}
      />
    );
  },
);
