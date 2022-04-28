pub mod handlers {
    use crate::errors::errors::MyError;
    use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
    use deadpool_postgres::{Client, Pool};

    // Implement a Health Handler. Very useful when containerizing the app
    //Checks to see if the service is alive. Also tests the database connection. Returns Ok('Healthy') if both are running.
    #[get("/health")]
    pub async fn health_check(db_pool: web::Data<Pool>) -> Result<HttpResponse, MyError> {
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let my_str = "hello world";

        // Now we can execute a simple statement that just returns its parameter.
        let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

        // And then check that we got back the same string we sent over.
        let value: &str = rows[0].get(0);

        // Build the HttpResponse depending on value
        if value == my_str {
            Ok(HttpResponse::Ok().body("Healthy Connection"))
        } else {
            Ok(HttpResponse::NotFound().body("Can't establish a connection to the database"))
        }
    }

    #[get("/")]
    async fn index(_req: HttpRequest) -> impl Responder {
        "Welcome to your Tls secured homepage!"
    }

    #[get("/dashboard")]
    async fn dashboard(_req: HttpRequest) -> impl Responder {
        "Welcome to your Dashboard!"
    }
    pub mod mpesa_handlers {

        use crate::{
            errors::errors::MyError,
            models::models::{MpesaStatement, MpesaStatementInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};
        use uuid::Uuid;

        /// Define the handlers for Mpesa
        /// Scope is /statements/mpesa
        /// fn get_mpesa_statement returns the full mpesa statement.
        #[get("/statements/mpesa")]
        pub async fn get_mpesa_statement(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
            let request_id = Uuid::new_v4();
            tracing::info!("request_id {} - Getting the mpesa statement", request_id);

            tracing::info!("request_id {} - Connecting to database", request_id);

            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
            tracing::info!("request_id {} - Connected to database", request_id);
            tracing::info!(
                "request_id {} - Querying database for latest Mpesa statement",
                request_id
            );

            match MpesaStatement::get_statement(&client).await {
                Ok(new_statement) => {
                    tracing::info!("Database query completed succesfully");
                    Ok(HttpResponse::Ok().json(new_statement))
                }
                Err(e) => {
                    tracing::error!("Failed to execute query: {:?}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
        }

        /// Query the reconciled mpesa statement using a specific date
        #[get("/reconciliations/mpesa/{date}")]
        pub async fn reconcile_mpesa_statement(
            db_pool: web::Data<Pool>,
            date: web::Path<String>,
        ) -> Result<HttpResponse, MyError> {
            let date = date.into_inner();

            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let reconciled_statement =
                MpesaStatement::get_reconciled_statement(&client, date).await?;

            Ok(HttpResponse::Ok().json(reconciled_statement))
        }

        // Post to the Mpesa Statement. This handler takes a String of Json POSTed by the user
        #[post("/statements/mpesa/update")]
        pub async fn update_mpesa_statement(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<MpesaStatementInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = MpesaStatementInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }
    pub mod collection_details_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{CollectionDetails, CollectionDetailsInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/collectiondetails")]
        pub async fn get_collection_details(
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = CollectionDetails::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        // Post to staging.collectiondetails. This handler takes a String of Json POSTed by the user
        #[post("/statements/collectiondetails/update")]
        pub async fn update_collection_details(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<CollectionDetailsInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = CollectionDetailsInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod bill_details_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{BillDetails, BillDetailsInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/billdetails")]
        pub async fn get_bill_details(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = BillDetails::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        // Post to staging.billdetails. This handler takes a String of Json POSTed by the user
        #[post("/statements/billdetails/update")]
        pub async fn update_bill_details(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<BillDetailsInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = BillDetailsInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod lab_visits_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{LabVisits, LabVisitsInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        //use chrono::NaiveDateTime;
        use deadpool_postgres::{Client, Pool};
        // Post to staging.lab_visits. This handler takes a String of Json POSTed by the user
        #[post("/statements/labvisits/update")]
        pub async fn update_lab_visits(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<LabVisitsInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = LabVisitsInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }

        #[get("/statements/labvisits")]
        pub async fn get_lab_visits(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = LabVisits::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
    }

    pub mod registered_patients_handlers {
        use crate::{errors::errors::MyError, models::models::RegisteredPatients};
        use actix_web::{get, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        /// Query the registered patients table
        #[get("/statements/registeredpatients")]
        pub async fn get_registered_patients(
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = RegisteredPatients::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
    }
    pub mod mtiba_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{MtibaStatement, MtibaStatementInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/mtiba")]
        pub async fn get_mtiba_statement(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = MtibaStatement::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        // Post to staging.mtiba. This handler takes a String of Json POSTed by the user
        #[post("/statements/mtiba/update")]
        pub async fn update_mtiba_statement(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<MtibaStatementInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = MtibaStatementInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod sidian_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{Sidian, SidianInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        /// Get the entire Sidian Bank Statement
        #[get("/statements/sidian")]
        pub async fn get_sidian_bank_statement(
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = Sidian::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        #[post("/statements/sidian/update")]
        pub async fn update_sidian_statement(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<SidianInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = SidianInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod absa_bank_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{ABSAInsert, ABSA},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/absa")]
        pub async fn get_absa_bank_statement(
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = ABSA::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }

        #[post("/statements/absa/update")]
        pub async fn update_absa_statement(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<ABSAInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = ABSAInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod pdq_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{PdqBreakdown, PdqBreakdownInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/pdq")]
        pub async fn get_pdq_statement(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = PdqBreakdown::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        #[post("/statements/pdq/update")]
        pub async fn update_pdq_breakdowns(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of MpesaPayment
            let datas: Vec<PdqBreakdownInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = PdqBreakdownInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }

    pub mod cfc_handlers {
        use crate::{
            errors::errors::MyError,
            models::models::{Cfc, CfcInsert},
        };
        use actix_web::{get, post, web, Error, HttpResponse};
        use deadpool_postgres::{Client, Pool};

        #[get("/statements/cfc")]
        pub async fn get_cfc_bank_statement(
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            let new_statement = Cfc::get_statement(&client).await?;

            Ok(HttpResponse::Ok().json(new_statement))
        }
        #[post("/statements/cfc/update")]
        pub async fn update_cfc_statement(
            payment: String,
            db_pool: web::Data<Pool>,
        ) -> Result<HttpResponse, Error> {
            // Deserialize into a vec of CFC transactions
            let datas: Vec<CfcInsert> = serde_json::from_str(&payment)?;

            // Create the database connection
            let mut client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

            // Call the db insertion handler. Insert the vec of Mpesa Payments. Result from DB should be number of rows inserted
            let insertion = CfcInsert::update(&mut client, datas).await?;

            Ok(HttpResponse::Ok().json(insertion))
        }
    }
}
