# Frontend: the React app

Status: owner-approved direction (showcase round, 2026-10-22, `docs/decisions.md`); the conventions below are binding for all code in `web/`. Why React and why it is served by the Rust binary: `docs/adr/0005-react-app-served-by-the-rust-binary.md`.

Before writing or reviewing React, read the skills in `.claude/skills/`:

| Skill | Use it for |
|---|---|
| `vercel-react-best-practices` | Performance: waterfalls, bundle size, re-renders. Read `rules/*.md`, not the compiled `AGENTS.md`, which is garbled in places upstream. These don't apply to a Vite app: every `server-*` rule, `async-api-routes`, `async-suspense-boundaries`, `bundle-dynamic-imports` (use `React.lazy`), `bundle-defer-third-party` and the `rendering-hydration-*` and `rendering-script-defer-async` rules. Where a rule uses SWR, read it as TanStack Query |
| `vercel-composition-patterns` | Component design: compound components, explicit variants, no boolean-prop piles, React 19 APIs. Its examples use React Native components; the patterns are the same on the web |
| `shadcn` | Adding, composing and styling shadcn/ui components. Skip its chat, registry, MCP and design-system files |

Where a skill and this file disagree, this file wins, because it knows Balanse's rules.

### Where the skills come from

Installed in Kompleto on 2026-10-22 and copied unchanged into Balanse on 2026-10-24, with the same entries in `skills-lock.json` (the hash covers each folder as installed, local changes included). All three were read in full before being committed; they hold guidance only, apart from the shadcn changes below.

| Skill | Source | Commit | Licence |
|---|---|---|---|
| `vercel-react-best-practices` | github.com/vercel-labs/agent-skills, `skills/react-best-practices` | `063bee9` (2026-08-28) | MIT, declared in its `SKILL.md`; the repository has no licence file or copyright line |
| `vercel-composition-patterns` | the same repository, `skills/composition-patterns` | `063bee9` | MIT, as above |
| `shadcn` | github.com/shadcn-ui/ui, `skills/shadcn` | `25af02b` (2026-10-22) | MIT, Copyright (c) 2023 shadcn; the licence is copied into the folder as `LICENSE.md` |

**Local changes to `shadcn/SKILL.md`** (re-apply them after any update):
- removed `allowed-tools`, which pre-approved every `shadcn` command with any arguments, including adding code from any web address;
- removed the line that ran `npx shadcn@latest info` automatically, unpinned, each time the skill loaded. It now says to run the pinned `pnpm dlx shadcn@4.22.1 … -c web` by hand.

**Adding components while `ui.shadcn.com` is unreachable.** This build environment's network policy blocks `ui.shadcn.com`, which `shadcn init` and `shadcn add` need. Until the host is allowed, components come from a checkout of github.com/shadcn-ui/ui through `web/scripts/vendor-shadcn.mjs`, which does what `shadcn add` does for the Base UI "nova" style: it turns each `cn-*` class into the Tailwind classes `style-nova.css` gives it, icon placeholders into lucide-react icons, and registry imports into our aliases. The components in `web/src/components/ui/` came from shadcn-ui/ui commit `25af02b`. `web/src/styles/shadcn.css` is the shadcn package's `tailwind.css` (version 4.22.1), copied unchanged. `use-mobile.ts` was rewritten to avoid setting state in an effect.

Vendored files (`components/ui/`, `shadcn.css`) and `pnpm-lock.yaml` don't count towards the ~150-line commit size, as `.sqlx/` and `Cargo.lock` don't.

**Rules for the shadcn command line in this project:**
- always the pinned version in the Stack table below, never `@latest`;
- only the official shadcn registry; any other registry or web address needs the owner's yes;
- never `--overwrite` or `init --force` without the owner's yes; preview with `--dry-run` and `--diff` first;
- read every file it adds before committing it.

## Principles

