#!/bin/sh
set -e

# Substitute only VOLETU_API_BASE_URL in the nginx template.
# Using an explicit variable list preserves nginx's own $uri, $request_uri, etc.
envsubst '${VOLETU_API_BASE_URL}' < /etc/nginx/nginx.conf.template > /etc/nginx/conf.d/default.conf
