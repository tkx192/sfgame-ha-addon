#!/usr/bin/env bashio

bashio::log.info "Starting Shakes & Fidget Reader"

echo "SUPERVISOR: ${SUPERVISOR:-<missing>}"
echo "SUPERVISOR_TOKEN present: $([[ -n "${SUPERVISOR_TOKEN:-}" ]] && echo yes || echo no)"
echo "options.json:"
cat /data/options.json

bashio::log.info "Testing Supervisor API..."

curl -sS \
  -o /tmp/supervisor-response \
  -w "HTTP_STATUS=%{http_code}\n" \
  -H "Authorization: Bearer ${SUPERVISOR_TOKEN}" \
  http://supervisor/addons/self/options/config || true

echo "Response:"
cat /tmp/supervisor-response || true
