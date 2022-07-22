export interface ActionImpl<Kind, Payload> {
  kind: Kind;
  payload: Payload;
}
