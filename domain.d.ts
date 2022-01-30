export function addDomainToImports(imports: any, obj: Domain, get_export: (name: string) => WebAssembly.ExportValue): void;
export interface Domain {
  add(input: string): string;
}
