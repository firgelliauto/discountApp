import { closeSync, existsSync, openSync } from "node:fs";
const path=new URL("../prisma/dev.sqlite",import.meta.url);
if(!existsSync(path))closeSync(openSync(path,"a"));
