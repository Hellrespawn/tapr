(eval (read-file "stdlib.tsp"))

(var number-list [1 2 3 4 5 6 7 8 9])

(println number-list)

(println (map ++ number-list))

(println (filter is-odd number-list))

(println (reduce 0 (fn [acc n] (+ acc n)) number-list))
