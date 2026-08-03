<script lang="ts">
  import { renderMarkdown } from "../../services/markdown";

  interface Props {
    content: string;
    filePath?: string;
    zoom: number;
  }

  let { content, filePath = "", zoom = 1 }: Props = $props();

  let renderedHtml = $derived(renderMarkdown(content));
</script>

<div class="markdown-wrapper">
  <div class="markdown-body" style="transform: scale({zoom}); transform-origin: top center; width: {100 / zoom}%;">
    {@html renderedHtml}
  </div>
</div>

<style>
  .markdown-wrapper {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-8) var(--space-6);
    background: var(--color-bg);
  }

  .markdown-body {
    max-width: 900px;
    margin: 0 auto;
    color: var(--color-text);
    font-family: var(--font-sans);
    font-size: var(--font-size-base);
    line-height: var(--line-height);
    word-wrap: break-word;
  }

  /* ---- Headings ---- */
  .markdown-body h1, .markdown-body h2, .markdown-body h3,
  .markdown-body h4, .markdown-body h5, .markdown-body h6 {
    margin-top: 24px;
    margin-bottom: 16px;
    font-weight: 600;
    line-height: 1.3;
    color: var(--color-text);
  }
  .markdown-body h1 { font-size: 2em; border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.3em; }
  .markdown-body h2 { font-size: 1.5em; border-bottom: 1px solid var(--color-border-light); padding-bottom: 0.3em; }
  .markdown-body h3 { font-size: 1.25em; }
  .markdown-body h4 { font-size: 1em; }
  .markdown-body h5 { font-size: 0.875em; }
  .markdown-body h6 { font-size: 0.85em; color: var(--color-text-secondary); }

  /* ---- Text ---- */
  .markdown-body p { margin-bottom: 16px; }
  .markdown-body a { color: var(--color-text-link); text-decoration: none; }
  .markdown-body a:hover { text-decoration: underline; }
  .markdown-body strong { font-weight: 600; }
  .markdown-body em { font-style: italic; }
  .markdown-body del { text-decoration: line-through; color: var(--color-text-muted); }
  .markdown-body sup, .markdown-body sub { font-size: 0.75em; }

  /* ---- Lists ---- */
  .markdown-body ul, .markdown-body ol {
    padding-left: 2em;
    margin-bottom: 16px;
  }
  .markdown-body li { margin-bottom: 4px; }
  .markdown-body li > ul, .markdown-body li > ol { margin-bottom: 0; }

  /* ---- Task Lists ---- */
  .markdown-body .task-list-item { list-style-type: none; margin-left: -1.5em; }
  .markdown-body .task-list-item input[type="checkbox"] { margin-right: 8px; }

  /* ---- Blockquote ---- */
  .markdown-body blockquote {
    border-left: 4px solid var(--color-blockquote-border);
    margin: 0 0 16px 0;
    padding: 0 1em;
    color: var(--color-blockquote-text);
  }
  .markdown-body blockquote > :last-child { margin-bottom: 0; }

  /* ---- Code ---- */
  .markdown-body code {
    font-family: var(--font-mono);
    background: var(--color-code-bg);
    color: var(--color-code-text);
    padding: 2px 6px;
    border-radius: var(--radius-sm);
    font-size: 0.875em;
  }
  .markdown-body pre {
    background: var(--color-code-bg);
    padding: 16px;
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin-bottom: 16px;
    line-height: 1.5;
  }
  .markdown-body pre code {
    background: none;
    padding: 0;
    font-size: 0.875em;
    line-height: inherit;
  }

  /* ---- Tables ---- */
  .markdown-body table {
    border-collapse: collapse;
    width: 100%;
    margin-bottom: 16px;
  }
  .markdown-body th, .markdown-body td {
    border: 1px solid var(--color-table-border);
    padding: 8px 12px;
    text-align: left;
  }
  .markdown-body th {
    background: var(--color-bg-alt);
    font-weight: 600;
  }
  .markdown-body tr:nth-child(2n) {
    background: var(--color-table-stripe);
  }

  /* ---- Horizontal Rule ---- */
  .markdown-body hr {
    height: 1px;
    border: none;
    background: var(--color-border-light);
    margin: 24px 0;
  }

  /* ---- Images ---- */
  .markdown-body img {
    max-width: 100%;
    border-radius: var(--radius-md);
  }

  /* ---- Details/Summary ---- */
  .markdown-body details {
    border: 1px solid var(--color-border-light);
    border-radius: var(--radius-md);
    padding: 12px 16px;
    margin-bottom: 16px;
  }
  .markdown-body summary {
    cursor: pointer;
    font-weight: 600;
    color: var(--color-text-secondary);
  }

  /* ---- KBD ---- */
  .markdown-body kbd {
    font-family: var(--font-mono);
    font-size: 0.85em;
    padding: 2px 6px;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    background: var(--color-bg-alt);
    box-shadow: 0 1px 0 var(--color-border);
  }

  /* ===== Dark mode overrides with !important ===== */
  :root.dark .markdown-body {
    color: var(--color-text) !important;
  }
  :root.dark .markdown-body h1,
  :root.dark .markdown-body h2,
  :root.dark .markdown-body h3,
  :root.dark .markdown-body h4,
  :root.dark .markdown-body h5,
  :root.dark .markdown-body h6 {
    color: #e6edf3 !important;
  }
  :root.dark .markdown-body p,
  :root.dark .markdown-body li,
  :root.dark .markdown-body td,
  :root.dark .markdown-body th {
    color: #e6edf3;
  }
  :root.dark .markdown-body a {
    color: #58a6ff !important;
  }
  :root.dark .markdown-body code {
    background: #161b22 !important;
    color: #e6edf3 !important;
  }
  :root.dark .markdown-body pre {
    background: #161b22 !important;
    color: #e6edf3;
  }
  :root.dark .markdown-body blockquote {
    color: #8b949e !important;
  }
  :root.dark .markdown-body th {
    background: #161b22;
  }
  :root.dark .markdown-body tr:nth-child(2n) {
    background: #161b22;
  }
</style>
