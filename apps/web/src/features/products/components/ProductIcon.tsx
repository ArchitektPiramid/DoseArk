import {
  AsthmaInhaler,
  BabyFormula,
  BandageAdhesive,
  BlisterPillsOvalX1,
  BlisterPillsOvalX14,
  BlisterPillsOvalX16,
  BlisterPillsOvalX4,
  BlisterPillsRoundX1,
  BlisterPillsRoundX14,
  BlisterPillsRoundX16,
  BlisterPillsRoundX4,
  ContraceptiveInjection,
  ContraceptivePatch,
  HormonalRing,
  Implant,
  IntravenousBag,
  MedicalSample,
  MedicineBottle,
  MedicineMortar,
  Medicines,
  OralContraceptionPillsx21,
  OralContraceptionPillsx28,
  Pharmacy,
  PharmacySymbols,
  Pill1,
  Pills2,
  Pills3,
  Pills4,
  PpeSanitizer,
  PrescriptionDocument,
  Rx,
  SayanaPress,
  Syringe,
  SyringeVaccine,
  TestTubes,
  VirusSanitizerSpray,
} from "healthicons-react/outline";

import { VisualIcon } from "@/core/visualTags";

type Props = {
  name?: string | null;
  size?: number;
};

const HEALTH_PRODUCT_ICONS = {
  "health-pill-1": Pill1,
  "health-pills-2": Pills2,
  "health-pills-3": Pills3,
  "health-pills-4": Pills4,
  "health-medicines": Medicines,
  "health-blister-oval-1": BlisterPillsOvalX1,
  "health-blister-oval-4": BlisterPillsOvalX4,
  "health-blister-oval-14": BlisterPillsOvalX14,
  "health-blister-oval-16": BlisterPillsOvalX16,
  "health-blister-round-1": BlisterPillsRoundX1,
  "health-blister-round-4": BlisterPillsRoundX4,
  "health-blister-round-14": BlisterPillsRoundX14,
  "health-blister-round-16": BlisterPillsRoundX16,
  "health-medicine-bottle": MedicineBottle,
  "health-medicine-mortar": MedicineMortar,
  "health-asthma-inhaler": AsthmaInhaler,
  "health-syringe": Syringe,
  "health-syringe-vaccine": SyringeVaccine,
  "health-contraceptive-injection": ContraceptiveInjection,
  "health-sayana-press": SayanaPress,
  "health-intravenous-bag": IntravenousBag,
  "health-medical-sample": MedicalSample,
  "health-test-tubes": TestTubes,
  "health-spray": VirusSanitizerSpray,
  "health-pump-bottle": PpeSanitizer,
  "health-patch": ContraceptivePatch,
  "health-bandage": BandageAdhesive,
  "health-implant": Implant,
  "health-ring": HormonalRing,
  "health-oral-pack-21": OralContraceptionPillsx21,
  "health-oral-pack-28": OralContraceptionPillsx28,
  "health-prescription": PrescriptionDocument,
  "health-rx": Rx,
  "health-pharmacy": Pharmacy,
  "health-pharmacy-symbols": PharmacySymbols,
  "health-powder-formula": BabyFormula,
} as const;

export function ProductIcon({ name, size = 15 }: Props) {
  const healthIconName = name as keyof typeof HEALTH_PRODUCT_ICONS;
  const HealthIcon = HEALTH_PRODUCT_ICONS[healthIconName];

  if (HealthIcon) {
    return <HealthIcon width={size} height={size} />;
  }

  return <VisualIcon name={name ?? "pill"} size={size} />;
}
