import { invoke } from "@tauri-apps/api/core";

export async function getAvailableEffects() {
    try {
        const effects = await invoke<string[]>("get_effects_list");
        return effects;
    } catch (error) {
        console.error("Error fetching available effects:", error);
        return [];
    }
}

export async function setEffect(effectName: string) {
    try {
        await invoke("set_effect", { effectName });
    } catch (error) {
        console.error(`Error setting effect ${effectName}:`, error);
    }
}

export async function getCurrentEffect() {
    try {
        const effectName = await invoke<string>("get_current_effect_name");
        return effectName;
    } catch (error) {
        // console.error("Error fetching current effect:", error);
        return null;
    }
}

export async function clearCurrentEffect() {
    try {
        await invoke("clear_effect");
    } catch (error) {
        console.error("Error clearing current effect:", error);
    }
}