import { StockQuantityCell } from "@/core/ui";

type Props = {
  remaining: number | null | undefined;
  initial: number | null | undefined;
};

export function InventoryLotRemainingCell({ remaining, initial }: Props) {
  return (
    <StockQuantityCell
      remaining={remaining}
      initial={initial}
    />
  );
}
