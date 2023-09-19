
# Deal Maker

- It is a premier backend solution dedicated to optimizing the document signing process for lenders and borrowers. 
- There are 2 kinds of user (BORROWER, LENDER). 
- Lender can create and make changes in the documents (Before Signing), accept document signing request from a borrower.
- Borrowers can apply for document Signing for any lenders, once accepted then the signing process takes place.


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`DATABASE_URL`


## Postgres DB images through Docker

- Download docker desktop
- create docker-compose.yml 
- docker-compose -f docker-compose.yml up -d   (TO create docker image and run in the background)
- docker stop container_name

    <img width="700" height="500" alt="Screenshot 2023-09-20 at 12 25 40 AM" src="https://github.com/Vikaass-08/deal-maker/assets/59832889/53593624-1e4f-47b3-a17d-fffc489ec8d0">

## Commands to run 

- diesel setup
- diesel migration table_name
- diesel migration run
- cargo build
- cargo run



## API Reference

#### Creat a User

```http
  POST borrower/create
```

#### Get document

```http
  GET /document/get
```


#### Create/Save document

```http
  POST /document/create
```

## Codebase Structure

- diesel setup (create a diesel.toml file)
    - This file contains migration directory and schema location (you can change the location of dir).
- diesel migration generate table_name (This we create a table migration)
- Inside src/database
    - schema.rs (It we be auto generated while generating migration)
    - models.rs (models types in native rust)
    - lib.rs (creating a pool, to connect to db and run queries)
    - queries file (seperate files for seperate table queries)
- Inside src/routes (handlers based on endpoints and request)
- main.rs (handlers mapping based on url and start of the project)
