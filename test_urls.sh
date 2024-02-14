
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/register"
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/login"




curl -v -X POST -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"

curl -v -X POST -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VyX2lkIjoiMDEwMjZGMTktMEUxQS00OEY0LTk4RjgtRUMyOUEwNTlBMjRGIiwiZW1haWwiOiJhbnRvbnlAYW50b255aG9sbWVzLmRldiIsImV4cCI6MTcwODAxODI0MH0.gGE8FyO4jtcXxybGcST07MhGf4LMsGUC-HGlkNgzBFGL5-Y1A2PPhBFsAZ5tcExrm05KIBdx-DxK1RFkdxBttA" -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"
curl -v -X POST -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJ1c2VyX2lkIjoiMDEwMjZGMTktMEUxQS00OEY0LTk4RjgtRUMyOUEwNTlBMjRGIiwiZW1haWwiOiJhbnRvbnlAYW50b255aG9sbWVzLmRldiIsImV4cCI6MTcwODAxODI0MH0.gGE8FyO4jtcXxybGcST07MhGf4LMsGUC-HGlkNgzBFGL5-Y1A2PPhBFsAZ5tcExrm05KIBdx-DxK1RFkdxBtt" -H "Content-Type: application/json" -d '{"locations":[{"chr":"chr1","start":100000,"end":100100}]}' "localhost:8080/dna/grch38?n=5"
