# Formline engineering instructions

These instructions apply to the Formline calisthenics product. Carry authorized work from implementation through verification and deployment evidence. Preserve unrelated user changes.

## Product and architecture constraints

- Read `README.md`, `DEPLOY.md`, the relevant component files, and the linked Obsidian product notes before changing behavior.
- Formline currently uses simulated movement capture. Keep `SIMULATED`, `DEMO`, and equivalent labels honest until real camera and pose-estimation inputs exist.
- Product context lives in the Obsidian vault under `01 Projects/Formline`. After changing roadmap, principles, or feedback notes, run `npm run brain:sync` and review `app/data/product-brain.json` before building.
- Do not make the deployed application depend on the local Obsidian filesystem. The committed product-brain snapshot is the build-time boundary.
- Keep coaching advice actionable, specific about the body and movement moment, and clear about measured versus simulated data.

## Full software development lifecycle

1. **Discover:** state the user problem, affected flow, acceptance criteria, risks, and evidence needed. Use `planning-and-task-breakdown`, `source-driven-development`, and `typesafe-ai` when a semantic judgment or ranking helps.
2. **Design:** preserve the existing Next.js App Router structure and responsive behavior. Use `frontend-ui-engineering`, `web-design-guidelines`, `make-interfaces-feel-better`, `impeccable`, `modern-web-guidance`, `api-and-interface-design`, `performance-optimization`, and `security-basics` when relevant. Define the user goal, information hierarchy, states, keyboard path, responsive layout, contrast, focus treatment, and empty/error/loading behavior before polishing visuals. Record durable product decisions in `01 Projects/Formline/Formline Decision Log.md`.
3. **Build:** make the smallest coherent change. Use `test-driven-development` for meaningful behavior, `code-simplification` for needless complexity, and `vercel-react-best-practices` for React or Next.js performance work.
4. **Verify locally:** run the narrowest affected checks, then the complete gate before handoff:

   ```sh
   npm run brain:sync
   npm run lint
   npm run format:check
   npm run build
   npm test
   ```

   Use `browser-testing-with-devtools` for real browser behavior and `verification-before-completion` before claiming success.

5. **Secure:** threat-model new flows with STRIDE. Use `security-and-hardening`, `secret-scan-triage`, `codeql`, `semgrep`, `supply-chain-risk-auditor`, and `dependency`/`npm audit` checks when the change touches authentication, user data, dependencies, build scripts, or deployment. Never commit API keys, tokens, camera data, or personal data.
6. **Integrate:** use `git-workflow-and-versioning`, review the diff, and keep commits reversible. Do not rewrite history or delete data without explicit authorization.
7. **CI/CD:** `.github/workflows/deploy.yml` is the source of truth. `quality` and `build` run in parallel; lint, formatting, Next.js build, Playwright, static export, and the artifact-size gate are blocking checks. Use `ci-cd-and-automation`, `github-actions-local-repro`, and `github-actions-failure-triage` when changing or diagnosing the workflow.
8. **Deploy:** push only the intended commit to `main` after local gates pass. Confirm the GitHub Actions run reaches success with `gh run list`/`gh run watch`. Use `deploy-to-vercel` or `vercel-cli-with-tokens` only when the target is Vercel; use the Pages workflow for the GitHub Pages target. Never call a deployment complete from a queued or partial run.
9. **Observe and learn:** verify the live URL and relevant user flow after deployment. Record failures, decisions, and next product experiments in the Obsidian Formline notes. Use `observability-and-instrumentation`, `observability-metrics`, and `debugging-and-error-recovery` for production issues.
10. **Recover:** prefer a small revert or forward fix. Use `finishing-a-development-branch` after verification, and document rollback evidence when a deployment is unhealthy.

## Netflix-inspired engineering practices

Use these as practices, adapted to this small product; do not copy Netflix scale assumptions:

- **Full ownership:** the change owner carries code, tests, deployment, observability, and rollback together.
- **Paved road:** use the existing npm scripts and GitHub Actions gates before adding new tooling.
- **Small, reversible delivery:** ship narrow changes with a clear rollback path.
- **Safe rollout:** prefer preview or static validation, then production deployment after all blocking gates pass.
- **Failure injection in tests:** exercise failed capture, missing data, build failures, and dependency failures without affecting real users. Use `browser-testing-with-devtools` and `integration-testing-http` where applicable.
- **Visibility:** expose useful health and build signals; do not invent telemetry or report simulated metrics as production measurements.
- **Least privilege:** keep GitHub Actions permissions minimal and keep credentials server-side.
- **Evidence before confidence:** compare expected behavior, automated checks, and live behavior; investigate disagreements instead of hiding them.

Reference practices: [Netflix TechBlog](https://netflixtechblog.com/) and [Netflix Simian Army](https://netflix.github.io/chaosmonkey/).

## Skill routing

Use the smallest applicable skill set. Do not invoke every skill for every change.

| Work                         | Skills                                                                                                                                     |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Product discovery and design | `planning-and-task-breakdown`, `source-driven-development`, `frontend-ui-engineering`, `typesafe-ai`                                       |
| UX and UI design             | `frontend-ui-engineering`, `web-design-guidelines`, `make-interfaces-feel-better`, `impeccable`, `modern-web-guidance`, `diagramming-code` |
| React, Next.js, performance  | `vercel-react-best-practices`, `performance-optimization`, `web-design-guidelines`                                                         |
| Tests and browser behavior   | `test-driven-development`, `browser-testing-with-devtools`, `integration-testing-http`, `verification-before-completion`                   |
| CI/CD and release            | `ci-cd-and-automation`, `github-actions-local-repro`, `github-actions-failure-triage`, `finishing-a-development-branch`                    |
| Deployment                   | `deploy-to-vercel`, `vercel-cli-with-tokens`, `shipping-and-launch`                                                                        |
| Security and supply chain    | `security-basics`, `security-and-hardening`, `secret-scan-triage`, `codeql`, `semgrep`, `supply-chain-risk-auditor`                        |
| Reliability and operations   | `observability-and-instrumentation`, `observability-metrics`, `debugging-and-error-recovery`, `systematic-debugging`                       |

## Completion standard

Report what changed, why, exact checks run, CI/deployment status, live verification, and any remaining limitation. For UI work, verify mobile and desktop paths. For product-brain work, show the snapshot date and source note changes. Never claim camera capture, autonomous repair, security coverage, or deployment health without evidence.

Review this file quarterly or after a material change to the stack, deployment workflow, or product-brain process.
