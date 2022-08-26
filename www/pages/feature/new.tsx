import { useRouter } from "next/router";
import React, { useCallback, useEffect } from "react";
import { FeatureInput } from "../../../lib/flagger-client";
import { InspectableForm } from "../../components/organisms";
import { ApplicationTemplate } from "../../components/templates";
import { useDoc, useManagement } from "../../store";

export default function New() {
  const router = useRouter();
  const management = useManagement();
  const doc = useDoc();

  useEffect(() => {
    management.actions.listFeatures();
    doc.actions.readApiDocs();
  }, []);

  const onSubmit = useCallback(async (feature: FeatureInput) => {
    await management.actions.createFeature(feature);
    router.push("/feature");
  }, [router, management.actions.createFeature]);

  if (!doc.data.docs) {
    return null;
  }

  return (
    <ApplicationTemplate
      group="Feature"
      title="Create a new feature"
      description="This is a simple creation form with sidebar and header examples using Tailwind CSS and implementing Flagger's API."
      breadcrumbs={[
        {
          text: "Features",
          url: "/feature",
        },
        {
          text: "Create feature",
          url: "/feature/new",
        },
      ]}
    >
      <InspectableForm
        onSubmit={onSubmit}
        schema={doc.data.docs?.FeatureInput}
        uiSchema={{
          "ui:order": ["name", "kind", "description", "*"],
          name: {
            "ui:title": "Name",
          },
          kind: {
            "ui:title": "Kind",
          },
          description: {
            "ui:title": "Description",
            "ui:widget": "textarea",
          },
        }}
      />
    </ApplicationTemplate>
  );
}
