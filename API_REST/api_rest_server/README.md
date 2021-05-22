# Execution guide

> .env Notes:
````
1.  Use the .env file to change the conections to MongoDB

2.  The current conection is to "test" database and "test" collection
````

> Run and process:
````
1.	Open the Terminal o CMD

2.	In the current path of this README file, run the commands

3.	Run "cargo update" forn search the updates

4.	After, run "cargo build" and finally run with "cargo run"

5.	This will activate the server in the 4000 port
````

### REST request:
````
1.	Use Postman or another request progrmas

2.	Into a "get", "delete" or "post" request use the follow url and structure
````

> Insert Query
````
- Into a "post" request use the follow URL: http://localhost:4000/add and we need the json body to insert:

{
    "title":"One Hundred Years of Solitude",
    "author":"Gabriel García Márquez"
}

- The result it's a single json with the ObjectId of the last document created, like this:

{
    "$oid": "60a85f260033d0d5005e6f29"
}

````

> Update Query
````
- Into a "post" request use the follow URL: http://localhost:4000/update/<ID>, this URL needs the ObjectID of the document to modify, and we need the json body to update:

{
    "title":"One Hundred Years of Solitude",
    "author":"Gabriel García Márquez"
}

URL EXAMPLE: http://localhost:4000/update/60a85f260033d0d5005e6f29

- The result only returns the document number that has been updated, in this case, its only one: 1

````

> Find by Query
````
- Into a "get" request use the follow URL: http://localhost:4000/get-by/<NAME>, this URL needs the name of author of the title.

URL EXAMPLE: http://localhost:4000/get-by/Gabriel García Márquez

- The result it's a single json with the ObjectId of the last document created, like this:

[
    {
        "_id": {
            "$oid": "60a85f260033d0d5005e6f29"
        },
        "title": "One Hundred Years of Solitude Year Edition",
        "author": "Gabriel García Márquez"
    }
]
````

> Delete Query
````
- Into a "delete" request use the follow URL: http://localhost:4000/delete, and we need the json body to delete:

{
    "title":"One Hundred Years of Solitude",
    "author":"Gabriel García Márquez"
}

- The result only returns the document number that has been updated, in this case, its only one: 1

````

> Find Query
````
- Into a "get" request use the follow URL: http://localhost:4000/get-all

- The result it's a Json Array with all documents into the data base, like these:

[
    {
        "_id": {
            "$oid": "60817272007be823004f1053"
        },
        "title": "1984",
        "author": "George Orwell"
    },
    {
        "_id": {
            "$oid": "60817272007be823004f1054"
        },
        "title": "Animal Farm",
        "author": "George Orwell"
    },
    {
        "_id": {
            "$oid": "60817272007be823004f1055"
        },
        "title": "The Great Gatsby",
        "author": "F. Scott Fitzgerald"
    }
]

````