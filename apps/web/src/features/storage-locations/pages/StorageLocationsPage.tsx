import { ApiErrorNotice } from "@/app/system_status";
import { Button, PageHeader } from "@/core/ui";

import { StorageLocationFormPanel } from "../components/StorageLocationFormPanel";
import { StorageLocationsTable } from "../components/StorageLocationsTable";
import { useStorageLocationsPageData } from "../hooks/useStorageLocationsPageData";

export function StorageLocationsPage() {
  const data = useStorageLocationsPageData();
  const isSaving = data.status === "saving";

  return (
    <>
      <PageHeader
        title="Storage locations"
        description="Create and manage places where medicine and supplement packages are stored."
        action={<Button onClick={() => void data.loadAll()}>Reload</Button>}
      />

      {data.error ? (
        <ApiErrorNotice error={data.error} onRetry={() => void data.loadAll()} />
      ) : null}

      <div className="mt-grid mt-grid-master-detail">
        <StorageLocationFormPanel
          selected={data.selected}
          disabled={isSaving}
          onCreate={(input) => void data.create(input)}
          onUpdate={(id, input) => void data.update(id, input)}
          onClear={data.clearSelected}
        />

        <StorageLocationsTable
          rows={data.rows}
          selectedId={data.selectedId}
          onSelect={data.setSelected}
        />
      </div>
    </>
  );
}
