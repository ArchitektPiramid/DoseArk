import type {
  CreateStorageLocationInput,
  StorageLocation,
  UpdateStorageLocationInput,
} from "@doseark/types";
import { useEffect, useMemo, useRef, useState } from "react";

import { apiClient } from "@/app/client";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("storage");

type Status = "idle" | "loading" | "saving" | "error";

export function useStorageLocationsPageData() {
  const [rows, setRows] = useState<StorageLocation[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);
  const didInitialLoad = useRef(false);

  const selected = useMemo(() => rows.find((row) => row.id === selectedId), [rows, selectedId]);

  async function loadAll() {
    try {
      setStatus("loading");
      setError(null);

      const result = await apiClient.listStorageLocations(true);
      setRows(result);

      logger.debug("Storage locations loaded", { count: result.length });

      setStatus("idle");
    } catch (err) {
      logger.error("Failed to load storage locations", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  async function create(input: CreateStorageLocationInput) {
    try {
      setStatus("saving");
      setError(null);

      await apiClient.createStorageLocation(input);
      await loadAll();
    } catch (err) {
      logger.error("Failed to create storage location", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  async function update(id: string, input: UpdateStorageLocationInput) {
    try {
      setStatus("saving");
      setError(null);

      await apiClient.updateStorageLocation(id, input);
      await loadAll();
    } catch (err) {
      logger.error("Failed to update storage location", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  async function archive(id: string) {
    try {
      setStatus("saving");
      setError(null);

      await apiClient.archiveStorageLocation(id);

      if (selectedId === id) {
        setSelectedId(null);
      }

      await loadAll();
    } catch (err) {
      logger.error("Failed to archive storage location", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  useEffect(() => {
    if (didInitialLoad.current) return;

    didInitialLoad.current = true;
    void loadAll();
  }, []);

  return {
    rows,
    selected,
    selectedId,
    status,
    error,
    setSelectedId,
    setSelected: setSelectedId,
    clearSelected: () => setSelectedId(null),
    loadAll,
    create,
    update,
    archive,
  };
}
