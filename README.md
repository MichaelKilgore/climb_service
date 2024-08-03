this is a rest backend service written in rust for ClimbApp.

## Testing

curl https://cloud-run-service-7khljjxtqa-vp.a.run.app:8080/hello

You can also run the server locally and test that way as well.

## Local Test Commands

### create-climbing-location

./cloud-sql-proxy --port 5432 climbing-app-426701:us-central1:beta-postgres-instance

curl -X POST \
     -H "Content-Type: application/json" \
     -d @test_payloads/create_climbing_location.json \
     http://127.0.0.1:8080/create-climbing-location

### hello

curl http://127.0.0.1:8080/hello

### send-verification-code

./cloud-sql-proxy --port 5432 climbing-app-426701:us-central1:beta-postgres-instance

curl -X POST \
     -H "Content-Type: application/json" \
     -d @test_payloads/send_verification_code.json \
     http://127.0.0.1:8080/send-verification-code

## Remote Test Commands

### create-climbing-location

curl -X POST \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $(gcloud auth print-identity-token)" \
     -d @test_payloads/create_climbing_location.json \
     https://beta.climb-service.dev/create-climbing-location

### hello

curl -H "Authorization: Bearer $(gcloud auth print-identity-token)" https://beta.climb-service.dev/hello

### create-climb-user

curl -X POST -H "Authorization: Bearer $(gcloud auth print-identity-token)" https://beta.climb-service.dev/create-climb-user

## Run Integration Tests Locally

1. ./cloud-sql-proxy --port 5432 climbing-app-426701:us-central1:beta-postgres-instance

2. start server in rust rover

3. cargo test

##  Connecting to sql instance:

1. gcloud beta sql connect  beta-postgres-instance
