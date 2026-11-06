import type {
  CreateNutrientInput,
  Nutrient,
  NutrientKind,
  UpdateNutrientInput,
} from "@doseark/types";
import { useEffect, useState } from "react";
import { Button, Field, FormActions, Panel, SelectInput, TextArea, TextInput } from "@/core/ui";

const KIND_OPTIONS: NutrientKind[] = [
  "vitamin",
  "mineral",
  "amino_acid",
  "fatty_acid",
  "herb_extract",
  "drug_substance",
  "probiotic",
  "other",
];

type Mode = "create" | "edit";

type Props = {
  mode: Mode;
  nutrient?: Nutrient;
  disabled?: boolean;
  onNew: () => void;
  onCreate: (input: CreateNutrientInput) => void;
  onUpdate: (id: string, input: UpdateNutrientInput) => void;
};

export function NutrientFormPanel({ mode, nutrient, disabled, onNew, onCreate, onUpdate }: Props) {
  const [canonicalName, setCanonicalName] = useState("");
  const [displayName, setDisplayName] = useState("");
  const [kind, setKind] = useState<NutrientKind>("vitamin");
  const [defaultUnit, setDefaultUnit] = useState("mg");
  const [notes, setNotes] = useState("");
  const [referenceDailyAmount, setReferenceDailyAmount] = useState<number | "">("");
  const [referenceDailyUnit, setReferenceDailyUnit] = useState("");
  const [referenceSource, setReferenceSource] = useState("");
  const [referenceNotes, setReferenceNotes] = useState("");

  useEffect(() => {
    if (mode === "edit" && nutrient) {
      setCanonicalName(nutrient.canonical_name);
      setDisplayName(nutrient.display_name);
      setKind(nutrient.kind);
      setDefaultUnit(nutrient.default_unit);
      setNotes(nutrient.notes ?? "");
      setReferenceDailyAmount(nutrient.reference_daily_amount ?? "");
      setReferenceDailyUnit(nutrient.reference_daily_unit ?? "");
      setReferenceSource(nutrient.reference_source ?? "");
      setReferenceNotes(nutrient.reference_notes ?? "");
      return;
    }

    setCanonicalName("");
    setDisplayName("");
    setKind("vitamin");
    setDefaultUnit("mg");
    setNotes("");
    setReferenceDailyAmount("");
    setReferenceDailyUnit("");
    setReferenceSource("");
    setReferenceNotes("");
  }, [mode, nutrient]);

  const canSubmit =
    canonicalName.trim().length > 0 &&
    displayName.trim().length > 0 &&
    defaultUnit.trim().length > 0;

  function submit() {
    if (!canSubmit) return;

    const input = {
      canonical_name: canonicalName.trim(),
      display_name: displayName.trim(),
      kind,
      default_unit: defaultUnit.trim(),
      notes: notes.trim() || null,
      reference_daily_amount: referenceDailyAmount === "" ? null : referenceDailyAmount,
      reference_daily_unit: referenceDailyUnit.trim() || null,
      reference_source: referenceSource.trim() || null,
      reference_notes: referenceNotes.trim() || null,
    };

    if (mode === "edit" && nutrient) {
      onUpdate(nutrient.id, input);
      return;
    }

    onCreate(input);
  }

  return (
    <Panel
      title={mode === "edit" ? "Edit nutrient" : "Create nutrient"}
      action={<Button onClick={onNew}>New nutrient</Button>}
    >
      <div className="mt-form">
        <Field label="Display name">
          <TextInput
            value={displayName}
            placeholder="Vitamin C"
            onChange={(event) => setDisplayName(event.target.value)}
          />
        </Field>

        <Field label="Canonical name">
          <TextInput
            value={canonicalName}
            placeholder="vitamin_c"
            onChange={(event) => setCanonicalName(event.target.value)}
          />
        </Field>

        <div className="mt-form-grid mt-form-grid--2">
          <Field label="Kind">
            <SelectInput
              value={kind}
              onChange={(event) => setKind(event.target.value as NutrientKind)}
            >
              {KIND_OPTIONS.map((option) => (
                <option key={option} value={option}>
                  {option}
                </option>
              ))}
            </SelectInput>
          </Field>

          <Field label="Default unit">
            <TextInput
              value={defaultUnit}
              placeholder="mg, ug, IU, CFU..."
              onChange={(event) => setDefaultUnit(event.target.value)}
            />
          </Field>
        </div>

        <div className="mt-separator" />

        <div className="mt-form-grid mt-form-grid--2">
          <Field label="Global daily reference amount">
            <TextInput
              type="number"
              min={0}
              step={0.01}
              value={referenceDailyAmount}
              onChange={(event) =>
                setReferenceDailyAmount(event.target.value === "" ? "" : Number(event.target.value))
              }
            />
          </Field>

          <Field label="Global daily reference unit">
            <TextInput
              value={referenceDailyUnit}
              placeholder="mg, ug, IU, CFU..."
              onChange={(event) => setReferenceDailyUnit(event.target.value)}
            />
          </Field>
        </div>

        <div className="mt-form-grid mt-form-grid--2">
          <Field label="Reference source">
            <TextInput
              value={referenceSource}
              placeholder="EU NRV, custom, label..."
              onChange={(event) => setReferenceSource(event.target.value)}
            />
          </Field>

          <Field label="Reference notes">
            <TextInput
              value={referenceNotes}
              onChange={(event) => setReferenceNotes(event.target.value)}
            />
          </Field>
        </div>

        <Field label="Notes">
          <TextArea value={notes} onChange={(event) => setNotes(event.target.value)} />
        </Field>

        <FormActions>
          <Button variant="primary" disabled={!canSubmit || disabled} onClick={submit}>
            {disabled ? "Saving..." : mode === "edit" ? "Save nutrient" : "Create nutrient"}
          </Button>
        </FormActions>
      </div>
    </Panel>
  );
}
