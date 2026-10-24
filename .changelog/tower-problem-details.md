---
mpp: minor
---

Add opt-in RFC 9457 JSON bodies to `PaymentLayer` and `PaymentBodyLayer` with
`with_problem_details`. Applications supply a JSON-bytes-to-body callback while
existing layers retain empty response bodies by default. Payment rejections keep
their status and retry challenge; internal failures return a generic 500 problem
without exposing backend details.

Generated request-body `400 Bad Request` and `413 Payload Too Large` responses
now include `Cache-Control: no-store` even when Problem Details is not enabled.
