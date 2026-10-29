# Temporary PDF downloads

Author: Daniel Hallman

HTTP calls to `generate_document`, `convert_document`, and `record_payment` can return a temporary PDF download URL. The result includes a Markdown link and matching `download` metadata in `structuredContent`. Clients using MCP 2025-06-18 or later also receive a `resource_link`. Older clients receive the text link and embedded PDF. The embedded PDF and `pdf_base64` remain available. Standard input/output MCP calls return the embedded PDF without a hosted link.

Anyone with the download URL can retrieve the PDF. Each URL contains 244 random bits and expires after 15 minutes. A process restart also removes access. The filename contains only ASCII letters, digits, hyphens, underscores, and the `.pdf` suffix.

The process keeps these download copies in memory, with limits of 8 MiB per PDF, 64 MiB total, and 64 PDFs. Expired entries are removed on cache access and by a cleanup task every 30 seconds. A full cache rejects new download copies without removing valid links. The tool then returns `download_error` alongside the embedded PDF.

Download responses use `application/pdf` and `Content-Disposition: attachment`. Successful and missing-link responses carry private/no-store cache headers, `noindex, nofollow, noarchive`, `no-referrer`, and `nosniff`. The route has no document listing.

## Deploy and verify

1. Build and deploy the reviewed commit. Keep the existing operator credentials and database. The change needs no database migration or new secret.
2. Set `PRESSMARK_BASE_URL` to the public HTTPS origin. For local checks, an HTTP loopback origin is accepted. Run one service instance, or ensure that a generated download returns to the same instance; the cache is local to its process.
3. Keep the startup call to `downloads.start_cleanup_task()`. Check that the reverse proxy passes the download headers through, does not cache `/api/v1/downloads/`, and omits or redacts the capability token from access logs. Avoid recording full generated URLs in diagnostic receipts.
4. Generate a sample PDF through HTTP MCP. Open the returned link and check the response status, download filename, PDF content, and private headers. Verify that a missing token returns a generic `404` response.
5. In a fresh ChatGPT conversation, generate a sample document through the connected app and open its returned PDF link. Confirm that the document downloads and matches the requested details. An HTTP test alone does not prove that ChatGPT exposes a usable link.
6. Verify expiry after 15 minutes. In a separate local check, restart the service and confirm that an earlier link no longer works.

Reverting the binary removes this route and all process-local download copies. There is no download data to migrate or delete from the account database.

## Automated checks

Run `cargo nextest run`. `downloads::tests` checks expiry, size and count limits, invalid tokens, filename handling, and allowed origins. `test_pdf_downloads` checks all three PDF tools, exact byte identity, GET and HEAD responses, restart behavior, private headers, missing links, and the robots exclusion.
