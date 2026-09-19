import nextCoreWebVitals from "eslint-config-next/core-web-vitals";

const config = [
  ...nextCoreWebVitals,
  {
    ignores: ["out/**", ".next/**", "node_modules/**", "index.html", "feed/**", "kb/**"],
  },
];

export default config;
