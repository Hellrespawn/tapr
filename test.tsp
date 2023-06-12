(
    (defn album_and_date
     [tags]
     (if (tags :album)  # if <album>
        (if (tags :date)   # then if <date>
         (string/join-not-nil "" [   # then join date and albumsort and album
            (date/year (tags :date))
            (if (tags :albumsort)
                (string/join "" ["." (number/align 2 (tags :albumsort))]))
            " - "
            (tags :album)])
         (tags :album))))     # else just album
                           # else fall off (nil)

    (defn filename
     [tags]
     (string/join-not-nil " - " [(track_number tags) (if (tags :albumartist) (tags :artist)) (tags :title)]))

    (defn track_number
     [tags]
     (string/join-not-nil "" (
        (if (tags :discnumber) (number/align 1 (tags :discnumber)))
        (if (tags :tracknumber) (number/align 2 (tags :tracknumber)))
     ))
    )

    (defn rename  # top-level function called "rename" must exist
    [tags directory]  # tags is provided by tfmttools, rest of arguments are user-defined
    (string/join-not-nil "/"
        [directory (?? (tags :albumartist) (tags :artist)) (album_and_date tags) (filename tags)]))
)
