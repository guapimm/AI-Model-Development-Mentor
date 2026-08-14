// Sync embedded prompt content from ../../cli/files into ./lib before publishing.
import { cp, mkdir, readdir, rm } from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const src = path.resolve(here, "../../cli/files");
const dest = path.resolve(here, "../lib");

await rm(dest, { recursive: true, force: true });
await mkdir(dest, { recursive: true });
for (const lang of await readdir(src)) {
  await cp(path.join(src, lang), path.join(dest, lang), { recursive: true });
}
console.log("npm lib/ synced from cli/files");
