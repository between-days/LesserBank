-- create some accounts
INSERT INTO accounts (id, customer_id, account_type, balance_cents, date_opened, account_status, name, available_balance_cents, account_number, bsb)
VALUES 
    (50, 5, 'savings', 2000000000, '2023-07-22 11:49:53.81936+00', 'active', 'Very Expensive Car', 1500000000, '123456789', '123456'),
    (51, 5, 'savings', 242342344, '2018-08-22 9:49:53.81936+00', 'active', 'Expensive House', 1503434, '274857367', '123456'),
    (52, 5, 'transaction', 20000000, '2015-07-22 12:49:53.81936+00', 'active', 'Everyday', 1500000000, '938573843', '123456'),
    (53, 5, 'term_deposit', 200340, '2023-07-22 11:49:53.81936+00', 'active', 'Fork to eat soup with', 1500000000, '847312938', '123456'),
    (54, 5, 'savings', 12312323453, '2023-07-22 11:48:53.81936+00', 'active', 'Lot of plants', 1500000000, '384756375', '123456'),
    (55, 5, 'term_deposit', 24124321, '2023-06-22 11:49:53.81936+00', 'active', 'Cheese of goat is expensive', 24104321, '273847563', '123456'),
    (56, 5, 'transaction', 2000000000, '2022-07-22 11:49:53.81936+00', 'active', 'BabloCoin day trading', 150000000000, '374884756', '123456');

-- create some transactions
INSERT INTO transactions (id, customer_id, transaction_type, from_us, amount_cents, from_number, from_bsb, from_name, to_number, to_bsb, available_balance_cents, date_start, date_end, transaction_status)
VALUES
    (50, 5, 'internal', true, 23423423, '123456789', '123456', 'Very Expensive Car', '274857367', '123456', 23423423, '2023-07-22 11:48:53.81936+00', '2023-07-22 11:50:53.81936+00', 'success')