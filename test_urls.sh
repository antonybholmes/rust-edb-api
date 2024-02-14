
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/register"
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/login"




curl -v -X POST -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"

curl -v -X POST -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VyX2lkIjoiMDEwMjZGMTktMEUxQS00OEY0LTk4RjgtRUMyOUEwNTlBMjRGIiwiZW1haWwiOiJhbnRvbnlAYW50b255aG9sbWVzLmRldiIsImV4cCI6MTcwODAxODI0MH0.gGE8FyO4jtcXxybGcST07MhGf4LMsGUC-HGlkNgzBFGL5-Y1A2PPhBFsAZ5tcExrm05KIBdx-DxK1RFkdxBttA" -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"
curl -v -X POST -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VyX2lkIjoiMDEwMjZGMTktMEUxQS00OEY0LTk4RjgtRUMyOUEwNTlBMjRGIiwiZW1haWwiOiJhbnRvbnlAYW50b255aG9sbWVzLmRldiIsImV4cCI6MTcwODAxODI0MH0.gGE8FyO4jtcXxybGcST07MhGf4LMsGUC-HGlkNgzBFGL5-Y1A2PPhBFsAZ5tcExrm05KIBdx-DxK1RFkdxBtt" -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"


curl -v -X POST -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjAxMDI2RjE5LTBFMUEtNDhGNC05OEY4LUVDMjlBMDU5QTI0RiIsImVtYWlsIjoiYW50b255QGFudG9ueWhvbG1lcy5kZXYiLCJleHAiOjE3MDgwMzk0Mzd9.mW-XdwcmhBYEK4hwRiW6Fn7Zonw0_0nA7atlMEbRTSw" -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr10","start":1043441,"end":1044114}]}' "localhost:8080/genes/within/grch38"



