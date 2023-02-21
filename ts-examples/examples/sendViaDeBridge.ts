import { initAll } from "./helpers";

async function main() {
  const { connection, wallet, example, deBridge } = initAll();
}

main().catch(console.error);
