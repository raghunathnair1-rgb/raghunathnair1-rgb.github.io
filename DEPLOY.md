# Deployment

## Vercel

The project is linked locally to `raghu-adf1/calisthenics`. Vercel uses the Next.js framework, `npm run build`, and `.next` output configured in `vercel.json`.

```sh
vercel deploy --target preview --scope raghu-adf1
vercel inspect <deployment-url> --scope raghu-adf1 --wait
```

Use `--prod` only when publishing to the production domain is intended. Automatic deployments from GitHub require connecting the GitHub account and repository in Vercel project settings.

## GitHub Pages

The workflow in `.github/workflows/deploy.yml` runs on pushes to `main` with two parallel jobs:

- **quality** — `npm ci`, then ESLint (`npm run lint`) and Prettier (`npm run format:check`) as blocking gates.
- **build** — `npm ci`, Playwright chromium (cached across runs), `npm run build`, Playwright browser tests (`npm test`) as a blocking gate, `npm run build:static`, copies existing blog routes and assets into the export, enforces a 15 MB artifact size gate (baseline ~11 MB, mostly the `kb/` knowledge base), and uploads `out/` to Pages.

The `deploy` job runs only when both pass. The legacy Rust coverage gate was removed: the `blog-logic` crate has no tests, so the 100% floor was vacuous.

Enable **Settings → Pages → Build and deployment → Source: GitHub Actions** in the repository.

## Local production check

```sh
npm ci
npm run build
npx playwright install chromium
npm test
npm start
```

The old `deploy.sh` belongs to the legacy VPS blog automation. Use the commands above for this Next.js app.
