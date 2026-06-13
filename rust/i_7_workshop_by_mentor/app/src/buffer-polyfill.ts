/**
 * Подключать первым в приложении. Ставит global Buffer до загрузки Anchor/borsh,
 * иначе в браузере будет "Expected Buffer" при вызове Program.
 */
import { Buffer } from "buffer";
if (typeof window !== "undefined") {
  (window as unknown as { Buffer: typeof Buffer }).Buffer = Buffer;
}
if (typeof globalThis !== "undefined") {
  (globalThis as unknown as { Buffer: typeof Buffer }).Buffer = Buffer;
}
