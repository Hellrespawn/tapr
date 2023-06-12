# Grammar

```clojure
(def grammar
  ~{:ws (set " \t\r\f\n\0\v")
    :symchars (+ (range "09" "AZ" "az" "\x80\xFF") (set "!$%&*+-./:<?=>@^_"))
    :token (some :symchars)
    :hex (range "09" "af" "AF")
    :escape (* "\\" (+ (set "ntrzfev0\"\\")
                       (* "x" :hex :hex)
                       (* "u" [4 :hex])
                       (* "U" [6 :hex])
                       (error (constant "bad escape"))))
    :comment (* "#" (any (if-not (+ "\n" -1) 1)))
    :symbol :token
    :keyword (* ":" (any :symchars))
    :constant (* (+ "true" "false" "nil") (not :symchars))
    :string (* "\"" (any (+ :escape (if-not "\"" 1))) "\"")
    :number (cmt (<- :token) ,scan-number)
    :raw-value (+ :comment :constant :number :keyword
                  :string :plist :blist :symbol)
    :value (* (any :ws) :raw-value (any :ws))
    :root (any :value)
    :plist (* "(" :root (+ ")" (error "")))
    :blist (* "[" :root (+ "]" (error "")))
    :main :root})
```
