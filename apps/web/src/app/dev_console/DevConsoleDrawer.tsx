import { useMemo, useState } from "react";

import { clearAppEvents, useAppEvents } from "@/core/events";
import { Button } from "@/core/ui";
import { APP_NAME } from "@/core/version/appVersion";
import { useDebugMode } from "../debug/DebugModeProvider";

const MAX_VISIBLE_EVENTS = 160;

export function DevConsoleDrawer() {
  const debug = useDebugMode();
  const events = useAppEvents();
  const [open, setOpen] = useState(false);

  const rows = useMemo(() => events.slice(0, MAX_VISIBLE_EVENTS), [events]);
  const sessionId = rows[0]?.sessionId.slice(0, 8);

  if (!debug.enabled) {
    return null;
  }

  return (
    <>
      <div className="mt-dev-console-toggle">
        <Button variant={open ? "primary" : "default"} onClick={() => setOpen((value) => !value)}>
          Console ({events.length})
        </Button>
      </div>

      {open ? (
        <section className="mt-dev-console">
          <div className="mt-dev-console__header">
            <div>
              <div className="mt-dev-console__title">{APP_NAME} frontend events</div>
              <div className="mt-dev-console__meta">
                {sessionId ? `session ${sessionId} · ` : ""}
                showing {rows.length}/{events.length}
              </div>
            </div>

            <div className="flex gap-2">
              <Button onClick={clearAppEvents}>Clear</Button>
              <Button onClick={() => setOpen(false)}>Close</Button>
            </div>
          </div>

          <div className="mt-dev-console__body">
            {rows.length === 0 ? (
              <div className="mt-dev-console__empty">No frontend events yet.</div>
            ) : (
              rows.map((event) => <EventRow key={event.id} event={event} />)
            )}
          </div>
        </section>
      ) : null}
    </>
  );
}

function EventRow({ event }: { event: ReturnType<typeof useAppEvents>[number] }) {
  const row = (
    <>
      <span className="mt-dev-console__sequence">#{event.sequence}</span>
      <span>{new Date(event.timestamp).toLocaleTimeString()}</span>
      <span className={`mt-dev-console__level mt-dev-console__level--${event.level}`}>
        {event.level.toUpperCase()}
      </span>
      <span className="mt-dev-console__source">{event.source}</span>
      <span className="mt-dev-console__message">{event.message}</span>
    </>
  );

  if (event.details === undefined) {
    return <div className="mt-dev-console__row">{row}</div>;
  }

  return (
    <details className="mt-dev-console__event">
      <summary className="mt-dev-console__row">{row}</summary>
      <pre className="mt-dev-console__details">{formatDetails(event.details)}</pre>
    </details>
  );
}

function formatDetails(details: unknown): string {
  try {
    return JSON.stringify(details, null, 2);
  } catch (error) {
    return `[Could not serialize details: ${error instanceof Error ? error.message : String(error)}]`;
  }
}
