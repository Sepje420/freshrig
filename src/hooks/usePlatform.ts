import { useMemo } from "react";
import { type as osType } from "@tauri-apps/plugin-os";

export type Platform = "windows" | "linux" | "macos" | "ios" | "android";

export interface PlatformInfo {
  platform: Platform;
  isWindows: boolean;
  isLinux: boolean;
}

/**
 * Read the host OS once per mount. `type()` from `@tauri-apps/plugin-os`
 * is synchronous (value baked in at app start), so no useEffect is needed.
 * Falls back to `"windows"` when running outside Tauri (e.g. Vite web preview).
 */
export function usePlatform(): PlatformInfo {
  return useMemo<PlatformInfo>(() => {
    let platform: Platform = "windows";
    try {
      platform = osType() as Platform;
    } catch {
      platform = "windows";
    }
    return {
      platform,
      isWindows: platform === "windows",
      isLinux: platform === "linux",
    };
  }, []);
}
