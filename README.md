# PanopticonERPBackend
The backend for the PanopticonERP System written in Rust

This Repo provides the backend infrastructure for the PanopticonERP System.

To Start:
1. Cd ~/Downloads/panopticonbackend

2. Create a .env file with the following parameters:
  SERVER_ADDR=127.0.0.1:8080
  PG.USER=postgres
  PG.PASSWORD=testing_password
  PG.HOST=127.0.0.1
  PG.PORT=5432
  PG.DBNAME= panopticonerp
  PG.POOL.MAX_SIZE=16
 
3. Then run:
``` Cargo run
