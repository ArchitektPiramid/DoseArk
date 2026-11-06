export const productIconOptions = [
  { value: "pill", label: "Capsule / pill · generic" },

  { value: "health-pill-1", label: "Tablet / capsule · single" },
  { value: "health-pills-2", label: "Tablets · 2" },
  { value: "health-pills-3", label: "Tablets · 3" },
  { value: "health-pills-4", label: "Tablets · 4" },
  { value: "health-medicines", label: "Medicines · mixed" },

  { value: "health-blister-oval-1", label: "Blister · oval x1" },
  { value: "health-blister-oval-4", label: "Blister · oval x4" },
  { value: "health-blister-oval-14", label: "Blister · oval x14" },
  { value: "health-blister-oval-16", label: "Blister · oval x16" },
  { value: "health-blister-round-1", label: "Blister · round x1" },
  { value: "health-blister-round-4", label: "Blister · round x4" },
  { value: "health-blister-round-14", label: "Blister · round x14" },
  { value: "health-blister-round-16", label: "Blister · round x16" },
  { value: "health-oral-pack-21", label: "Tablet pack · 21" },
  { value: "health-oral-pack-28", label: "Tablet pack · 28" },

  { value: "health-medicine-bottle", label: "Bottle · medicine" },
  { value: "health-pump-bottle", label: "Bottle · pump" },
  { value: "health-spray", label: "Spray · bottle" },
  { value: "health-powder-formula", label: "Powder · container" },
  { value: "health-medicine-mortar", label: "Powder / compound · mortar" },

  { value: "health-asthma-inhaler", label: "Inhaler" },
  { value: "health-syringe", label: "Injection · syringe" },
  { value: "health-syringe-vaccine", label: "Injection · syringe / vial" },
  { value: "health-contraceptive-injection", label: "Injection · device" },
  { value: "health-sayana-press", label: "Injection · auto device" },
  { value: "health-intravenous-bag", label: "Infusion · IV bag" },
  { value: "health-medical-sample", label: "Vial / medical sample" },
  { value: "health-test-tubes", label: "Vial / tubes" },

  { value: "health-patch", label: "Patch · transdermal" },
  { value: "health-bandage", label: "Patch / adhesive" },
  { value: "health-implant", label: "Implant" },
  { value: "health-ring", label: "Medication ring" },

  { value: "health-prescription", label: "Prescription" },
  { value: "health-rx", label: "Rx" },
  { value: "health-pharmacy", label: "Pharmacy" },
  { value: "health-pharmacy-symbols", label: "Pharmacy symbols" },

  { value: "package", label: "Package · generic" },
  { value: "box", label: "Box · generic" },
  { value: "heart", label: "Heart" },
  { value: "shield", label: "Shield" },
  { value: "leaf", label: "Herbal / leaf" },
  { value: "brain", label: "Brain" },
  { value: "moon", label: "Night / sleep" },
  { value: "zap", label: "Energy" },
] as const;

export type ProductIconName = (typeof productIconOptions)[number]["value"];
