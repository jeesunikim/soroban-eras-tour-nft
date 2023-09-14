# soroban-nft

## Utilizing Following Soroban Concepts

### Contract storage

Contracts have an exclusive read and write access to their storage in the ledger. This allows contracts to safely control and manage user access to their data. For example, a token contract may ensure that only the administrator can mint more of the token by storing the administrator identity in its storage. Similarly, it can make sure that only an owner of the balance may transfer that balance.