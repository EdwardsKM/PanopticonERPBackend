\set ON_ERROR_STOP 1
begin;
   CREATE OR REPLACE FUNCTION strip_all_triggers() RETURNS text AS $$ DECLARE
        triggNameRecord RECORD;
    triggTableRecord RECORD;
BEGIN
    FOR triggNameRecord IN select distinct(trigger_name) from information_schema.triggers where trigger_schema = 'public' LOOP
        SELECT distinct(event_object_table) INTO triggTableRecord from information_schema.triggers where trigger_name = triggNameRecord.trigger_name;
        RAISE NOTICE 'Dropping trigger: % on table: %', triggNameRecord.trigger_name, triggTableRecord.event_object_table;
        EXECUTE 'DROP TRIGGER ' || triggNameRecord.trigger_name || ' ON ' || triggTableRecord.event_object_table || ';';
    END LOOP;

    RETURN 'done';
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- Call the strip triggers function
select strip_all_triggers();


---- Create New Triggers ------------


-- Triggers for the public schema to update materialized views and tables
-- Update production.mpesa_statement when staging.mpesa_statement is updated while casting the column types
create or replace function update_production_mpesa_statement() returns trigger as $update_production_mpesa_statement$
	begin
	insert
	into
	production.mpesa_statement
select 
	receipt_no,
	completion_time::timestamp,
	initiation_time::timestamp,
	details,
	transaction_status,
	paid_in::double precision,
	withdrawn::double precision,
	balance::double precision,
	balance_confirmed::boolean,
	reason_type,
	other_party_info,
	linked_transaction_id,
	ac_no
from
	public.foreign_mpesa b
where
	not exists (
	select
		*
	from
		production.mpesa_statement a
	where
		a.receipt_no = b.receipt_no);
	
	RETURN NULL;
end;

$update_production_mpesa_statement$ 
language plpgsql;

-- Create Trigger that watches foreigndata.foreign_mpesa for updates
create trigger update_mpesa_statement after
insert
	or
update
	on
	public.foreign_mpesa for each row execute function update_production_mpesa_statement();

-- Update production.collection_details when staging.collection_details is updated while casting the column types
create or replace function update_production_collection_details() returns trigger as $update_production_collection_details$
	begin
with moved_rows as (
delete
from
  staging.collection_details a
where
  not exists (
  select
    (receipt_no,
    receipt_date::timestamp,
    patient_name,
    payee,
    cash,
    cheque,
    card,
    card_no,
    mpesa,
    e_transfer,
    transaction_no,
    adv_used,
    employee_name,
    unit_name)
  from
    production.collection_details b
  where
    a.receipt_no = b.receipt_no)
returning *
)
insert
  into
  production.collection_details 
select
  (receipt_no,
  receipt_date::timestamp,
  patient_name,
  payee,
  cash,
  cheque,
  card,
  card_no,
  mpesa,
  e_transfer,
  transaction_no,
  adv_used,
  employee_name,
  unit_name)
from
  moved_rows;
end;

$update_production_collection_details$ language plpgsql;

-- Create Trigger that watches staging.collection_details for updates
create trigger update_collection_details after
insert
	or
update
	on
	staging.collection_details for each row execute function update_production_collection_details();

commit;