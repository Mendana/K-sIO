import { invoke } from '@tauri-apps/api/core';

export interface EvalResult {
    success: boolean;
    result?: string;
    error?: string;
}

export async function evaluate(expression: string): Promise<EvalResult> {
    console.log("Evaluating expression:", expression);
    return await invoke('evaluate', { expression });
}

export async function getVariables(): Promise<[string, number][]> {
    return await invoke('get_variables');
}

export async function setAngleMode(mode: 'deg' | 'rad' | 'grad'): Promise<void> {
    return await invoke('set_angle_mode', { mode });
}

export async function getAngleMode(): Promise<string> {
    return await invoke('get_angle_mode');
}