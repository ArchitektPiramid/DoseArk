/** Rejects results from older requests when a newer load or reset supersedes them. */
export function createRequestGate() {
  let currentRequestId = 0;

  return {
    begin(): number {
      currentRequestId += 1;
      return currentRequestId;
    },
    isCurrent(requestId: number): boolean {
      return requestId === currentRequestId;
    },
    invalidate(): void {
      currentRequestId += 1;
    },
  };
}
