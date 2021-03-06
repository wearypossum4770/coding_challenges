-- Find the number of records where the customer number is null
select  count(*) from customers left join orders on customers.customerNumber = orders.customerNumber where orders.customerNumber is null;

-- Find the exact records which are orphaned.
select  customers.customerNumber, orders.orderNumber from customers left join orders on customers.customerNumber = orders.customerNumber where orders.customerNumber is null;