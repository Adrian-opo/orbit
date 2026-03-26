import { writable } from 'svelte/store';
import type { DetailLevel, RightPanelTab } from '../types';

export const detailLevel = writable<DetailLevel>('full');
export const rightPanelTab = writable<RightPanelTab>('diff');
