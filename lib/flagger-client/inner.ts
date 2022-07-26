interface InnerRequest {
  path: string;
  method: "DELETE" | "GET" | "POST" | "PUT";
  query?: Record<string, string>;
  body?: unknown;
  token?: string;
}

export class Inner {
  #baseUrl: string;

  constructor(baseUrl: string) {
    this.#baseUrl = baseUrl;
  }

  async request<T>({
    path,
    method,
    query = {},
    body,
    token,
  }: InnerRequest): Promise<T> {
    const url = `${this.#baseUrl}/${path}`.replace("//", "/");
    const searchParams = new URLSearchParams();
    Object.entries(query).forEach(([key, value]) => {
      searchParams.append(key, value);
    });

    const request = new Request(`${url}?${searchParams.toString()}`, {
      method,
      headers: {
        "authorization": token ? `Bearer ${token}` : "",
        "content-type": body ? "application/json" : "",
      },
      mode: "cors",
      body: body ? JSON.stringify(body) : undefined,
    });

    const response = await fetch(request);
    return response.json();
  }
}
