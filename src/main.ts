import * as Comlink from "comlink";
import type { API } from "./worker";
import MyWorker from './worker?worker'

const fileInput = document.getElementById("fileInput") as HTMLInputElement;
const loader = document.getElementById("loader") as HTMLDivElement;

const worker = new MyWorker();
const workerAPI = Comlink.wrap<API>(worker);

fileInput.addEventListener("change", async (event: Event) => {
    const target = event.target as HTMLInputElement;

    if (!target.files || target.files.length === 0) {
        return;
    }

    fileInput.classList.add("hidden");
    loader.style.display = "flex";

    const file = target.files[0];

    try {
        const buffer: ArrayBuffer = await file.arrayBuffer();
        const uint8 = new Uint8Array(buffer);

        await handleBuffer(uint8, file);
    } catch (err) {
        alert("Error  processing file. Check console for details.");
        console.error("Error file reading:", err);
    } finally {
      fileInput.classList.remove("hidden");
      loader.style.display = "none";
    }
});

async function handleBuffer(buffer: Uint8Array, file: File): Promise<void> {
  const result = await workerAPI.encode(buffer);
  downloadUint8Array(result, `${file.name.split('.')[0]}.fgb`, "application/octet-stream");
}

function downloadUint8Array(
    data: Uint8Array,
    fileName: string,
    mimeType: string = "application/octet-stream"
): void {
    //@ts-ignore
    const blob = new Blob([data], { type: mimeType });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");

    a.href = url;
    a.download = fileName;
    document.body.appendChild(a);
    a.click();

    document.body.removeChild(a);
    URL.revokeObjectURL(url);
}