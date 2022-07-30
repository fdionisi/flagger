import { ApiDocController, ManagementController } from "./controllers";
import { Inner } from "./inner";

export class FlaggerClient {
  readonly apiDoc: ApiDocController;
  readonly management: ManagementController;

  constructor(baseUrl: string) {
    const inner = new Inner(baseUrl);
    this.apiDoc = new ApiDocController(inner);
    this.management = new ManagementController(inner);
  }
}
