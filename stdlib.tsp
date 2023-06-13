(defn is-even [n] (== 0 (% n 2)))
(defn is-odd  [n] (!(is-even n)))
(defn increment [n] (+ n 1))
(defn decrement [n] (- n 1))

# Get highest number from list of values
(defn max [values]
    # Repeatedly apply function to list to produce single value
    (reduce
        # Initial value, first element of list
        (head values)
        # Anonymous function, applied to every element of list
        (fn
            # Two parameters
            [lhs rhs]
            # if (lhs rhs) is monotonically ascending
            (if (> lhs rhs)
                rhs  # then
                lhs  # else
            )
        )
        # Input list, all elements after the first.
        (tail values)
    )
)

# Get lowest number from list of values
(defn min [values]
    # Repeatedly apply function to list to produce single value
    (reduce
        # Initial value, first element of list
        (head values)
        # Anonymous function, applied to every element of list
        (fn
            # Two parameters
            [lhs rhs]
            # if (lhs rhs) is monotonically descending
            (if (< lhs rhs)
                rhs  # then
                lhs  # else
            )
        )
        # Input list, all elements after the first.
        (tail values)
    )
)

# Filter values in list based on function, expressed in terms of `reduce`.
(defn filter-local [function values]
    (reduce
        # Initial value, empty list
        [list]
        # Anonymous function
        (fn [acc val] (
            # Concat `acc` and `val` if `function` returns true
            if (function val) (concat acc val) acc)
        )
        values
    )
)

# Apply a function to every element of list, expressed in terms of `reduce`.
(defn map-local [function values]
    (reduce
        # Initial value, empty list
        [list]
        # Anonymous function
        (fn [acc val]
            # Concatenate acc and transformed value.
            (concat acc (function val))
        )
        values
    )
)
