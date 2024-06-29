this is a rest backend service written in rust for ClimbApp.

## Testing

curl https://cloud-run-service-7khljjxtqa-vp.a.run.app:8080/hello

You can also run the server locally and test that way as well.

## Local Test Commands

### create-climbing-location

curl -X POST \
     -H "Content-Type: application/json" \
     -d @test_payloads/create_climbing_location.json \
     http://127.0.0.1:8080/create-climbing-location

### hello

curl http://127.0.0.1:8080/hello




## Remote Test Commands

### create-climbing-location

curl -X POST \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $(gcloud auth print-identity-token)" \
     -d @test_payloads/create_climbing_location.json \
     https://beta.climb-service.dev/create-climbing-location

### hello

curl -H "Authorization: Bearer $(gcloud auth print-identity-token)" https://beta.climb-service.dev/hello
