import type { ClassValue } from 'clsx';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function isWindowTauri() {
  return typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
}
