# SQL

Simple relational database written in **Rust**.

<p align='center'>
    <img src="./sql.png" title="SQL" alt="SQL"/>
</p>

## Featrues

1. CRUD on tables
2. CRUD on databases

## Commands

### Database

1. Create new database

   ```sql
   CREATE DATABASE <DB_NAME>;
   ```

2. Drop database
   ```sql
   DROP DATABASE <DB_NAME>;
   ```
3. Switch database
   ```sql
   USE DATABASE <DB_NAME>;
   ```

### Tables

1. Create table
   ```sql
   CREATE TABLE <TABLE_NAME> (
    column1 datatype,
    column2 datatype,
    column3 datatype,
   ....
   );
   ```
2. Drop table
   ```sql
   DROP TABLE <TABLE_NAME>;
   ```
3. Truncate table

   ```sql
   TRUNCATE TABLE <TABLE_NAME>;
   ```

4. Alter table
   - ADD Column
     ```sql
     ALTER TABLE table_name
     ADD <COL_NAME> datatype;
     ```
   - DROP COLUMN
     ```sql
     ALTER TABLE table_name
     DROP COLUMN column_name;
     ```
   - ALTER/MODIFY COLUMN
     ```sql
     ALTER TABLE table_name
     ALTER COLUMN column_name datatype;
     ```

### Query

1. Select

```sql
SELECT column1, column2, ...
FROM table_name;
```

```sql
SELECT * FROM table_name;
```
