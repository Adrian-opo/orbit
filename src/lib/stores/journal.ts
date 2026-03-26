import { writable } from 'svelte/store';
import type { JournalEntry } from '../types';

export const journal = writable<JournalEntry[]>([]);
