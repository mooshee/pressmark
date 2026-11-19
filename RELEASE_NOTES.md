# Pressmark 1.4.0

Author: Daniel Hallman

November 19, 2026

Pressmark 1.4.0 adds public template, pricing, documentation, and calculator pages, shareable recipient links, and account and webhook validation:

- **Public catalog, pricing, docs, and calculators:** Added server-rendered pages for `/templates`, `/templates/:slug` (57 archetypes and 51 jurisdictions), `/pricing`, `/docs`, `/calculators`, and 4 statutory calculators (`/calculators/security-deposit-return`, `/calculators/rent-increase-notice`, `/calculators/commercial-nnn-lease`, `/calculators/late-fee-invoice`) with direct PDF export and sitemap indexing (`266` template URLs).
- **Shareable recipient links:** Added 30-day expiring share links (`/v/:token`, `/v/:token/pdf`, `/v/:token/preview.png`, `/v/:token/sign`) so senders can send a preview and capture a recipient signature without manual PDF attachments.
- **Auth, credit, and webhook hardening:** Added SHA-256 `api_keys` validation, one-time `$2.99` (1 credit) and `$9.99` (5 credits) export-credit fulfillment, unentitled free-tier email defaults, production OAuth demo gating, fail-closed Stripe customer identity checks, and `503`/`422` errors when OCR or copilot parsing is unavailable or unmatched.
- **Mobile Studio, CRM hygiene, and faster page load:** Replaced the `649 KB` header JPEG with `948 B` `icon.svg`, cleared default placeholder payment links and Studio sample-email CRM syncs, and added mobile preview/edit switching controls.

[Open Pressmark](https://pressmark.dev) · [Template catalog](https://pressmark.dev/templates) · [Installation instructions](https://github.com/mooshee/pressmark#use-pressmark-with-an-agent)

---

# Pressmark 1.3.0

Author: Daniel Hallman

November 15, 2026

Pressmark no longer shows its plans or starts checkout in agent conversations. You can still create and preview documents, calculate totals, convert estimates, and prepare receipts from payment details you provide. Documents can include a payment link you supply. Creating a receipt does not charge anyone or verify a transaction.

The connection now lists seven tools, including tools to send requested feedback and check its status. The tool permission descriptions also identify document creation and customer-record changes more accurately. When customer sync is enabled, PDF generation can update contact details and start follow-up workflows, as described in the [privacy policy](https://pressmark.dev/privacy).

Guest connections require no credentials. This release also corrects connection checks that could prompt guest clients to sign in.

## Update your installation

Update or reinstall your Pressmark package, then refresh its tool list. The server address remains `https://pressmark.dev/mcp`. If an older instruction calls `get_pricing`, use the [pricing page](https://pressmark.dev/pricing) in a browser instead. The pricing tool and resource are no longer part of the agent connection.

See the [installation instructions](https://github.com/mooshee/pressmark#use-pressmark-with-an-agent) and [published downloads](https://github.com/mooshee/pressmark/releases). Each directory or app review may make the update available at a different time.

Built by [Moosh Works](https://mooshworks.com).
