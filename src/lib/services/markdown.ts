import MarkdownIt from "markdown-it";
import DOMPurify from "dompurify";
import hljs from "highlight.js";

/**
 * Configured markdown-it instance with GFM, syntax highlighting,
 * and all required extensions.
 */
const md = new MarkdownIt({
  html: true,           // Allow HTML in source
  linkify: true,        // Auto-convert URLs to links
  typographer: true,    // Smart quotes, dashes, etc.
  breaks: false,        // Single newline = <br> (GFM off)
  xhtmlOut: false,
});

// Syntax highlighting via highlight.js
md.set({
  highlight: (str: string, lang: string): string => {
    if (lang && hljs.getLanguage(lang)) {
      try {
        const highlighted = hljs.highlight(str, {
          language: lang,
          ignoreIllegals: true,
        }).value;
        return `<pre><code class="hljs language-${lang}">${highlighted}</code></pre>`;
      } catch {
        // Fall through to auto-detection
      }
    }
    // Auto-detect language
    try {
      const highlighted = hljs.highlightAuto(str).value;
      return `<pre><code class="hljs">${highlighted}</code></pre>`;
    } catch {
      return `<pre><code>${md.utils.escapeHtml(str)}</code></pre>`;
    }
  },
});

/** Sanitize rendered HTML to prevent XSS */
export function sanitize(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: [
      "h1", "h2", "h3", "h4", "h5", "h6",
      "p", "br", "hr",
      "ul", "ol", "li",
      "blockquote", "pre", "code",
      "table", "thead", "tbody", "tr", "th", "td",
      "a", "img", "em", "strong", "del", "ins",
      "sup", "sub", "small", "span", "div",
      "input", "label",
      "details", "summary",
    ],
    ALLOWED_ATTR: [
      "href", "src", "alt", "title", "class", "id",
      "target", "rel", "type", "checked", "disabled",
      "width", "height", "align",
    ],
    ALLOW_DATA_ATTR: false,
  });
}

/** Parse markdown to HTML */
export function parseMarkdown(content: string): string {
  return md.render(content);
}

/** Parse and sanitize markdown in one step */
export function renderMarkdown(content: string): string {
  const rawHtml = parseMarkdown(content);
  return sanitize(rawHtml);
}

/** Extract the first heading from markdown content */
export function extractTitle(content: string): string {
  const match = content.match(/^#\s+(.+)$/m);
  return match ? match[1].trim() : "";
}

/** Count words in markdown content (strips formatting) */
export function countWords(content: string): number {
  const plain = content
    .replace(/```[\s\S]*?```/g, "")  // Remove code blocks
    .replace(/`[^`]+`/g, "")          // Remove inline code
    .replace(/[#*_~\[\]()>|-]/g, " ") // Remove formatting chars
    .replace(/\s+/g, " ")
    .trim();
  return plain ? plain.split(" ").length : 0;
}

/** Estimate reading time in minutes */
export function readingTime(content: string): number {
  const words = countWords(content);
  return Math.max(1, Math.ceil(words / 200));
}

export default md;
