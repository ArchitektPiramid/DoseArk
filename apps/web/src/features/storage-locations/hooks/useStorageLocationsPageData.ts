import type {
  CreateStorageLocationInput,
  StorageLocation,
  UpdateStorageLocationInput,
} from "@doseark/types";
import { useEffect, useMemo, useRef, useState } from "react";

import { useApiProfiles } from "@/app/api_profiles";
import { apiClient } from "@/app/client";
import { createRequestGate } from "@/core/async/requestGate";
import { createLogger } from "@/core/logger/logger";

const logger = createLogger("storage");
type Status = "idle" | "loading" | "saving" | "error";

export function useStorageLocationsPageData() {
  const { effectiveApiBaseUrl } = useApiProfiles();
  const [rows, setRows] = useState<StorageLocation[]>([]);
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const [status, setStatus] = useState<Status>("idle");
  const [error, setError] = useState<string | null>(null);
  const requestGate = useRef(createRequestGate());
  const activeProfileUrl = useRef(effectiveApiBaseUrl);

  const selected = useMemo(() => rows.find((row) => row.id === selectedId), [rows, selectedId]);

  async function loadAll() {
    if (activeProfileUrl.current !== effectiveApiBaseUrl) {
      activeProfileUrl.current = effectiveApiBaseUrl;
      requestGate.current.invalidate();
      setRows([]);
      setSelectedId(null);
    }
    const requestId = requestGate.current.begin();
    const requestBaseUrl = apiClient.getBaseUrl();

    try {
      setStatus("loading");
      setError(null);
      const result = await apiClient.listStorageLocations(true);
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;
      setRows(result);
      logger.debug("Storage locations loaded", { count: result.length });
      setStatus("idle");
    } catch (err) {
      if (!requestGate.current.isCurrent(requestId) || requestBaseUrl !== apiClient.getBaseUrl()) return;
      logger.error("Failed to load storage locations", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  async function create(input: CreateStorageLocationInput) {
    try {
      requestGate.current.invalidate();
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
      requestGate.current.invalidate();
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
      requestGate.current.invalidate();
      setStatus("saving");
      setError(null);
      await apiClient.archiveStorageLocation(id);
      if (selectedId === id) setSelectedId(null);
      await loadAll();
    } catch (err) {
      logger.error("Failed to archive storage location", err);
      setError(err instanceof Error ? err.message : "Unknown error");
      setStatus("error");
    }
  }

  useEffect(() => {
    void loadAll();
    return () => requestGate.current.invalidate();
  }, [effectiveApiBaseUrl]);

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
