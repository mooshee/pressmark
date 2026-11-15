# Pressmark release packages

Author: Daniel Hallman

Build the release archives from the reviewed manifests and skills in the repository:

```console
./publishing/build_bundles.sh
```

This requires Rust, Cargo, and `agent-publish`. The generator's output under `publishing/generated` is a draft and is not a release artifact. It may include placeholder review fields and OAuth discovery metadata that does not apply to Pressmark's guest connection.

Use the two archives in `dist`, with `server.json`, `RELEASE_NOTES.md`, and `SHA256.json`. The final packager checks version parity, the seven-tool submission inventory, draft markers, common credential markers, and local paths. It copies only named public manifests, reviewed documentation, seven public image assets, and Markdown skills. It rejects symlinks and unexpected skill files. Runtime source, configuration, databases, build output, and private release records are not included.

Archive paths, timestamps, permissions, and file order are fixed. Each saved archive is reopened and compared byte-for-byte with the selected inputs. `SHA256.json` records the final archive and file hashes. These checks do not replace review of the live service, host installation, or a pending app submission.

Run the packaging tests separately:

```console
cargo nextest run --locked --manifest-path tools/pressmark-package/Cargo.toml
```