1. **Rust decides; React shows.** Every rule that protects an acceptance criterion lives in the Rust API: approval, sending, isolation, pre-send checks, schedules, the frequency cap, quiet hours. The React app never decides who gets a message, whether a Draft is approved or which Business owns a record. It shows what the API returns and asks the API to act.
2. **The API is the contract.** TypeScript types for API requests and responses are generated from the Rust types, never written by hand. If the Rust type changes, the TypeScript build fails until the app is updated.
3. **Untrusted text stays text.** Debtor names, Draft bodies, Replies, notes and file names may contain anything. Render them as React text, never with `dangerouslySetInnerHTML`, and never build a link from them.
4. **Use the glossary's words** (`GLOSSARY.md`) in code, screens and messages: Business, Owner, Staff, Debtor, Contact, Debt, Claim, Dispute, Promise, Draft, Reminder, Reply, Debtor page, Evidence log.
5. **Phone first.** Every screen works at 360 px wide and by keyboard. An Owner should be able to approve a batch of Drafts on a phone.
6. **Small and boring.** Few dependencies, no global store, no clever abstractions. Copy a pattern twice before extracting it a third time.
7. **Dates are shown, not computed.** The API sends dates and times; the app formats them in Manila time. It never works out a Cut-off, a deadline or "days left" on its own.

## Stack

