# Tron Foundational Modules

The **Tron Foundational Modules** Substreams contains a set of modules that allow you to easily retrieve basic information from the Tron blockchain.

## Modules

### index_transactions

This module creates a cache of transactions based on:
- The _contract type_ of every transactions.

You can use this module as a `blockFilter` to filter transactions based on the parameters specified above:

```yaml
  - name: my_module
    ...
    blockFilter:
      module: index_transactions
      query:
        string: (contract_type:contract_type1 || contract_type:contract_type2)
```
