# TRON Foundational Modules

The **TRON Foundational Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the TRON blockchain.

## Modules

### map_transactions

This module retrieves all the **NOT** failed transactions, without any more filtering.

### filtered_transactions

This module uses the `index_transactions` cache to match the filtered transactions based on the parameters passed as input to the module.

You can directly _use_ this module to retrieve filtered transactions:

```yaml
modules:
    - name: my_module
      use: tron_common:filtered_transactions

params:
    my_module: source_account:account1
```

### index_transactions

This module creates a cache of transactions based on:
- The _contract type_ of every transactions.
- The _contract address_ of every transactions.
- The _owner address_ of every transactions that support it.
- The _to address_ of every transactions that suppoirt it

You can use this module as a `blockFilter` to filter transactions based on the parameters specified above. Here's an example to filter all the USDT (`TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t`) transfers from `TMAP4Dnyh2og7bzW6HxZfuZqRSTHsVDKRT`:

```yaml
  - name: my_module
    ...
    blockFilter:
      module: index_transactions
      query:
        string: (contract_type:TriggerSmartContract && contract_address:TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t && from:TMAP4Dnyh2og7bzW6HxZfuZqRSTHsVDKRT)
```
