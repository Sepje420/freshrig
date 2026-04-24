export interface ShellExtension {
  name: string;
  clsid: string;
  dllPath: string;
  company: string | null;
  isBlocked: boolean;
  isMicrosoft: boolean;
}
