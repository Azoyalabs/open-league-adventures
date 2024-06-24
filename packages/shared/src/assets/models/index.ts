/// <reference types="vite/client" />

import type { MODELS } from "$src/types";

export const models = import.meta.glob("./*.glb", {
  query: "?url",
  eager: true,
import: "default",
}) as Record<MODELS, string>;
