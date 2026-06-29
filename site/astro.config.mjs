// @ts-check
import { defineConfig } from "astro/config";
import react from "@astrojs/react";
import starlight from "@astrojs/starlight";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  site: "https://xiaoze-cn.github.io/hikcamera-rs",
  vite: {
    plugins: [tailwindcss()],
    resolve: {
      alias: {
        "@components": new URL("./src/components", import.meta.url).pathname,
      },
    },
  },
  integrations: [
    react({
      include: ["**/src/components/**/*.{jsx,tsx}"],
    }),
    starlight({
      title: "hikcamera-rs",
      defaultLocale: "en",
      locales: {
        en: { label: "English", lang: "en" },
        zh: { label: "中文", lang: "zh-CN" },
      },
      customCss: ["./src/styles/hikcamera.css"],
      social: [
        {
          icon: "github",
          label: "GitHub",
          href: "https://github.com/xiaoze-cn/hikcamera-rs",
        },
      ],
      sidebar: [
        {
          label: "Guide",
          translations: { zh: "使用指南" },
          items: [
            { label: "Overview", translations: { zh: "概览" }, link: "/" },
            { slug: "guide" },
            { slug: "guide/installation" },
            { slug: "guide/quick-start" },
            { slug: "guide/sdk-lifecycle" },
            { slug: "guide/device-selection" },
            { slug: "guide/camera-configuration" },
            { slug: "guide/streaming" },
            { slug: "guide/image-and-video" },
          ],
        },
        {
          label: "Developer",
          translations: { zh: "开发者文档" },
          items: [
            { slug: "developer" },
            { slug: "developer/architecture" },
            { slug: "developer/error-model" },
            { slug: "developer/contributing" },
          ],
        },
        {
          label: "Reference",
          translations: { zh: "参考" },
          items: [
            { slug: "reference" },
            { slug: "reference/runtime-dependencies" },
            { slug: "reference/device-info-fields" },
            {
              label: "C SDK reference",
              translations: { zh: "C SDK 参考" },
              collapsed: true,
              items: [
                { slug: "reference/c-sdk/functions" },
                { slug: "reference/c-sdk/structs" },
                { slug: "reference/c-sdk/pixel-types" },
                { slug: "reference/c-sdk/error-codes" },
                { slug: "reference/c-sdk/isp-error-codes" },
                { slug: "reference/c-sdk/obsolete-interfaces" },
                { slug: "reference/c-sdk/obsolete-params" },
              ],
            },
          ],
        },
      ],
    }),
  ],
});
