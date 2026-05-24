import { open } from "@tauri-apps/plugin-dialog";

export async function pickDirectory(initial?: string): Promise<string | null> {
  const selected = await open({
    directory: true,
    multiple: false,
    defaultPath: initial && initial.trim() ? initial.trim() : undefined
  });
  if (!selected) return null;
  return Array.isArray(selected) ? selected[0] ?? null : selected;
}
