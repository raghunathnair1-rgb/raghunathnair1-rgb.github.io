import fs from "node:fs/promises";
import path from "node:path";

const vault = process.env.FORMLINE_VAULT ?? "/Users/raghunathnair/Documents/Obsidian Vault";
const projectDir = path.join(vault, "01 Projects", "Formline");
const knowledgeFile = path.join(
  vault,
  "02 Knowledge",
  "Formline",
  "Movement Coaching Principles.md",
);
const roadmapFile = path.join(projectDir, "Formline Roadmap.md");
const feedbackFile = path.join(projectDir, "Formline Feedback Inbox.md");
const output = path.join(process.cwd(), "app", "data", "product-brain.json");

const read = (file) => fs.readFile(file, "utf8");
const bullets = (text, heading) => {
  const section = text.split(`## ${heading}`)[1]?.split("\n## ")[0] ?? "";
  return [...section.matchAll(/^[-*] (?:\[[ x]\] )?(.+)$/gm)].map((m) => m[1].trim());
};
const first = (items, fallback) => items[0] ?? fallback;

const [roadmap, principles, feedback] = await Promise.all([
  read(roadmapFile),
  read(knowledgeFile),
  read(feedbackFile),
]);

const snapshot = {
  source: "Obsidian Vault/01 Projects/Formline",
  updated: new Date().toISOString().slice(0, 10),
  focus: first(bullets(roadmap, "Now"), "Keep the next training focus visible."),
  principles: [...principles.matchAll(/^\d+\.\s+(.+)$/gm)]
    .map((match) => match[1].trim())
    .slice(0, 3),
  next: first(
    [...feedback.matchAll(/^[-*] \[ \] (.+)$/gm)].map((match) => match[1].trim()),
    "Collect one athlete observation.",
  ),
};

await fs.mkdir(path.dirname(output), { recursive: true });
await fs.writeFile(output, `${JSON.stringify(snapshot, null, 2)}\n`);
console.log(`Synced product brain from ${projectDir}`);
