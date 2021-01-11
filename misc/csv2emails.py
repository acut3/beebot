#!/usr/bin/python3

import fileinput

for line in fileinput.input():
    l = line.rstrip()
    (firstname, lastname, email) = l.split(";")
    # Skip invalid lines, includind header line
    if "@" not in email:
        continue
    firstname = firstname.title()
    lastname = lastname.title()
    email = email.lower()
    print(email)
