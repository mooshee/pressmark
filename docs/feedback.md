# Feedback integration

Author: Daniel Hallman

The website's **Give feedback** link, `/feedback`, `/support`, and `/contact` lead to the feedback form. It accepts bug reports, feature requests, and general feedback without a GitHub account. Email is optional. The form keeps the submitted text when receipt cannot be confirmed and reuses the request ID for an unchanged retry.

`POST /api/v1/feedback` and the MCP tool `submit_feedback` use the same private Hallman Platform service. `POST /api/v1/feedback/status` and `get_feedback_status` check one report using its private receipt. The HTTP and stdio MCP transports expose both tools. Submission does not call document generation, customer sync, checkout, or email delivery.

`PRESSMARK_FEEDBACK_URL` defaults to `http://127.0.0.1:3844`. Only explicit loopback HTTP addresses are accepted. The service must be started and registered for Pressmark before release. An unavailable backend returns an error rather than a receipt. `/api/v1/feedback/health` checks the shared service and its database, and `/api/v1/setup-check` includes feedback readiness.

The page is embedded in the service binary. The Studio's feedback link is inserted into its existing help links when serving the homepage; the release does not replace the Studio's static assets. The existing guarded release operator updates the binary and the privacy page with drift checks and a rollback copy.

Verification covers local web and MCP requests, retry persistence, a wrong receipt or product, unavailable service, source collection, backup recovery, and desktop/mobile form interaction. Live verification reports must be named as internal QA and marked as test records after receipt and status readback. Do not count them as customer feedback.
