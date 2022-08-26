import { JSONSchema7 } from "json-schema";
import { FlaggerClientContext } from "../context";
import { Inner } from "../inner";

export class ApiDocController {
  #inner: Inner;

  constructor(inner: Inner) {
    this.#inner = inner;
  }

  async get(context: FlaggerClientContext): Promise<JSONSchema7> {
    return this.#inner.request({
      path: "/api-doc/openapi.json",
      method: "GET",
      token: context.token,
    });
  }
}
