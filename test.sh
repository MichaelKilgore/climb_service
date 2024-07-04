#!/bin/bash


curl -X POST \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $(gcloud auth print-identity-token)" \
     -d @test_payloads/create_climbing_location.json \
     https://beta.climb-service.dev/create-climbing-location
