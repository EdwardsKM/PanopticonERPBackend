\set ON_ERROR_STOP 1
begin;
   --- The Next part defines the materialized views that will be used for caching --------

create materialized view public.mpesa_reconciliations as
	WITH mpesa_bill_list AS (
    SELECT
        receipt_no AS billing_number,
        employee_name,
        receipt_date,
        patient_name,
        mpesa,
        UPPER(transaction_no) "transaction_code"
    FROM
        public.foreign_collection_details
    WHERE
        mpesa > 0
        AND (unit_name = 'MEDITEST DIAGNOSTIC SERVICES LTD.')),
          mpesa_statement AS (
    SELECT
        receipt_no,
        completion_time,
        initiation_time,
        details,
        transaction_status,
        paid_in,
        withdrawn,
        balance,
        balance_confirmed,
        reason_type,
        other_party_info,
        linked_transaction_id,
        ac_no
    FROM
        public.foreign_mpesa),
    matched_transactions AS (
    SELECT
        billing_number,
        employee_name,
        receipt_date,
        patient_name,
        mpesa_bill_list.mpesa,
        (sum(mpesa_bill_list.mpesa) over (partition by mpesa_statement.receipt_no)) as total_billed,
        mpesa_statement.paid_in,
        mpesa_bill_list.transaction_code,
        mpesa_statement.receipt_no,
        levenshtein(mpesa_bill_list.transaction_code,
        mpesa_statement.receipt_no) AS distance,
        completion_time
    FROM
        mpesa_bill_list,
        mpesa_statement
    WHERE
        levenshtein(mpesa_bill_list.transaction_code,
        mpesa_statement.receipt_no)< 3
    ORDER BY
        receipt_no),
    unmatched_transactions AS (
    SELECT
        *
    FROM
        mpesa_bill_list
    WHERE
        NOT EXISTS(
        SELECT
            billing_number
        FROM
            matched_transactions
        WHERE
            mpesa_bill_list.billing_number = matched_transactions.billing_number)),
         preliminary_reconciliations AS (
    SELECT
        *
    FROM
        matched_transactions
    UNION ALL
    SELECT
        billing_number,
        employee_name,
        receipt_date,
        patient_name,
        mpesa,
        NULL as total_billed,
        NULL AS paid_in,
        transaction_code,
        NULL AS receipt_no,
        NULL AS distance,
        NULL AS completion_time
    FROM
        unmatched_transactions)
    SELECT
        DISTINCT *
    FROM
        preliminary_reconciliations
    ORDER BY
        receipt_no
;
commit;

CREATE OR REPLACE FUNCTION tg_refresh_mpesa_reconciliations()
RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY public.mpesa_reconciliations;
    RETURN NULL;
END;
$$;

-- Then, in any table that involves changes on the view, you do:
-- Collection details can be updated with new details
CREATE TRIGGER tg_refresh_mpesa_reconciliations AFTER INSERT OR UPDATE OR DELETE
ON public.foreign_collection_details
FOR EACH STATEMENT EXECUTE PROCEDURE tg_refresh_mpesa_reconciliations();

-- The Mpesa Statement can also be updated

CREATE TRIGGER tg_refresh_mpesa_reconciliations AFTER INSERT OR UPDATE OR DELETE
ON public.foreign_mpesa
FOR EACH STATEMENT EXECUTE PROCEDURE tg_refresh_mpesa_reconciliations();
