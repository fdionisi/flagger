import { useRouter } from "next/router";
import React, { useCallback, useEffect } from "react";
import { FeatureInput } from "../../../lib/flagger-client";
import { CreateFeature } from "../../components/organisms/create-feature";
import { ApplicationTemplate } from "../../components/templates";
import { useManagement } from "../../store";

export default function New() {
  const router = useRouter();
  const management = useManagement();

  useEffect(() => {
    management.actions.listFeatures();
  }, []);

  const onSubmit = useCallback(async (feature: FeatureInput) => {
    await management.actions.createFeature(feature);
    router.push("/feature");
  }, [router, management.actions.createFeature]);

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
      <CreateFeature onSubmit={onSubmit} />
    </ApplicationTemplate>
  );
}
