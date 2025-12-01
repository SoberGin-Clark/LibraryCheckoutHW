
# Description
A small library made in python that allows a program refrencing it to create both catalogs of items, either dvds or books, 
as well as members who may check out and return said items.

# Library
items.py
    Contains the functionality of the items themselves. Created modularly so as to make adding additional item types easier.
catalog.py
    Contains the functionality for creating catalogs made of items. Duplicate IDs are not accepted, 
    as well as requests for info on IDs not in the catalog.
member.py
    Contains the functionality for the creation of a member, which may then check out books. Information on books may be sourced from a catalog
    created using catalog.py, though attempts to request information that does not exist are not supported.

## Install pytest
```
python3 -m pip install -U pytest
```

## Run
Use `-B` to not create cache directories 
```bash
python3 -B -m app
```

## Tests
```bash
python3 -m pytest
```
