pub mod models {

    use crate::errors::errors::MyError;
    use chrono::NaiveDateTime;
    use deadpool_postgres::Client;

    use futures_util::pin_mut;
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper::FromTokioPostgresRow;
    use tokio_pg_mapper_derive::PostgresMapper;
    use tokio_postgres::binary_copy::BinaryCopyInWriter;
    use tokio_postgres::types::{ToSql, Type};

    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "staging.mpesa_statement")]
    pub struct MpesaStatementInsert {
        pub receipt_no: String,
        pub completion_time: String,
        pub initiation_time: String,
        pub details: String,
        pub transaction_status: String,
        pub paid_in: Option<f64>,
        pub withdrawn: Option<f64>,
        pub balance: Option<f64>,
        pub balance_confirmed: bool,
        pub reason_type: String,
        pub other_party_info: String,
        pub linked_transaction_id: Option<String>,
        pub ac_no: String,
    }
    impl MpesaStatementInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<MpesaStatementInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_mpesa_statement.sql");

            //let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());

            // Create a prepared statement through transaction.prepare method
            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::BOOL,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::writee(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn writee(
            writer: BinaryCopyInWriter,
            data: &Vec<MpesaStatementInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.receipt_no);
                row.push(&m.completion_time);
                row.push(&m.initiation_time);
                row.push(&m.details);
                row.push(&m.transaction_status);
                row.push(&m.paid_in);
                row.push(&m.withdrawn);
                row.push(&m.balance);
                row.push(&m.balance_confirmed);
                row.push(&m.reason_type);
                row.push(&m.other_party_info);
                row.push(&m.linked_transaction_id);
                row.push(&m.ac_no);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.mpesa_reconciliations")]
    pub struct ReconciledMpesa {
        billing_number: String,
        cashier: String,
        receipt_date: chrono::NaiveDateTime,
        patient_name: String,
        mpesa: f64,
        transaction_code: String,
        comments: String,
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.mpesa_statement")]
    pub struct MpesaStatement {
        pub receipt_no: String,
        pub completion_time: NaiveDateTime,
        pub initiation_time: NaiveDateTime,
        pub details: String,
        pub transaction_status: String,
        pub paid_in: Option<f64>,
        pub withdrawn: Option<f64>,
        pub balance: f64,
        pub balance_confirmed: bool,
        pub reason_type: String,
        pub other_party_info: String,
        pub linked_transaction_id: Option<String>,
        pub ac_no: Option<String>,
    }

    impl MpesaStatement {
        pub async fn get_statement(client: &Client) -> Result<Vec<MpesaStatement>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_mpesa_statement.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| MpesaStatement::from_row_ref(&row).unwrap())
                .collect::<Vec<MpesaStatement>>();
            Ok(res)
        }
        pub async fn get_reconciled_statement(
            client: &Client,
            date: String,
        ) -> Result<Vec<ReconciledMpesa>, MyError> {
            let date = date;

            let stmt = include_str!("../sql/user_actions/get_reconciled_mpesa_statement.sql");

            let res = client
                .query(stmt, &[&date])
                .await?
                .into_iter()
                .map(|row| ReconciledMpesa::from_row_ref(&row).unwrap())
                .collect::<Vec<ReconciledMpesa>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.collection_details")]
    pub struct CollectionDetailsInsert {
        pub receipt_no: Option<String>,
        pub receipt_date: String,
        pub patient_name: Option<String>,
        pub payee: Option<String>,
        pub cash: Option<f64>,
        pub cheque: Option<f64>,
        pub card: Option<f64>,
        pub card_no: Option<f64>,
        pub mpesa: Option<f64>,
        pub e_transfer: Option<f64>,
        pub transaction_no: Option<String>,
        pub adv_used: Option<f64>,
        pub employee_name: Option<String>,
        pub unit_name: Option<String>,
    }
    impl CollectionDetailsInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<CollectionDetailsInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_collection_details.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::TEXT,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<CollectionDetailsInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.receipt_no);
                row.push(&m.receipt_date);
                row.push(&m.patient_name);
                row.push(&m.payee);
                row.push(&m.cash);
                row.push(&m.cheque);
                row.push(&m.card);
                row.push(&m.card_no);
                row.push(&m.mpesa);
                row.push(&m.e_transfer);
                row.push(&m.transaction_no);
                row.push(&m.adv_used);
                row.push(&m.employee_name);
                row.push(&m.unit_name);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.collection_details")]
    pub struct CollectionDetails {
        pub receipt_no: Option<String>,
        pub receipt_date: chrono::NaiveDateTime,
        pub patient_name: Option<String>,
        pub payee: Option<String>,
        pub cash: Option<f64>,
        pub cheque: Option<f64>,
        pub card: Option<f64>,
        pub card_no: Option<String>,
        pub mpesa: Option<f64>,
        pub e_transfer: Option<f64>,
        pub transaction_no: Option<String>,
        pub adv_used: Option<f64>,
        pub employee_name: Option<String>,
        pub unit_name: Option<String>,
    }
    impl CollectionDetails {
        pub async fn get_statement(client: &Client) -> Result<Vec<CollectionDetails>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_collection_details.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| CollectionDetails::from_row_ref(&row).unwrap())
                .collect::<Vec<CollectionDetails>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.bill_details")]
    pub struct BillDetailsInsert {
        pub bill_date: String,
        pub bill_no: Option<String>,
        pub skypeid: Option<String>,
        pub uhid: Option<String>,
        pub visit_type: Option<String>,
        pub patient_name: Option<String>,
        pub payee: Option<String>,
        pub service_name: Option<String>,
        pub quantity: Option<f64>,
        pub rate_per_unit: Option<f64>,
        pub discount: Option<f64>,
        pub gross: Option<f64>,
        pub paid_amount: Option<f64>,
        pub outstanding: Option<f64>,
        pub service_doc: Option<String>,
        pub department: Option<String>,
        pub consulting_dr: Option<String>,
        pub referring_dr: Option<String>,
        pub servicing_dr: Option<String>,
        pub payment_mode: Option<String>,
        pub unit: String,
    }
    impl BillDetailsInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<BillDetailsInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_bill_details.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<BillDetailsInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.bill_date);
                row.push(&m.bill_no);
                row.push(&m.skypeid);
                row.push(&m.uhid);
                row.push(&m.visit_type);
                row.push(&m.patient_name);
                row.push(&m.payee);
                row.push(&m.service_name);
                row.push(&m.quantity);
                row.push(&m.rate_per_unit);
                row.push(&m.discount);
                row.push(&m.gross);
                row.push(&m.paid_amount);
                row.push(&m.outstanding);
                row.push(&m.service_doc);
                row.push(&m.department);
                row.push(&m.consulting_dr);
                row.push(&m.referring_dr);
                row.push(&m.servicing_dr);
                row.push(&m.payment_mode);
                row.push(&m.unit);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.bill_details")]
    pub struct BillDetails {
        pub bill_date: NaiveDateTime,
        pub bill_no: Option<String>,
        pub skypeid: Option<String>,
        pub uhid: Option<String>,
        pub visit: Option<String>,
        pub patient_name: Option<String>,
        pub payee: Option<String>,
        pub service_name: Option<String>,
        pub quantity: Option<f64>,
        pub rate_per_unit: Option<f64>,
        pub discount: Option<f64>,
        pub gross: Option<f64>,
        pub paid_amount: Option<f64>,
        pub outstanding: Option<f64>,
        pub service_doc: Option<String>,
        pub department: Option<String>,
        pub consulting_dr: Option<String>,
        pub referring_dr: Option<String>,
        pub servicing_dr: Option<String>,
        pub payment_mode: Option<String>,
    }
    impl BillDetails {
        // Get all Bill Details
        pub async fn get_statement(client: &Client) -> Result<Vec<BillDetails>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_bill_details.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| BillDetails::from_row_ref(&row).unwrap())
                .collect::<Vec<BillDetails>>();
            Ok(res)
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.lab_visits")]
    pub struct LabVisitsInsert {
        pub sample_number: String,
        pub name: String,
        pub id_passport_no: Option<String>,
        pub age: f64,
        pub age_unit: String,
        pub gender: String,
        pub phone_number: Option<String>,
        pub sample_date: String,
        pub result: String,
        pub email_address: Option<String>,
    }

    impl LabVisitsInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<LabVisitsInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_lab_visits.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<LabVisitsInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.sample_number);
                row.push(&m.name);
                row.push(&m.id_passport_no);
                row.push(&m.age);
                row.push(&m.age_unit);
                row.push(&m.gender);
                row.push(&m.phone_number);
                row.push(&m.sample_date);
                row.push(&m.result);
                row.push(&m.email_address);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.lab_visits")]
    pub struct LabVisits {
        pub sample_number: String,
        pub name: String,
        pub id_passport_no: String,
        pub age: i32,
        pub age_unit: String,
        pub gender: String,
        pub phone_number: Option<String>,
        pub sample_date: chrono::NaiveDateTime,
        pub result: String,
        pub email_address: Option<String>,
    }

    impl LabVisits {
        /// Get all lab visits
        pub async fn get_statement(client: &Client) -> Result<Vec<LabVisits>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_lab_visits.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| LabVisits::from_row_ref(&row).unwrap())
                .collect::<Vec<LabVisits>>();
            Ok(res)
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "registered_patients")]
    pub struct RegisteredPatients {
        pub uhid: String,
        pub date: chrono::NaiveDateTime,
        pub patient_name: String,
        pub age: String,
        pub gender: String,
        pub address: Option<String>,
        pub contact_no: Option<String>,
    }
    impl RegisteredPatients {
        pub async fn get_statement(client: &Client) -> Result<Vec<RegisteredPatients>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_registered_patients.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| RegisteredPatients::from_row_ref(&row).unwrap())
                .collect::<Vec<RegisteredPatients>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.mtiba_statement")]
    pub struct MtibaStatementInsert {
        transactionstateid: Option<i32>,
        transactiontypeid: Option<i32>,
        facilityzohold: String,
        facilityname: String,
        fullreferencenumber: String,
        phonenumber: String,
        payername: String,
        sendername: String,
        medicalprogramname: String,
        amountfordisplay: Option<f64>,
        transactiondate: chrono::NaiveDateTime,
        paymentdate: chrono::NaiveDateTime,
        transactiontype: String,
    }
    impl MtibaStatementInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<MtibaStatementInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_mtiba_statement.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::FLOAT8,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<MtibaStatementInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.transactionstateid);
                row.push(&m.transactiontypeid);
                row.push(&m.facilityzohold);
                row.push(&m.facilityname);
                row.push(&m.fullreferencenumber);
                row.push(&m.phonenumber);
                row.push(&m.payername);
                row.push(&m.sendername);
                row.push(&m.medicalprogramname);
                row.push(&m.amountfordisplay);
                row.push(&m.transactiondate);
                row.push(&m.paymentdate);
                row.push(&m.transactiontype);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.mtiba_statement")]
    pub struct MtibaStatement {
        transactionstateid: Option<i32>,
        transactiontypeid: Option<i32>,
        facilityzohold: String,
        facilityname: String,
        fullreferencenumber: String,
        phonenumber: String,
        payername: String,
        sendername: String,
        medicalprogramname: String,
        amountfordisplay: f64,
        transactiondate: chrono::NaiveDateTime,
        paymentdate: chrono::NaiveDateTime,
        transactiontype: String,
    }
    impl MtibaStatement {
        pub async fn get_statement(client: &Client) -> Result<Vec<MtibaStatement>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_mtiba_statement.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| MtibaStatement::from_row_ref(&row).unwrap())
                .collect::<Vec<MtibaStatement>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.absa_statement")]
    pub struct ABSAInsert {
        pub transaction_date: String,
        pub value_date: String,
        pub description: String,
        pub user_reference_number: Option<String>,
        pub cheque_number: Option<f64>,
        pub debit_amount: Option<f64>,
        pub credit_amount: Option<f64>,
        pub running_balance: Option<f64>,
    }
    impl ABSAInsert {
        pub async fn update(client: &mut Client, data: Vec<ABSAInsert>) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_absa_statement.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of ABSA bank transactions
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<ABSAInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.transaction_date);
                row.push(&m.value_date);
                row.push(&m.description);
                row.push(&m.user_reference_number);
                row.push(&m.cheque_number);
                row.push(&m.debit_amount);
                row.push(&m.credit_amount);
                row.push(&m.running_balance);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }

    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.absa_statement")]
    pub struct ABSA {
        pub transaction_date: chrono::NaiveDateTime,
        pub value_date: chrono::NaiveDateTime,
        pub description: String,
        pub user_reference_number: Option<String>,
        pub cheque_number: Option<i32>,
        pub debit_amount: Option<f64>,
        pub credit_amount: Option<f64>,
        pub running_balance: Option<f64>,
    }
    impl ABSA {
        pub async fn get_statement(client: &Client) -> Result<Vec<ABSA>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_absa_statement.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| ABSA::from_row_ref(&row).unwrap())
                .collect::<Vec<ABSA>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.pdq_statement")]
    pub struct PdqBreakdownInsert {
        pub account_no: Option<f64>,
        pub location_no: Option<f64>,
        pub legal_name: Option<String>,
        pub card_no: String,
        pub txn_date: Option<String>,
        pub processing_date: Option<String>,
        pub payment_date: Option<String>,
        pub terminal_id: Option<f64>,
        pub auth_id: Option<String>,
        pub amount: Option<f64>,
        pub commission: Option<f64>,
        pub net_amount: Option<f64>,
        pub trxn_type: Option<String>,
        pub currency: Option<String>,
        pub pmnt_type: Option<String>,
        pub trxn_source: Option<String>,
        pub scheme: Option<String>,
        pub commercial_name: Option<String>,
        pub arn_reference: Option<String>,
        pub retrieval_ref_no: Option<String>,
        pub tip_amount: Option<f64>,
        pub card_present: Option<String>,
    }
    impl PdqBreakdownInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<PdqBreakdownInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_pdq_breakdowns.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::FLOAT8,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::TEXT,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<PdqBreakdownInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.account_no);
                row.push(&m.location_no);
                row.push(&m.legal_name);
                row.push(&m.card_no);
                row.push(&m.txn_date);
                row.push(&m.processing_date);
                row.push(&m.payment_date);
                row.push(&m.terminal_id);
                row.push(&m.auth_id);
                row.push(&m.amount);
                row.push(&m.commission);
                row.push(&m.net_amount);
                row.push(&m.trxn_type);
                row.push(&m.currency);
                row.push(&m.pmnt_type);
                row.push(&m.trxn_source);
                row.push(&m.scheme);
                row.push(&m.commercial_name);
                row.push(&m.arn_reference);
                row.push(&m.retrieval_ref_no);
                row.push(&m.tip_amount);
                row.push(&m.card_present);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.pdq_statement")]
    pub struct PdqBreakdown {
        pub account_no: Option<f64>,
        pub location_no: Option<f64>,
        pub legal_name: Option<String>,
        pub card_no: String,
        pub txn_date: Option<chrono::NaiveDateTime>,
        pub processing_date: Option<chrono::NaiveDateTime>,
        pub payment_date: Option<chrono::NaiveDateTime>,
        pub terminal_id: Option<f64>,
        pub auth_id: Option<String>,
        pub amount: Option<f64>,
        pub commission: Option<f64>,
        pub net_amount: Option<f64>,
        pub trxn_type: Option<String>,
        pub currency: Option<String>,
        pub pmnt_type: Option<String>,
        pub trxn_source: Option<String>,
        pub scheme: Option<String>,
        pub commercial_name: Option<String>,
        pub arn_reference: Option<String>,
        pubretrieval_ref_no: Option<String>,
        pub tip_amount: Option<f64>,
        pub card_present: Option<String>,
    }
    impl PdqBreakdown {
        pub async fn get_statement(client: &Client) -> Result<Vec<PdqBreakdown>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_pdq_breakdowns.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| PdqBreakdown::from_row_ref(&row).unwrap())
                .collect::<Vec<PdqBreakdown>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.sidian_statement")]
    pub struct SidianInsert {
        pub date: String,
        pub valuedate: Option<String>,
        pub reference: Option<String>,
        pub narration: Option<String>,
        pub chequenumber: Option<f64>,
        pub debit: Option<f64>,
        pub credit: Option<f64>,
        pub balance: f64,
    }
    impl SidianInsert {
        pub async fn update(
            client: &mut Client,
            data: Vec<SidianInsert>,
        ) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_sidian_statement.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<SidianInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.date);
                row.push(&m.valuedate);
                row.push(&m.reference);
                row.push(&m.narration);
                row.push(&m.chequenumber);
                row.push(&m.debit);
                row.push(&m.credit);
                row.push(&m.balance);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.sidian_statement")]
    pub struct Sidian {
        pub date: chrono::NaiveDateTime,
        pub valuedate: Option<chrono::NaiveDate>,
        pub reference: Option<String>,
        pub narration: Option<String>,
        pub chequenumber: Option<i32>,
        pub debit: Option<f64>,
        pub credit: Option<f64>,
        pub balance: f64,
    }
    impl Sidian {
        pub async fn get_statement(client: &Client) -> Result<Vec<Sidian>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_sidian_statement.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| Sidian::from_row_ref(&row).unwrap())
                .collect::<Vec<Sidian>>();
            Ok(res)
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "staging.cfc_statement")]
    pub struct CfcInsert {
        pub date: chrono::NaiveDateTime,
        pub transaction: String,
        pub value_date: chrono::NaiveDateTime,
        pub debit: Option<f64>,
        pub credit: Option<f64>,
        pub ledger_balance: Option<f64>,
        pub available_balance: Option<f64>,
    }
    impl CfcInsert {
        pub async fn update(client: &mut Client, data: Vec<CfcInsert>) -> Result<usize, MyError> {
            // Create the transaction to represent db tx
            let tx = client.transaction().await?;

            let stmt = include_str!("../sql/user_actions/update_cfc_statement.sql");

            let statement = tx.prepare(stmt).await?;

            // Create a sink that will take the vec of MpesaPayments
            let sink = tx.copy_in(&statement).await?;

            let types = &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
                Type::FLOAT8,
            ];

            let writer = BinaryCopyInWriter::new(sink, types);

            let num_written = Self::insertintodb(writer, &data).await?;

            tx.commit().await?;

            Ok(num_written)
        }

        pub async fn insertintodb(
            writer: BinaryCopyInWriter,
            data: &Vec<CfcInsert>,
        ) -> Result<usize, MyError> {
            pin_mut!(writer);

            let mut row: Vec<&'_ (dyn ToSql + Sync)> = Vec::new();
            for m in data {
                row.clear();
                row.push(&m.date);
                row.push(&m.transaction);
                row.push(&m.value_date);
                row.push(&m.debit);
                row.push(&m.credit);
                row.push(&m.ledger_balance);
                row.push(&m.available_balance);

                writer.as_mut().write(&row).await?;
            }

            writer.finish().await?;

            Ok(data.len())
        }
    }
    #[derive(Deserialize, PostgresMapper, Serialize)]
    #[pg_mapper(table = "production.cfc_statement")]
    pub struct Cfc {
        pub date: chrono::NaiveDateTime,
        pub transaction: String,
        pub value_date: chrono::NaiveDateTime,
        pub debit: Option<f64>,
        pub credit: Option<f64>,
        pub ledger_balance: Option<f64>,
        pub available_balance: Option<f64>,
    }
    impl Cfc {
        pub async fn get_statement(client: &Client) -> Result<Vec<Cfc>, MyError> {
            let stmt = include_str!("../sql/user_actions/get_cfc_statement.sql");

            let res = client
                .query(stmt, &[])
                .await?
                .into_iter()
                .map(|row| Cfc::from_row_ref(&row).unwrap())
                .collect::<Vec<Cfc>>();
            Ok(res)
        }
    }
    // Used to register a new user
    #[derive(Deserialize, PostgresMapper, Serialize, Debug)]
    #[pg_mapper(table = "users")] // singular 'user' is a keyword..
    pub struct User {
        pub first_name: String,
        pub middle_name: String,
        pub last_name: String,
        pub email: String,
        pub phone_number: String,
        pub username: String,
        pub password: String,
    }
    // TODO where to add password length and complexity requirements.
    // TODO check provided username against db usernames before registering new user
    // TODO check email does not match current users' email
    //
    // Define the various scopes available for the app's users
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    enum Scope {
        Guest,
        User,
        Admin,
    }

    impl Default for Scope {
        fn default() -> Self {
            Scope::Guest
        }
    }
}
