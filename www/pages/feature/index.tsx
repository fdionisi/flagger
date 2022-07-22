import React, { useEffect } from "react";
import { FeatureList } from "../../components/organisms";
import { ApplicationTemplate } from "../../components/templates";
import { useManagement } from "../../store";

export default function Index() {
  const management = useManagement();

  useEffect(() => {
    management.actions.listFeatures();
  }, []);

  return (
    <ApplicationTemplate
      group="Feature"
      title="List all features"
      description="This is a simple list view with sidebar and header examples using Tailwind CSS and implementing Flagger's API."
      breadcrumbs={[
        {
          text: "Features",
          url: "/feature",
        },
      ]}
    >
      <FeatureList
        features={management.data.listed}
        toggleFeature={management.actions.toggleFeature}
      />
    </ApplicationTemplate>
  );
}
