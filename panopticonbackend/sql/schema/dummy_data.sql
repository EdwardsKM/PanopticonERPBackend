\set ON_ERROR_STOP 1
begin;
create extension if not exists postgres_fdw;
create schema foreigndata;

CREATE SERVER his_foreign_server
FOREIGN DATA WRAPPER postgres_fdw
OPTIONS (host '127.0.0.1', port '5432', dbname
'his');

CREATE USER MAPPING FOR postgres
SERVER his_foreign_server
OPTIONS (user 'postgres', password 'correct_horse');


CREATE FOREIGN TABLE foreigndata.foreign_collection_details (
receipt_no text NOT NULL,
	receipt_date timestamp NULL,
	patient_name text NULL,
	payee text NULL,
	cash numeric NULL,
	cheque numeric NULL,
	card numeric NOT NULL,
	card_no text NULL,
	mpesa numeric NULL,
	e_transfer numeric NULL,
	transaction_no varchar(30) NULL,
	adv_used text NULL,
	employee_name text NULL,
	unit_name text NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'collection_details');

CREATE FOREIGN TABLE foreigndata.foreign_bill_details (
bill_date timestamp NULL,
	bill_no text NULL,
	skypeid text NULL,
	uhid text NULL,
	visit text NULL,
	patient_name text NULL,
	payee text NULL,
	service_name text NULL,
	quantity int4 NULL,
	rate_per_unit numeric NULL,
	discount numeric NULL,
	gross numeric NULL,
	paid_amount numeric NULL,
	outstanding numeric NULL,
	service_doc text NULL,
	department text NULL,
	consulting_dr text NULL,
	referring_dr text NULL,
	servicing_dr text NULL,
	payment_mode text NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'bill_details');

CREATE FOREIGN TABLE foreigndata.foreign_lab_visits (
sample_number varchar(15) NOT NULL,
	"name" text NULL,
	id_passport_no text NULL,
	age int4 NULL,
	age_unit varchar(6) NULL,
	gender bpchar(1) NULL,
	phone_number varchar(20) NULL,
	sample_date timestamp NULL,
	"result" text NULL,
	email_address text NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'lab_visits');

CREATE FOREIGN TABLE foreigndata.foreign_mpesa (
receipt_no varchar(11) NOT NULL,
	completion_time timestamp NULL,
	initiation_time timestamp NULL,
	details text NULL,
	transaction_status text NULL,
	paid_in numeric NULL,
	withdrawn numeric NULL,
	balance numeric NULL,
	balance_confirmed varchar(5) NULL,
	reason_type text NULL,
	other_party_info text NULL,
	linked_transaction_id text NULL,
	ac_no text NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'mpesa');

CREATE FOREIGN TABLE foreigndata.foreign_mtiba (
transactionstateid text NULL,
	transactiontypeid int4 NULL,
	facilityzohold text NULL,
	facilityname text NULL,
	fullreferencenumber text NULL,
	phonenumber text NULL,
	payername text NULL,
	sendername text NULL,
	medicalprogramname text NULL,
	amountfordisplay numeric NULL,
	transactiondate timestamp NULL,
	paymentdate timestamp NULL,
	transactiontype text NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'mtiba');

CREATE FOREIGN TABLE foreigndata.foreign_pdq_breakdowns (
account_no int4 NULL,
	location_no int4 NULL,
	legal_name text NULL,
	card_no text NULL,
	txn_date timestamp NULL,
	processing_date timestamp NULL,
	payment_date timestamp NULL,
	terminal_id int4 NULL,
	auth_id varchar(6) NULL,
	amount numeric NULL,
	commission numeric NULL,
	net_amount numeric NULL,
	trxn_type text NULL,
	currency varchar(5) NULL,
	pmnt_type text NULL,
	trxn_source text NULL,
	scheme text NULL,
	commercial_name text NULL,
	arn_reference text NULL,
	retrieval_ref_no text NULL,
	tip_amount numeric NULL,
	card_present varchar NULL
)
SERVER his_foreign_server
OPTIONS (schema_name 'public', table_name
'pdq_breakdowns');


commit;