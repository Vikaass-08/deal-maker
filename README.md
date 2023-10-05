
# Deal Maker
Transforming Signatures, Elevating Transactions

![undraw_nakamoto_-2-iv6 2-2](https://github.com/Shreyans13/deal-maker/assets/50544190/17677670-9ac5-4dd6-aac5-9de1d0d93071)


Introducing **Deal Maker**, the leading backend solution meticulously designed to streamline the document signing process for both lenders and borrowers. Our platform caters to two distinct user roles: BORROWER and LENDER, each equipped with specialized functionalities to enhance their document management experience.

For **LENDERS**, Deal Maker provides a comprehensive suite of tools, allowing them to seamlessly create and modify documents before the crucial signing stage. Lenders can effortlessly accept document signing requests initiated by borrowers, ensuring a smooth and efficient collaboration throughout the process.

On the other side, **BORROWERS** benefit from a user-friendly interface that enables them to apply for document signing with any preferred lender. Once a lending partner accepts the request, the signing process kicks into gear, providing a secure and convenient environment for all parties involved.

**Deal Maker** is not just a platform; it's a premier solution dedicated to optimizing and elevating the document signing experience for lenders and borrowers alike. Experience a new standard of efficiency and collaboration in the world of document management with Deal Maker.


## Postgres DB images through Docker

- Download docker desktop
- create docker-compose.yml 
- docker-compose -f docker-compose.yml up -d   (TO create docker image and run in the background)
- docker stop container_name

    <img width="700" height="500" alt="Database Structure" src="https://github.com/Vikaass-08/deal-maker/assets/59832889/2ca46beb-22bc-4e2a-b1ef-66da2c2a08dc">

## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

- `DATABASE_URL` : `postgres://postgres:password@localhost/db_name`
- `JWT_SECRET_USER`
- `HASH_SECRET`
- `JWT_SECRET_LENDER`


## Commands to run 

- diesel setup
- diesel migration table_name
- diesel migration run
- cargo build
- cargo run

- diesel migration revert (used to redo the applied migration)


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



## JWT Auth

- import all the dependencies
- create 2 Tokens (Lender, Borrower)
- validator_lender, validator_user (Used to verify token)
- need to pass token during request (Bearer Token)



## API Reference

#### Create a borrower account

```http
  POST create-borrower
```

#### Create a Lender account

```http
  POST /create-lender
```

#### Login as borrower

```http
  POST /login-borrower
```

#### login as Lender

```http
  POST /login-lender
```


#### Create document (Auth: Lender)

```http
  POST /lender/auth/create-document
```

#### Create/Get the document request status (Auth: Borrower)

```http
  POST /borrower/auth/get-or-create-req
```

#### Get connection req by borrower (Auth: Lender)

```http
  GET /lender/auth/get-all-request
```

#### Update the connection req by borrower (Auth: Lender)

```http
  PUT /lender/auth/update-request
```


#### Create a Deal Req (Auth: Borrower)

```http
  PUT /borrower/auth/create-deal
```


#### Accept or reject the deal (Auth: Lender)

```http
  PUT /lender/auth/update-deal
```
