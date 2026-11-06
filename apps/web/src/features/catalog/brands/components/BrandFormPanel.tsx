import type { Brand, CreateBrandInput, UpdateBrandInput } from "@doseark/types";
import { useEffect, useState } from "react";
import { Button, Field, FormActions, Panel, TextArea, TextInput } from "@/core/ui";

type Mode = "create" | "edit";

type Props = {
  mode: Mode;
  brand?: Brand;
  disabled?: boolean;
  onNew: () => void;
  onCreate: (input: CreateBrandInput) => void;
  onUpdate: (id: string, input: UpdateBrandInput) => void;
};

export function BrandFormPanel({ mode, brand, disabled, onNew, onCreate, onUpdate }: Props) {
  const [name, setName] = useState("");
  const [country, setCountry] = useState("");
  const [website, setWebsite] = useState("");
  const [notes, setNotes] = useState("");

  useEffect(() => {
    if (mode === "edit" && brand) {
      setName(brand.name);
      setCountry(brand.country ?? "");
      setWebsite(brand.website ?? "");
      setNotes(brand.notes ?? "");
      return;
    }

    setName("");
    setCountry("");
    setWebsite("");
    setNotes("");
  }, [mode, brand]);

  const canSubmit = name.trim().length > 0;

  function submit() {
    if (!canSubmit) return;

    const input = {
      name: name.trim(),
      country: country.trim() || null,
      website: website.trim() || null,
      notes: notes.trim() || null,
    };

    if (mode === "edit" && brand) {
      onUpdate(brand.id, input);
      return;
    }

    onCreate(input);
  }

  return (
    <Panel
      title={mode === "edit" ? "Edit brand" : "Create brand"}
      action={<Button onClick={onNew}>New brand</Button>}
    >
      <div className="mt-form">
        <Field label="Brand name">
          <TextInput value={name} onChange={(event) => setName(event.target.value)} />
        </Field>

        <div className="mt-form-grid mt-form-grid--2">
          <Field label="Country">
            <TextInput value={country} onChange={(event) => setCountry(event.target.value)} />
          </Field>

          <Field label="Website">
            <TextInput value={website} onChange={(event) => setWebsite(event.target.value)} />
          </Field>
        </div>

        <Field label="Notes">
          <TextArea value={notes} onChange={(event) => setNotes(event.target.value)} />
        </Field>

        <FormActions>
          <Button variant="primary" disabled={!canSubmit || disabled} onClick={submit}>
            {disabled ? "Saving..." : mode === "edit" ? "Save brand" : "Create brand"}
          </Button>
        </FormActions>
      </div>
    </Panel>
  );
}
