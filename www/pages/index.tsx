import React, { useEffect } from "react";
import { FeatureCard } from "../components/moleculas";
import { ApplicationTemplate } from "../components/templates";
import { useDoc, useManagement } from "../store";

export default function Index() {
  const docs = useDoc();
  const management = useManagement();

  useEffect(() => {
    docs.actions.readApiDocs();
    management.actions.listFeatures();
  }, []);

  return (
    <ApplicationTemplate
      group=""
      title="Dashboard"
      description="This is a simple dashboard with sidebar build in Tailwind CSS and implementing Flagger API."
    >
      {management.data.listed.map((feature) => (
        <FeatureCard key={feature.name} feature={feature} />
      ))}
    </ApplicationTemplate>
  );
}
