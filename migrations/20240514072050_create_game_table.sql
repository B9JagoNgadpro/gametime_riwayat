-- Up migration
CREATE TABLE games (
    id UUID NOT NULL,
    transaction_id UUID,
    seller_id UUID NOT NULL,
    buyer_id UUID NOT NULL,
    amount INTEGER NOT NULL,
    PRIMARY KEY (id, transaction_id),
    FOREIGN KEY (transaction_id) REFERENCES transactions(id)
)

-- Down migrations