#!/bin/sh
export OIDC_DISCOVERY_URL
export OIDC_CLIENT_ID
export OIDC_CLIENT_SECRET
export OIDC_AUTH_SCOPE
OIDC_DISCOVERY_URL="$(pass oidc/discovery_url)"
OIDC_CLIENT_ID="$(pass show oidc/client_id)"
OIDC_CLIENT_SECRET="$(pass show oidc/client_secret)"
OIDC_AUTH_SCOPE="openid profile groups email"
"$@"
