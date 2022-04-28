\set ON_ERROR_STOP 1
begin;

-- Drop existing Schema with cascades
drop schema if exists staging cascade;
drop schema if exists production cascade;
drop schema if exists public cascade;


create schema public;
GRANT ALL ON SCHEMA public TO postgres;
GRANT ALL ON SCHEMA public TO public;


set datestyle = 'dmy';

-- Create Schemas
create schema staging;
create schema production;

create extension if not exists fuzzystrmatch;

-- Mpesa Statement Tables

create table if not exists staging.mpesa_statement (
            receipt_no text not null,
            completion_time text not null,
            initiation_time text not null,
            details text not null,
            transaction_status text not null,
            paid_in double precision,
            withdrawn double precision,
            balance double precision,
            balance_confirmed bool,
            reason_type text,
            other_party_info text,
            linked_transaction_id text,
            ac_no text
        );

create table if not exists production.mpesa_statement (
            receipt_no text not null,
            completion_time timestamp not null,
            initiation_time timestamp not null,
            details text not null,
            transaction_status text not null,
            paid_in double precision,
            withdrawn double precision,
            balance double precision,
            balance_confirmed bool,
            reason_type text,
            other_party_info text,
            linked_transaction_id text,
            ac_no text
        );
-- Collection Details 
create table staging.collection_details (
	receipt_no text not null,
	receipt_date text null,
	patient_name text null,
	payee text null,
	cash double precision null,
	cheque double precision null,
	card double precision,
	card_no text null,
	mpesa double precision null,
	e_transfer double precision null,
	transaction_no text null,
	adv_used double precision null,
	employee_name text null,
	unit_name text null
);

create table if not exists production.collection_details (
	receipt_no text not null,
	receipt_date timestamp null,
	patient_name text null,
	payee text null,
	cash double precision null,
	cheque double precision null,
	card double precision not null,
	card_no text null,
	mpesa double precision null,
	e_transfer double precision null,
	transaction_no varchar(30) null,
	adv_used double precision null,
	employee_name text null,
	unit_name text null
);

--- Bill Details Table Definitions

create table staging.bill_details (
	bill_date text null,
	bill_no text null,
	skypeid text null,
	uhid text null,
	visit_type text null,
	patient_name text null,
	payee text null,
	service_name text null,
	quantity double precision null,
	rate_per_unit double precision null,
	discount double precision null,
	gross double precision null,
	paid_amount double precision null,
	outstanding double precision null,
	service_doctor text null,
	department text null,
	consulting_doctor text null,
	referring_doctor text null,
	servicing_doctor text null,
	payment_mode text null,
	unit text
);

create table production.bill_details (
	bill_date timestamp null,
	bill_no text null,
	skypeid text null,
	uhid text null,
	visit text null,
	patient_name text null,
	payee text null,
	service_name text null,
	quantity int null,
	rate_per_unit double precision null,
	discount double precision null,
	gross double precision null,
	paid_amount double precision null,
	outstanding double precision null,
	service_doctor text null,
	department text null,
	consulting_doctor text null,
	referring_doctor text null,
	servicing_doctor text null,
	payment_mode text null
);


--- Lab Visits table definitions

create table if not exists staging.lab_visits (
	sample_number text,
	name text, 
	id_passport_no text,
	age double precision,
	age_unit text,
	gender text,
	phone_number text,
	sample_date text,
	result text,
	email_address text
);

create table production.lab_visits (
	sample_number text not null,
	name text null,
	id_passport_no text null,
	age int null,
	age_unit text null,
	gender text null,
	phone_number text null,
	sample_date timestamp null,
	result text null,
	email_address text null,
	constraint lab_visits_pkey primary key (sample_number)
);

--- Registered Patients table definitions
create table if not exists staging.registered_patients (
	uhid text,
	date text,
	patient_name text,
	age text,
	gender text,
	address text,
	contact_no text
);

create table if not exists production.registered_patients (
	uhid text null,
	"date" timestamp null,
	patient_name text null,
	age text null,
	gender text null,
	address text null,
	contact_no text null
);

--- Mtiba Statement table definitions

create table if not exists staging.mtiba_statement (
	transactionstateid int,
	transactiontypeid int,
	facilityzohold text,
	facilityname text,
	fullreferencenumber text,
	phone_number text,
	payer_name text,
	sender_name text,
	medical_program_name text,
	amount_for_display double precision,
	transaction_date text,
	payment_date text,
	transaction_type text
);

create table production.mtiba_statement (
	transactionstateid text null,
	transactiontypeid int null,
	facilityzohold text null,
	facilityname text null,
	fullreferencenumber text null,
	phonenumber text null,
	payername text null,
	sendername text null,
	medicalprogramname text null,
	amountfordisplay double precision null,
	transactiondate timestamp null,
	paymentdate timestamp null,
	transactiontype text null
);

--- Absa Bank table definitions

create table if not exists staging.absa (
	transaction_date text,
	value_date text,
	description text,
	user_reference_number text,
	cheque_number int,
	debit_amount double precision,
	credit_amount double precision,
	running_balance double precision
);


create table if not exists production.absa_statement (
	transaction_date timestamp,
	value_date timestamp,
	description text,
	user_reference_number text null,
	cheque_number int null,
	debit_amount double precision null,
	credit_amount double precision null,
	running_balance double precision null);


--- Pdq Breakdowns table definitions

create table if not exists staging.pdq_breakdowns (
	account_no int,
	location_no int,
	legal_name text,
	card_no text,
	txn_date text,
	processing_date text,
	payment_date text,
	terminal_id int,
	auth_id text,
	amount double precision,
	commission double precision,
	net_amount double precision,
	trxn_type text,
	currency text,
	pmnt_type text,
	trxn_source text,
	scheme text,
	commercial_name text,
	arn_reference text,
	retrieval_ref_no text,
	tip_amount double precision,
	card_present text
);

create table if not exists production.pdq_breakdowns (
	account_no int null,
	location_no int null,
	legal_name text null,
	card_no text null,
	txn_date timestamp null,
	processing_date timestamp null,
	payment_date timestamp null,
	terminal_id int4 null,
	auth_id text null,
	amount double precision null,
	commission double precision null,
	net_amount double precision null,
	trxn_type text null,
	currency text null,
	pmnt_type text null,
	trxn_source text null,
	scheme text null,
	commercial_name text null,
	arn_reference text null,
	retrieval_ref_no text null,
	tip_amount double precision null,
	card_present text null
);


-- Sidian Bank table definitions
create table if not exists staging.sidian_statement (
	date text,
	valuedate text,
	reference text,
	narration text,
	chequenumber int,
	debit double precision,
	credit double precision,
	balance double precision
);

create table if not exists production.sidian_statement (
	date timestamp,
	valuedate timestamp null,
	reference text null,
	narration text null,
	chequenumber int null,
	debit double precision null,
	credit double precision null,
	balance double precision
);

--- CFC Bank table definitions

create table if not exists staging.cfc_statement (
	date text,
	transaction text,
	value_date text,
	debit double precision,
	credit double precision,
	ledger_balance double precision,
	available_balance double precision
);

create table if not exists production.cfc_statement (
	date timestamp,
	transaction text,
	value_date timestamp,
	debit double precision,
	credit double precision,
	ledger_balance double precision,
	available_balance double precision
);

commit;