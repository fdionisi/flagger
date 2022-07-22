import React, { useEffect } from "react";
import { FeatureCard } from "../components/moleculas";
import { CreateFeatureForm } from "../components/organisms";
import { ApplicationTemplate } from "../components/templates";
import { useManagement } from "../store";

export default function Index() {
  const management = useManagement();

  useEffect(() => {
    management.actions.listFeatures();
  }, []);

  return (
    <ApplicationTemplate
      group=""
      title="Dashboard"
      description="This is a simple dashboard with sidebar build in Tailwind CSS and implementing Flagger API."
    >
      <CreateFeatureForm onSubmit={management.actions.createFeature} />
      {management.data.listed.map((feature) => (
        <FeatureCard key={feature._id} feature={feature} />
      ))}
    </ApplicationTemplate>
  );
}
