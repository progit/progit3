// Parse the repo-root progress.asc — a hand-edited AsciiDoc checklist — into
// structured data for the progress page. The file's contract is simple:
// `== Section title` headings with `* [ ]` / `* [x]` checklist items below.
// Anything else (the doc title, paragraphs, comments) is ignored.

import progressSrc from '../../../progress.asc?raw';

export interface Task {
  text: string;
  done: boolean;
  /** 0 for `*` items, 1 for `**`, … */
  depth: number;
}

export interface Section {
  title: string;
  tasks: Task[];
  done: number;
  total: number;
}

export interface Progress {
  sections: Section[];
  done: number;
  total: number;
  percent: number;
}

export function parseProgress(src: string): Progress {
  const sections: Section[] = [];
  let current: Section | null = null;

  for (const line of src.split(/\r?\n/)) {
    const heading = line.match(/^={2,3} +(.+?)\s*$/);
    if (heading) {
      current = { title: heading[1], tasks: [], done: 0, total: 0 };
      sections.push(current);
      continue;
    }

    const task = line.match(/^(\*+) +\[([ xX*])\] +(.+?)\s*$/);
    if (task && current) {
      const done = task[2] !== ' ';
      current.tasks.push({ text: task[3], done, depth: task[1].length - 1 });
      current.total += 1;
      if (done) current.done += 1;
    }
  }

  const kept = sections.filter((s) => s.total > 0);
  const done = kept.reduce((n, s) => n + s.done, 0);
  const total = kept.reduce((n, s) => n + s.total, 0);
  return {
    sections: kept,
    done,
    total,
    percent: total === 0 ? 0 : Math.round((done / total) * 100),
  };
}

/** Render a task's text as HTML: escape, then apply `backtick code` and `->` arrows. */
export function renderInline(text: string): string {
  return text
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replace(/`([^`]+)`/g, '<code>$1</code>')
    .replaceAll('-&gt;', '\u2192');
}

export const progress = parseProgress(progressSrc);
