import { ManagementController } from "./controllers";
import { Inner } from "./inner";

export class FlaggerClient {
  readonly management: ManagementController;

  constructor(baseUrl: string) {
    const inner = new Inner(baseUrl);
    this.management = new ManagementController(inner);
  }
}
