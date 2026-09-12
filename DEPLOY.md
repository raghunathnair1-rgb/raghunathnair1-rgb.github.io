# Deployment

## Vercel

The project is linked locally to `raghu-adf1/calisthenics`. Vercel uses the Next.js framework, `npm run build`, and `.next` output configured in `vercel.json`.

```sh
vercel deploy --target preview --scope raghu-adf1
vercel inspect <deployment-url> --scope raghu-adf1 --wait
```

Use `--prod` only when publishing to the production domain is intended. Automatic deployments from GitHub require connecting the GitHub account and repository in Vercel project settings.

## GitHub Pages

The workflow in `.github/workflows/deploy.yml` runs on pushes to `main`. It retains the legacy Rust coverage gate, builds and browser-tests Next.js, exports the app with `npm run build:static`, and uploads `out/` to Pages. Existing blog routes and assets are copied into the export before upload.

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
