import type { UserRole } from 'src/types/users.type';
import { writable } from 'svelte/store';

export const userStore = writable<{
  id: string;
  email: string[];
  role: UserRole;
} | null>(null);
export const isReminder = writable(false);
export const apiStore = writable<{ [key: string]: () => void }>();
