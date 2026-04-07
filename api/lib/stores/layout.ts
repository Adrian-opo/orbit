import { writable } from 'svelte/store';

export type PaneId = 'tl' | 'tr' | 'bl' | 'br';

export interface SplitLayout {
  panes: Record<PaneId, number | null>;
  visible: PaneId[];
  focused: PaneId;
}

const defaultLayout: SplitLayout = {
  panes: { tl: null, tr: null, bl: null, br: null },
  visible: ['tl'],
  focused: 'tl',
};

export const splitLayout = writable<SplitLayout>(defaultLayout);

/** Put a session in a pane that is already visible; focus it. */
export function assignSession(paneId: PaneId, sessionId: number): void {
  splitLayout.update((l) => ({
    ...l,
    panes: { ...l.panes, [paneId]: sessionId },
    focused: paneId,
  }));
}

/** Open a pane (add to visible if needed), assign a session, focus it. */
export function openPane(paneId: PaneId, sessionId: number): void {
  splitLayout.update((l) => {
    const alreadyVisible = l.visible.includes(paneId);
    return {
      ...l,
      panes: { ...l.panes, [paneId]: sessionId },
      visible: alreadyVisible ? l.visible : [...l.visible, paneId],
      focused: paneId,
    };
  });
}

/** Close a pane. Never closes the last remaining pane. Clears its session. */
export function closePane(paneId: PaneId): void {
  splitLayout.update((l) => {
    const visible = l.visible.filter((p) => p !== paneId);
    if (visible.length === 0) return l; // guard: never close last
    const focused = l.focused === paneId ? visible[0] : l.focused;
    return {
      ...l,
      panes: { ...l.panes, [paneId]: null },
      visible,
      focused,
    };
  });
}

/** Set which pane has keyboard/MetaPanel focus. */
export function focusPane(paneId: PaneId): void {
  splitLayout.update((l) => ({ ...l, focused: paneId }));
}

/** Remove a deleted/hidden session from every pane that held it. */
export function clearSession(sessionId: number): void {
  splitLayout.update((l) => {
    const panes = { ...l.panes };
    (Object.keys(panes) as PaneId[]).forEach((key) => {
      if (panes[key] === sessionId) panes[key] = null;
    });
    return { ...l, panes };
  });
}
