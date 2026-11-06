import {
  Backpack,
  Box,
  Brain,
  Building2,
  Heart,
  Home,
  Leaf,
  Moon,
  Package,
  Pill,
  Shield,
  Zap,
} from "lucide-react";

import type { VisualIconName } from "./visualTagOptions";

type Props = {
  name?: string | null;
  size?: number;
};

export function VisualIcon({ name, size = 15 }: Props) {
  switch (name) {
    case "pill":
      return <Pill size={size} />;
    case "package":
      return <Package size={size} />;
    case "box":
      return <Box size={size} />;
    case "home":
      return <Home size={size} />;
    case "building":
      return <Building2 size={size} />;
    case "backpack":
      return <Backpack size={size} />;
    case "heart":
      return <Heart size={size} />;
    case "shield":
      return <Shield size={size} />;
    case "leaf":
      return <Leaf size={size} />;
    case "brain":
      return <Brain size={size} />;
    case "moon":
      return <Moon size={size} />;
    case "zap":
      return <Zap size={size} />;
    default:
      return <Box size={size} />;
  }
}

export function isVisualIconName(value: string): value is VisualIconName {
  return [
    "pill",
    "package",
    "box",
    "home",
    "building",
    "backpack",
    "heart",
    "shield",
    "leaf",
    "brain",
    "moon",
    "zap",
  ].includes(value);
}