| Need | Choice |
|---|---|
| Language | TypeScript, `strict`, no `any` |
| Build | Vite, React 19.2 or later (the skills use `useEffectEvent` and `<Activity>`) |
| Package manager | pnpm, exact versions, `pnpm-lock.yaml` committed |
| Components | shadcn/ui on **Base UI** (shadcn's default for new projects; `base` in `components.json`), copied into `web/src/components/ui/`; Tailwind CSS v4; lucide-react icons |
| shadcn command line | `pnpm dlx shadcn@4.22.1 … -c web` (pinned) |
| Routing | React Router (client-side) |
| Server data | TanStack Query |
| Forms | shadcn's `Field` components, with react-hook-form and zod |
| API types | generated from Rust with `ts-rs` into `web/src/api/generated/` |
| Unit tests | Vitest and Testing Library |
| Browser tests | Playwright, against the real Rust server and Postgres |
| Lint and format | ESLint (typescript-eslint, react-hooks) and Prettier |

Versions are pinned when `web/` is created, and a dependency is added only with a reason in its commit message. Licences must be MIT, Apache-2.0, BSD or ISC.

## Folder layout

```
balanse/
├── src/                        Rust (the engine is the nudgewright crate, re-exported as crate::engine)
│   └── balanse/
│       ├── web/
│       │   ├── api/            JSON endpoints under /api/, one file per area
│       │   ├── debtor_page.rs  the Debtor page (server-rendered)
│       │   ├── hooks.rs        /hooks/ for inbound email and delivery events
│       │   └── spa.rs          serves the built React app
│       └── …                   domain modules (debtors, debts, drafts, schedule, …)
├── templates/                  only the Debtor page and its "not found" page
├── tests/                      Rust tests, including every acceptance criterion
├── web/                        the React app
│   ├── index.html
│   ├── package.json
│   ├── pnpm-lock.yaml
│   ├── vite.config.ts
│   ├── tsconfig.json
│   ├── components.json         shadcn settings
│   ├── eslint.config.js
│   ├── e2e/                    Playwright tests of the main journeys
│   ├── scripts/                vendor-shadcn.mjs
│   └── src/
│       ├── main.tsx            entry: providers and router
│       ├── app/                router, app shell (navigation), error and 404 pages
│       ├── api/
│       │   ├── client.ts       the one fetch wrapper
│       │   └── generated/      types made by ts-rs; never edited by hand
│       ├── features/           one folder per area of the product
│       │   ├── dashboard/
│       │   ├── debtors/
│       │   ├── drafts/
│       │   ├── import/
│       │   ├── activity/
│       │   ├── settings/
│       │   ├── login/
│       │   ├── help/           help page; its text is help.md
│       │   └── legal/          privacy notice, terms, data processing agreement (.md + pages)
│       ├── components/
│       │   ├── ui/             shadcn components
│       │   └── …               shared app components (page-header, empty-state, status-badge)
│       ├── lib/                small helpers: utils.ts (cn), dates.ts (Manila formatting),
│       │                       markdown.tsx (renders our own .md documents as React elements)
│       └── styles/
│           └── globals.css     Tailwind and the theme's CSS variables
└── …
```

Inside a feature folder:

```
features/drafts/
├── drafts-page.tsx             the route's component
├── draft-card.tsx              components used only by this feature
├── review-batch-dialog.tsx
├── queries.ts                  this feature's TanStack Query hooks and query keys
└── drafts-page.test.tsx
```

**Import rules:**
- A feature imports from `components/`, `lib/` and `api/`. From another feature it may import only that feature's `queries.ts` (its hooks and query keys, e.g. refreshing the dashboard after approving a Draft), never its components. A component two features need moves to `components/`.
- No barrel files (`index.ts` re-exports); import from the file itself.
- Use the `@/` alias for `web/src/` (`@/components/ui/button`), never long `../../` paths.

## Coding conventions

### Naming

| Thing | Convention | Example |
|---|---|---|
| Files and folders | kebab-case | `draft-card.tsx`, `use-copy-text.ts` |
| Components | PascalCase, named export | `export function DraftCard()` |
| Hooks | `use` + camelCase | `useDrafts`, `useApproveBatch` |
| Event props / handlers | `onX` / `handleX` | `onApprove`, `handleApprove` |
| Booleans | `is`, `has`, `can` | `isExpired`, `hasReplied` |
| Types | PascalCase, no `I` prefix; `Props` for a component's own props | `type DraftCardProps` |
| Query keys | arrays from a `keys` object in `queries.ts` | `draftKeys.list()` |

Default exports only where a tool requires them (Vite config, lazy-loaded routes if needed).

### Components

- One component per file, unless a small helper is used only by it.
- Function components and hooks only.
- Props are typed; children are passed as `children` rather than render props where possible.
- No boolean props that switch layouts (`isCompact`, `isDialog`): make explicit variants or compound components (`vercel-composition-patterns`).
- Derive values during render instead of copying props into state or syncing them with `useEffect`. An effect is for talking to something outside React, and rarely needed here.
- Don't define components inside other components.
- React 19: pass `ref` as an ordinary prop (no `forwardRef`); read context with `use(Context)`.
- Every list item has a stable `key` from the data's ID, never the array index.

### Data and state

- **Server data lives in TanStack Query**, nowhere else. Each feature's `queries.ts` holds its query keys and hooks; components call the hooks, never `fetch`.
- **All requests go through `api/client.ts`**, which sends the session cookie, sends JSON, turns errors into one typed error, and on `401` sends the user to the login page.
- After a change (approve, edit, mark received), invalidate the affected queries rather than editing the cache by hand, unless the screen needs an instant update.
- **Screen state** (which dialog is open, which Drafts are ticked) uses `useState` in the nearest component that needs it. Lift it only as far as needed. No Redux, Zustand or other global store.
- **Filters and tabs live in the URL** (search parameters), so a page can be reloaded or bookmarked.
- Nothing is stored in `localStorage`, except harmless display preferences; those keys are versioned and read inside `try`/`catch`.

### Forms

- shadcn's `FieldGroup`, `Field`, `FieldLabel` and `FieldError`, with `FieldSet` and `FieldLegend` for groups such as "Forms filed". An invalid field gets `data-invalid` on the `Field` and `aria-invalid` on its control (`shadcn` skill, `rules/forms.md`).
- react-hook-form and a zod schema per form.
- The browser's checks are for convenience; the API's checks are final. Show the API's per-field errors next to the field, in the API's words.
- Disable the submit button while a request is running, so a double click can't send twice. The API is still the real guard (approval binds content; sends are idempotent).

### Styling

- Semantic colour tokens only (`bg-primary`, `text-muted-foreground`, `border-destructive`): no raw colours such as `bg-blue-500`, no `dark:` overrides, no inline `style`, no CSS file per component.
- The theme is CSS variables in `:root` and `.dark` inside `styles/globals.css`, registered with Tailwind's `@theme inline`. That is the only CSS file.
- `className` on a shadcn component is for layout (margins, width, grid placement). To change how it looks, use its variants; add a new `cva` variant in the component if none fits.
- Spacing with `gap-*`, not `space-*`; sizes with `size-*` when width equals height.
- Combine classes with `cn()` from `lib/utils.ts`.
- shadcn components in `components/ui/` are ours to edit, but keep changes small and note why in a comment, so a later `shadcn` update stays easy.
- Icons from lucide-react only, passed as components, with `data-icon="inline-start"` or `"inline-end"` inside buttons and no sizing classes. Icon-only buttons need an `aria-label`.
- Prefer shadcn's own pieces over custom markup: `Alert` for messages, `Empty` for empty states, `Skeleton` while loading, `Badge` for statuses, `Spinner` with a disabled button while saving, `AlertDialog` to confirm anything destructive (closing the account, discarding a Draft). Every `Dialog` and `Sheet` has a title.

### Accessibility

- Every input has a visible label. Errors are announced (`aria-describedby`, `role="alert"`).
- Everything works by keyboard, with a visible focus ring.
- Touch targets are at least 44 px high on phones.
- Colour is never the only signal: "Missing" has a word or icon, not just red.
- Text contrast meets WCAG AA.

### Performance

- Each route is loaded lazily, so the first screen downloads only what it needs.
- Start independent requests together, not one after another (`vercel-react-best-practices`, async rules).
- Long lists (the activity log) are paginated by the API, not loaded all at once.

### Security

- The session is an `HttpOnly`, `SameSite` cookie set by the API. The app never sees or stores a token.
- Every changing request is JSON from the same origin; the API refuses anything else, as the pages do now.
- The Content Security Policy stays strict: scripts only from our own origin, no inline scripts, nothing from a CDN, no third-party fonts or analytics.
- Links shown to the Owner (Debtor page links, "copy for Viber/Messenger", "Open WhatsApp") come from the API as data; the app never builds one.

### Copy

- English on the Owner's screens, in short plain sentences.
- Say what happened and what to do next: "3 Drafts approved. They'll go out within a minute," not "Success."
- Empty states explain the next step: "No Debtors yet. Add a Debt or import a spreadsheet."

## Tests

- **Every acceptance criterion keeps its Rust test**, now against the JSON API, named with its ID as before (e.g. `a2_another_firms_ids_return_not_found`).
- **Browser tests (Playwright)** cover the main journeys end to end: log in, add and import Debts, approve a batch of Drafts, see a Claim arrive, read a Reply, export a Debtor's Evidence log. A test that proves a criterion in the browser starts its name with the ID, e.g. `d3 batch review shows every draft before approving`.
- **Unit tests (Vitest)** cover components with logic, such as the batch-review selection, not static layout.
- Test what the user sees: find elements by role and label, not by CSS class.

## Running it

- **With the Rust server:** `pnpm --dir web build`, then `cargo run -- serve` serves the app from `web/dist` (`WEB_DIR`) at http://127.0.0.1:3000, as in production.
- **While working on it:** `pnpm --dir web dev` serves it at http://localhost:5173 with instant reloads, passing `/api`, `/u/`, `/static/`, `/hooks` and `/health` to the Rust server. Set `PUBLIC_BASE_URL=http://localhost:5173` in `.env` so login links open there and the API accepts changes from it.
- **Browser tests:** `pnpm --dir web e2e` runs `web/e2e/` against a running server, logging in once through Mailpit for a new Business per run. Set `CHROMIUM_PATH` to use an installed Chromium.

## Checks before every commit

Alongside the Rust checks in `AGENTS.md`:

```
pnpm --dir web lint
pnpm --dir web typecheck
pnpm --dir web test
pnpm --dir web build
```

CI runs the same checks and the Playwright tests.
