import { Feature, FeatureInput } from "../entities";
import { FlaggerClientContext } from "../context";
import { Inner } from "../inner";

export class ManagementController {
  #inner: Inner;

  constructor(inner: Inner) {
    this.#inner = inner;
  }

  async createFeature(context: FlaggerClientContext, input: FeatureInput) {
    return this.#inner.request({
      path: "/management/feature",
      method: "POST",
      token: context.token,
      body: input,
    });
  }

  async listFeatures(context: FlaggerClientContext): Promise<Feature[]> {
    return this.#inner.request({
      path: "/management/feature",
      method: "GET",
      token: context.token,
    });
  }

  async toggleFeature(
    context: FlaggerClientContext,
    feature: string,
    enabled: boolean,
  ): Promise<Feature> {
    const toggleStatus = enabled ? "enable" : "disable";
    return this.#inner.request({
      path: `/management/feature/${feature}/${toggleStatus}`,
      method: "POST",
      token: context.token,
    });
  }
}
