import { useMemo, useState } from "react";

const defaultLabels = {
  en: {
    searchPlaceholder: "Filter by name, code, or message…",
    empty: "No matching entries.",
    codeHeader: "Code",
    nameHeader: "Name",
    messageHeader: "Message",
  },
  zh: {
    searchPlaceholder: "按名称、数值或描述过滤…",
    empty: "没有匹配的条目。",
    codeHeader: "数值",
    nameHeader: "名称",
    messageHeader: "描述",
  },
};

/**
 * Searchable, copy-paste friendly table for SDK constant tables (error codes,
 * pixel types, structures…).
 *
 * Props:
 * - rows: Array<[code: string, name: string, message: string]>
 * - locale: "en" | "zh"
 */
export default function ConstantTable({ rows = [], locale = "en" }) {
  const t = defaultLabels[locale] ?? defaultLabels.en;
  const [query, setQuery] = useState("");

  const filtered = useMemo(() => {
    const needle = query.trim().toLowerCase();
    if (!needle) return rows;
    return rows.filter((row) =>
      row.some((cell) => String(cell).toLowerCase().includes(needle))
    );
  }, [rows, query]);

  return (
    <section className="my-6 overflow-hidden rounded-2xl border border-slate-200 dark:border-slate-800">
      <div className="border-b border-slate-200 bg-slate-50 p-3 dark:border-slate-800 dark:bg-slate-900">
        <input
          type="search"
          value={query}
          onChange={(event) => setQuery(event.target.value)}
          placeholder={t.searchPlaceholder}
          className="w-full rounded-lg border border-slate-300 bg-white px-3 py-2 text-sm text-slate-900 shadow-sm outline-none focus:border-sky-400 focus:ring-2 focus:ring-sky-200 dark:border-slate-700 dark:bg-slate-950 dark:text-slate-100 dark:focus:border-sky-500 dark:focus:ring-sky-900"
        />
      </div>
      <div className="max-h-[28rem] overflow-auto">
        <table className="w-full border-collapse text-sm">
          <thead className="sticky top-0 bg-slate-100 text-left text-xs uppercase tracking-wider text-slate-500 dark:bg-slate-900 dark:text-slate-400">
            <tr>
              <th className="px-3 py-2 font-semibold">{t.nameHeader}</th>
              <th className="px-3 py-2 font-semibold">{t.codeHeader}</th>
              <th className="px-3 py-2 font-semibold">{t.messageHeader}</th>
            </tr>
          </thead>
          <tbody>
            {filtered.map((row, index) => (
              <tr
                key={`${row[1]}-${index}`}
                className="border-t border-slate-200 align-top dark:border-slate-800"
              >
                <td className="px-3 py-2">
                  <code className="text-sky-700 dark:text-sky-300">{row[1]}</code>
                </td>
                <td className="px-3 py-2">
                  <code className="text-slate-700 dark:text-slate-300">{row[0]}</code>
                </td>
                <td className="px-3 py-2 text-slate-700 dark:text-slate-300">
                  {row[2]}
                </td>
              </tr>
            ))}
            {filtered.length === 0 && (
              <tr>
                <td
                  colSpan={3}
                  className="px-3 py-6 text-center text-sm text-slate-500"
                >
                  {t.empty}
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>
    </section>
  );
}
