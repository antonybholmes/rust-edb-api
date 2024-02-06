curl -X POST -d '{"locations":[{"chr":"chr1","start":10,"end":10}], "assembly":"grch38", "level":"gene", "n":10, "tss":[-2000, 1000]}' "localhost:8080/v1/annotation"
