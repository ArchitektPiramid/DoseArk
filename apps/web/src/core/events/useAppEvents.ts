import { useSyncExternalStore } from "react";
import { getAppEventsSnapshot, subscribeAppEvents } from "./appEventStore";

export function useAppEvents() {
  return useSyncExternalStore(subscribeAppEvents, getAppEventsSnapshot, getAppEventsSnapshot);
}
