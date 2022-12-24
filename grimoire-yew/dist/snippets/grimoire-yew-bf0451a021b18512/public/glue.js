const invoke = window.__TAURI__.invoke

export async function invoke_open_grimoire_dialog() {
    console.log("FUCKL!");
    var result = await invoke("open_grimoire_dialog", {});
    console.log(result);
    return result;
}