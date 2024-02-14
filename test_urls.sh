
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/register"
curl -v -X POST -H "Content-Type: application/json" -d '{"email":"antony@antonyholmes.dev", "password":"5MvviYPE5jjPmGw8M4g6"}' "localhost:8080/register"





curl -v -X POST -d '{"locations":[{"chr":"chr1","start":10,"end":10}], "level":"gene", "tss":[-2000, 1000]}' "localhost:8080/v1/annotation/grch38?n=5"
curl -v -X POST -d '{"locations":[{"chr":"chr1","start":10,"end":10}], "level":"gene", "tss":[-2000, 1000]}' "localhost:8080/v1/annotation/grch38?n=5&output=text"
