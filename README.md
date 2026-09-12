# Formline — Movement Lab

A Next.js App Router app converted from the supplied Formline HTML design. React components provide the live session demo, dashboard, move library, and session report. Fonts are served locally.

## Run locally

Use Node.js 24 LTS and npm.

```sh
npm ci
npm run dev
```

Open http://localhost:3000.

## Verify

```sh
npm run build
npx playwright install chromium
npm test
```

`npm run build` creates the production Next.js app; `npm start` serves it. `npm run build:static` produces a static website in `out/` for GitHub Pages. Browser tests cover navigation, simulated capture, reports, keyboard use, and responsive layouts.

## Application structure

- `app/page.jsx`: homepage
- `app/components/FormlineApp.jsx`: session state and sample data
- `app/components/AppShell.jsx`: navigation and shared layout
- `app/components/*View.jsx`: session, dashboard, library, and report views
- `app/globals.css`: original typography, theme, and responsive styles
- `public/fonts/`: original bundled fonts

Session capture is a simulation using a timer, not camera capture or pose estimation. Dashboard values and coaching cues are sample data. Session state is held in memory and resets on page reload.

## Deployment

`vercel.json` configures a native Next.js deployment. Run `vercel deploy` for a preview. GitHub Pages builds the Next.js export and preserves the existing published blog routes and assets.

The original `index.html` bundle and legacy Rust blog sources remain available as references. They are not used by the Next.js app.
