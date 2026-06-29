import { useState } from "react";

const defaultLabels = {
  en: {
    recipeHeader: "Recipe",
    descriptionHeader: "What it does",
    copyLabel: "Copy",
    copiedLabel: "Copied",
  },
  zh: {
    recipeHeader: "Recipe",
    descriptionHeader: "做什么",
    copyLabel: "复制",
    copiedLabel: "已复制",
  },
};

/**
 * Specialized table for `just` recipes.
 *
 * Props:
 * - rows: Array<[recipe: string, description: string]>
 * - locale: "en" | "zh"
 *
 * The recipe column renders as a copy-to-clipboard chip so contributors can
 * grab the exact command without selecting around backticks in a markdown
 * table cell.
 */
export default function RecipeTable({ rows = [], locale = "en" }) {
  const t = defaultLabels[locale] ?? defaultLabels.en;

  return (
    <section className="my-6 overflow-hidden rounded-2xl border border-slate-200 dark:border-slate-800">
      <table className="w-full border-collapse text-sm">
        <thead className="bg-slate-100 text-left text-xs uppercase tracking-wider text-slate-500 dark:bg-slate-900 dark:text-slate-400">
          <tr>
            <th className="px-3 py-2 font-semibold">{t.recipeHeader}</th>
            <th className="px-3 py-2 font-semibold">{t.descriptionHeader}</th>
          </tr>
        </thead>
        <tbody>
          {rows.map((row, index) => (
            <RecipeRow
              key={row[0]}
              recipe={row[0]}
              description={row[1]}
              copyLabel={t.copyLabel}
              copiedLabel={t.copiedLabel}
              zebra={index % 2 === 1}
            />
          ))}
        </tbody>
      </table>
    </section>
  );
}

function RecipeRow({ recipe, description, copyLabel, copiedLabel, zebra }) {
  const [copied, setCopied] = useState(false);

  const onCopy = async () => {
    try {
      const text = `just ${recipe}`;
      if (navigator?.clipboard?.writeText) {
        await navigator.clipboard.writeText(text);
      } else {
        // SSR / older browsers fallback.
        const el = document.createElement("textarea");
        el.value = text;
        document.body.appendChild(el);
        el.select();
        document.execCommand("copy");
        document.body.removeChild(el);
      }
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1200);
    } catch {
      /* clipboard unavailable — silently ignore */
    }
  };

  return (
    <tr
      className={`border-t border-slate-200 align-top dark:border-slate-800 ${
        zebra ? "bg-slate-50/60 dark:bg-slate-900/40" : ""
      }`}
    >
      <td className="px-3 py-2">
        <div className="flex flex-wrap items-center gap-2">
          <code className="rounded-md border border-slate-200 bg-slate-50 px-2 py-0.5 font-mono text-[0.85em] text-sky-700 dark:border-slate-700 dark:bg-slate-950 dark:text-sky-300">
            just {recipe}
          </code>
          <button
            type="button"
            onClick={onCopy}
            className="rounded-md border border-slate-200 bg-white px-2 py-0.5 text-xs font-medium text-slate-500 transition hover:border-sky-300 hover:text-sky-600 dark:border-slate-700 dark:bg-slate-900 dark:text-slate-400 dark:hover:border-sky-700 dark:hover:text-sky-300"
            aria-label={`${copyLabel}: just ${recipe}`}
          >
            {copied ? copiedLabel : copyLabel}
          </button>
        </div>
      </td>
      <td className="px-3 py-2 text-slate-700 dark:text-slate-300">
        {description}
      </td>
    </tr>
  );
}
