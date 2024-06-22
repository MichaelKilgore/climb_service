this is a rest backend service written in rust for ClimbApp.

## Testing

curl https://cloud-run-service-7khljjxtqa-vp.a.run.app:8080/hello

You can also run the server locally and test that way as well.


curl -X POST \
     -H "Content-Type: application/json" \
     -d @test_payloads/create_climbing_location.json \
     http://127.0.0.1:8080/create-climbing-location

curl http://127.0.0.1:8080/hello
