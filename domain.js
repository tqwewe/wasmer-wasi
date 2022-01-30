import { data_view, UTF8_DECODER, utf8_encode, UTF8_ENCODED_LEN } from './intrinsics.js';
export function addDomainToImports(imports, obj, get_export) {
  if (!("domain" in imports)) imports["domain"] = {};
  imports["domain"]["add"] = function(arg0, arg1, arg2) {
    const memory = get_export("memory");
    const realloc = get_export("canonical_abi_realloc");
    const ptr0 = arg0;
    const len0 = arg1;
    const ret = obj.add(UTF8_DECODER.decode(new Uint8Array(memory.buffer, ptr0, len0)));
    const ptr1 = utf8_encode(ret, realloc, memory);
    const len1 = UTF8_ENCODED_LEN;
    data_view(memory).setInt32(arg2 + 8, len1, true);
    data_view(memory).setInt32(arg2 + 0, ptr1, true);
  };
}